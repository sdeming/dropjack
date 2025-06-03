// Game states
use crate::game::Game;
use crate::ui::animated_background::AnimatedBackground;
use crate::ui::drawing::{
    DrawingHelpers, BOARD_OFFSET_X, BOARD_OFFSET_Y, INFO_PANEL_WIDTH, INFO_PANEL_X, SCREEN_HEIGHT,
    SCREEN_WIDTH,
};
use crate::ui::particle_system::ParticleSystem;
use raylib::prelude::*;

// Trait that all game states must implement
pub trait GameState {
    fn should_update(&self) -> bool {
        false // Default: most states don't update
    }

    fn state_name(&self) -> &'static str;

    fn render(
        &self,
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        particle_system: &mut ParticleSystem,
        animated_background: &mut AnimatedBackground,
    );
}

// Individual game state implementations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartScreen;

impl GameState for StartScreen {
    fn state_name(&self) -> &'static str {
        "StartScreen"
    }

    fn render(
        &self,
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        _particle_system: &mut ParticleSystem,
        animated_background: &mut AnimatedBackground,
    ) {
        // Draw a sophisticated gradient background
        DrawingHelpers::draw_gradient_background(d);

        // Draw animated background cards instead of static ones
        animated_background.draw(d, card_atlas);

        // Main title with shadow effect
        DrawingHelpers::draw_title_with_shadow(d, title_font);

        // Subtitle with elegant styling
        DrawingHelpers::draw_subtitle(d, font);

        // Central game panel with rounded corners and shadow
        DrawingHelpers::draw_main_panel(d);

        // Difficulty selection with nice styling
        DrawingHelpers::draw_difficulty_selector(d, title_font, font, game, has_controller);

        // High scores in a nice panel
        DrawingHelpers::draw_high_scores_panel(d, title_font, font, game);

        // Call-to-action button with glow effect
        DrawingHelpers::draw_start_button(d, title_font, has_controller);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Playing;

impl GameState for Playing {
    fn should_update(&self) -> bool {
        true // Playing state should update
    }

    fn state_name(&self) -> &'static str {
        "Playing"
    }

    fn render(
        &self,
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        particle_system: &mut ParticleSystem,
        _animated_background: &mut AnimatedBackground,
    ) {
        Self::draw_game_view(d, game, has_controller, title_font, font, card_atlas, particle_system, true);
    }
}

impl Playing {
    /// Draws the complete game view (board + info panel + particles)
    /// This is used by both Playing and Paused states to avoid duplication
    /// 
    /// # Parameters
    /// * `show_dynamic_cards` - If true, shows falling cards and current card. If false, only shows static board state (for pause screen)
    pub fn draw_game_view(
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        particle_system: &mut ParticleSystem,
        show_dynamic_cards: bool,
    ) {
        Self::draw_game_board(d, game, card_atlas, show_dynamic_cards);
        Self::draw_info_panel(d, game, has_controller, title_font, font, card_atlas);

        // Draw particle effects on top of everything
        particle_system.draw(d);
    }

    fn draw_game_board(d: &mut RaylibDrawHandle, game: &Game, card_atlas: &Texture2D, show_dynamic_cards: bool) {
        // Draw the beautiful game board background with green felt and grid
        DrawingHelpers::draw_game_board_background(d, game.board.width, game.board.height, game.board.cell_size);

        // Only draw static cards on the board when in playing mode
        // In pause mode, hide them so players can't analyze board patterns
        if show_dynamic_cards {
            // Draw cards on the board
            for y in 0..game.board.height {
                for x in 0..game.board.width {
                    if let Some(card) = game.board.grid[y as usize][x as usize] {
                        // Check if this position has a falling card animation
                        let has_falling =
                            game.board.falling_cards.iter().any(|falling| {
                                falling.x == x && falling.to_y == y && falling.is_animating
                            });

                        // Only draw static cards if there's no falling animation
                        if !has_falling {
                            DrawingHelpers::draw_card_inline(
                                d,
                                card_atlas,
                                card,
                                BOARD_OFFSET_X + x * game.board.cell_size,
                                BOARD_OFFSET_Y + y * game.board.cell_size,
                                game.board.cell_size,
                            );
                        }
                    }
                }
            }

            // Draw falling cards with smooth animation
            for falling_card in &game.board.falling_cards {
                if falling_card.is_animating {
                    DrawingHelpers::draw_card_inline(
                        d,
                        card_atlas,
                        falling_card.card,
                        BOARD_OFFSET_X + falling_card.x * game.board.cell_size,
                        BOARD_OFFSET_Y + falling_card.visual_y as i32,
                        game.board.cell_size,
                    );
                }
            }
        }

        // Always draw current falling card (even in pause mode, as requested)
        if let Some(ref playing_card) = game.current_card {
            DrawingHelpers::draw_card_inline(
                d,
                card_atlas,
                playing_card.card,
                BOARD_OFFSET_X + playing_card.visual_position.x as i32,
                BOARD_OFFSET_Y + playing_card.visual_position.y as i32,
                game.board.cell_size,
            );
        }
    }

    fn draw_info_panel(
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
    ) {
        // Enhanced panel background with sophisticated styling and depth
        let panel_height = SCREEN_HEIGHT - 2 * BOARD_OFFSET_Y;
        let panel_center_y = BOARD_OFFSET_Y + panel_height / 2;
        
        // Outermost shadow for dramatic depth
        d.draw_rectangle(
            INFO_PANEL_X - 8,
            BOARD_OFFSET_Y - 8,
            INFO_PANEL_WIDTH + 16,
            panel_height + 16,
            Color::new(0, 0, 0, 120),
        );
        
        // Multiple frame layers for rich depth
        // Outer dark wood frame matching the board
        d.draw_rectangle(
            INFO_PANEL_X - 6,
            BOARD_OFFSET_Y - 6,
            INFO_PANEL_WIDTH + 12,
            panel_height + 12,
            Color::new(80, 40, 20, 255),
        );
        
        // Middle wood frame with grain effect
        d.draw_rectangle(
            INFO_PANEL_X - 4,
            BOARD_OFFSET_Y - 4,
            INFO_PANEL_WIDTH + 8,
            panel_height + 8,
            Color::new(139, 69, 19, 255),
        );
        
        // Add wood grain lines for consistency with board frame
        for i in 0..6 {
            let grain_offset = i * 2;
            d.draw_line(
                INFO_PANEL_X - 4 + grain_offset,
                BOARD_OFFSET_Y - 4,
                INFO_PANEL_X - 4 + grain_offset,
                BOARD_OFFSET_Y + panel_height + 4,
                Color::new(110, 55, 15, 80),
            );
        }
        
        // Inner decorative border
        d.draw_rectangle(
            INFO_PANEL_X - 2,
            BOARD_OFFSET_Y - 2,
            INFO_PANEL_WIDTH + 4,
            panel_height + 4,
            Color::new(210, 180, 140, 255),
        );
        
        // Create sophisticated radial gradient background for the panel - OPTIMIZED
        let panel_center_x = INFO_PANEL_X + INFO_PANEL_WIDTH / 2;
        let max_distance = ((INFO_PANEL_WIDTH * INFO_PANEL_WIDTH + panel_height * panel_height) as f32).sqrt() / 2.0;
        
        // Use efficient overlapping rectangles for smooth gradient - NO GAPS
        let gradient_steps = 20; // Reduced for performance but still smooth
        let step_width = (INFO_PANEL_WIDTH as f32 / gradient_steps as f32).ceil() as i32;
        let step_height = (panel_height as f32 / gradient_steps as f32).ceil() as i32;
        
        for y in 0..gradient_steps {
            for x in 0..gradient_steps {
                let rect_x = INFO_PANEL_X + x * step_width;
                let rect_y = BOARD_OFFSET_Y + y * step_height;
                
                // Make rectangles overlap slightly to eliminate gaps
                let rect_width = if x == gradient_steps - 1 { 
                    INFO_PANEL_WIDTH - x * step_width + 2 
                } else { 
                    step_width + 2 
                };
                let rect_height = if y == gradient_steps - 1 { 
                    panel_height - y * step_height + 2 
                } else { 
                    step_height + 2 
                };
                
                // Calculate center of this rectangle for distance calculation
                let center_x_offset = (rect_x + rect_width / 2) - panel_center_x;
                let center_y_offset = (rect_y + rect_height / 2) - (BOARD_OFFSET_Y + panel_height / 2);
                let distance = ((center_x_offset * center_x_offset + center_y_offset * center_y_offset) as f32).sqrt();
                let distance_ratio = (distance / max_distance).min(1.0);
                
                // Create sophisticated color transitions
                let light_factor = 1.0 - (distance_ratio * distance_ratio * 0.5);
                let x_factor = x as f32 / gradient_steps as f32;
                let y_factor = y as f32 / gradient_steps as f32;
                
                // Rich blue gradient with subtle variations
                let base_r = 25.0 + y_factor * 20.0;
                let base_g = 25.0 + x_factor * 25.0 + y_factor * 15.0;
                let base_b = 80.0 + x_factor * 30.0 + y_factor * 25.0;
                
                let r = (base_r * light_factor) as u8;
                let g = (base_g * light_factor) as u8;
                let b = (base_b * light_factor + 10.0) as u8;
                
                let color = Color::new(r, g, b, 255);
                d.draw_rectangle(rect_x, rect_y, rect_width, rect_height, color);
            }
        }
        
        // Add subtle fabric-like texture to match the board
        for i in 0..80 {
            let x = INFO_PANEL_X + (i * 61) % INFO_PANEL_WIDTH;
            let y = BOARD_OFFSET_Y + (i * 97) % panel_height;
            
            // Distance from center affects texture visibility
            let dx = x - panel_center_x;
            let dy = y - panel_center_y;
            let distance_from_center = ((dx * dx + dy * dy) as f32).sqrt();
            let distance_ratio = (distance_from_center / max_distance).min(1.0);
            
            // Texture is more visible in lit areas
            let base_alpha = 20.0 * (1.0 - distance_ratio * 0.6);
            let alpha = ((i * 23) % 12 + base_alpha as i32) as u8;
            
            let size = 0.2 + ((i * 7) % 4) as f32 * 0.1;
            d.draw_circle(x, y, size, Color::new(255, 255, 255, alpha));
        }

        // Enhanced panel title with multiple shadow layers and glow effect
        let title_text = "DropJack";
        let title_x = INFO_PANEL_X + 30;
        let title_y = BOARD_OFFSET_Y + 30;
        
        // Outer glow effect
        for glow_layer in 1..=4 {
            let glow_offset = glow_layer as i32;
            let glow_alpha = 40 / glow_layer;
            d.draw_text_ex(
                title_font,
                title_text,
                Vector2::new((title_x + glow_offset) as f32, (title_y + glow_offset) as f32),
                40.0,
                1.5,
                Color::new(255, 215, 0, glow_alpha as u8),
            );
        }
        
        // Deep shadow
        d.draw_text_ex(
            title_font,
            title_text,
            Vector2::new((title_x + 3) as f32, (title_y + 3) as f32),
            40.0,
            1.5,
            Color::new(0, 0, 0, 180),
        );
        
        // Medium shadow
        d.draw_text_ex(
            title_font,
            title_text,
            Vector2::new((title_x + 2) as f32, (title_y + 2) as f32),
            40.0,
            1.5,
            Color::new(0, 0, 0, 120),
        );
        
        // Close shadow
        d.draw_text_ex(
            title_font,
            title_text,
            Vector2::new((title_x + 1) as f32, (title_y + 1) as f32),
            40.0,
            1.5,
            Color::new(0, 0, 0, 80),
        );
        
        // Main title with gradient effect
        d.draw_text_ex(
            title_font,
            title_text,
            Vector2::new(title_x as f32, title_y as f32),
            40.0,
            1.5,
            Color::new(255, 215, 0, 255), // Gold text
        );
        
        // Top highlight for 3D effect
        d.draw_text_ex(
            title_font,
            title_text,
            Vector2::new(title_x as f32, (title_y - 1) as f32),
            40.0,
            1.5,
            Color::new(255, 255, 200, 100),
        );

        // Enhanced difficulty display with styling
        let difficulty_text = format!("Difficulty: {}", game.difficulty.to_string());
        let diff_x = INFO_PANEL_X + 30;
        let diff_y = BOARD_OFFSET_Y + 90;
        
        // Multiple shadow layers
        d.draw_text_ex(
            font,
            &difficulty_text,
            Vector2::new((diff_x + 2) as f32, (diff_y + 2) as f32),
            24.0,
            1.0,
            Color::new(0, 0, 0, 150),
        );
        d.draw_text_ex(
            font,
            &difficulty_text,
            Vector2::new((diff_x + 1) as f32, (diff_y + 1) as f32),
            24.0,
            1.0,
            Color::new(0, 0, 0, 100),
        );
        d.draw_text_ex(
            font,
            &difficulty_text,
            Vector2::new(diff_x as f32, diff_y as f32),
            24.0,
            1.0,
            Color::new(255, 255, 255, 255),
        );

        // Enhanced score display with glow effect
        let score_text = format!("Score: {}", game.score);
        let score_x = INFO_PANEL_X + 30;
        let score_y = BOARD_OFFSET_Y + 130;
        
        // Glow effect for the score
        for glow in 1..=3 {
            let glow_alpha = 60 / glow;
            d.draw_text_ex(
                font,
                &score_text,
                Vector2::new((score_x + glow) as f32, (score_y + glow) as f32),
                30.0,
                1.25,
                Color::new(255, 215, 0, glow_alpha as u8),
            );
        }
        
        // Main score shadow
        d.draw_text_ex(
            font,
            &score_text,
            Vector2::new((score_x + 2) as f32, (score_y + 2) as f32),
            30.0,
            1.25,
            Color::new(0, 0, 0, 150),
        );
        
        // Main score text
        d.draw_text_ex(
            font,
            &score_text,
            Vector2::new(score_x as f32, score_y as f32),
            30.0,
            1.25,
            Color::new(255, 215, 0, 255),
        );

        // Enhanced next card preview with sophisticated frame
        let next_card_text = "Next Card:";
        let next_x = INFO_PANEL_X + 30;
        let next_y = BOARD_OFFSET_Y + 190;
        
        // Shadow and text
        d.draw_text_ex(
            title_font,
            next_card_text,
            Vector2::new((next_x + 2) as f32, (next_y + 2) as f32),
            28.0,
            1.0,
            Color::new(0, 0, 0, 120),
        );
        d.draw_text_ex(
            title_font,
            next_card_text,
            Vector2::new(next_x as f32, next_y as f32),
            28.0,
            1.0,
            Color::new(255, 255, 255, 255),
        );

        if let Some(card) = game.next_card {
            // Enhanced decorative frame around the next card with lighting effects
            let card_x = INFO_PANEL_X + 60;
            let card_y = BOARD_OFFSET_Y + 230;
            let frame_size = game.board.cell_size + 16;
            
            // Outer shadow
            d.draw_rectangle(
                card_x - 10,
                card_y - 8,
                frame_size + 4,
                frame_size + 4,
                Color::new(0, 0, 0, 100),
            );
            
            // Multiple frame layers for depth
            d.draw_rectangle(
                card_x - 8,
                card_y - 8,
                frame_size,
                frame_size,
                Color::new(80, 40, 20, 255),
            );
            d.draw_rectangle(
                card_x - 6,
                card_y - 6,
                frame_size - 4,
                frame_size - 4,
                Color::new(139, 69, 19, 255),
            );
            d.draw_rectangle(
                card_x - 4,
                card_y - 4,
                frame_size - 8,
                frame_size - 8,
                Color::new(210, 180, 140, 255),
            );
            
            // Inner highlight
            d.draw_rectangle(
                card_x - 2,
                card_y - 2,
                frame_size - 12,
                frame_size - 12,
                Color::new(255, 255, 200, 60),
            );
            
            DrawingHelpers::draw_card_inline(
                d,
                card_atlas,
                card,
                card_x,
                card_y,
                game.board.cell_size,
            );
        }

        // Draw conditional controls based on controller availability
        DrawingHelpers::draw_controls(
            d,
            title_font,
            font,
            INFO_PANEL_X,
            BOARD_OFFSET_Y,
            has_controller,
        );
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paused;

impl GameState for Paused {
    fn state_name(&self) -> &'static str {
        "Paused"
    }

    fn render(
        &self,
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        particle_system: &mut ParticleSystem,
        _animated_background: &mut AnimatedBackground,
    ) {
        // Draw the complete game view as background (reusing Playing state's rendering)
        Playing::draw_game_view(d, game, has_controller, title_font, font, card_atlas, particle_system, false);

        // Draw semi-transparent pause overlay
        d.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0, 0, 0, 200));

        // Draw paused text using title font
        d.draw_text_ex(
            title_font,
            "GAME PAUSED",
            Vector2::new(460.0, 250.0),
            60.0,
            2.5,
            Color::WHITE,
        );

        // Draw current score (not final score)
        let score_text = format!("Current Score: {}", game.score);
        d.draw_text_ex(
            font,
            &score_text,
            Vector2::new(500.0, 330.0),
            36.0,
            1.5,
            Color::WHITE,
        );

        // Draw conditional pause instructions based on controller availability
        DrawingHelpers::draw_pause_instructions(d, font, has_controller);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameOver;

impl GameState for GameOver {
    fn state_name(&self) -> &'static str {
        "GameOver"
    }

    fn render(
        &self,
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        particle_system: &mut ParticleSystem,
        _animated_background: &mut AnimatedBackground,
    ) {
        // Draw the complete game view as background (reusing Playing state's rendering)
        Playing::draw_game_view(d, game, has_controller, title_font, font, card_atlas, particle_system, false);

        // Draw semi-transparent overlay
        d.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0, 0, 0, 200));

        // Draw game over text using title font - centered for new resolution
        d.draw_text_ex(
            title_font,
            "GAME OVER",
            Vector2::new(490.0, 250.0),
            60.0,
            2.5,
            Color::WHITE,
        );

        // Draw final score
        let score_text = format!("Final Score: {}", game.score);
        d.draw_text_ex(
            font,
            &score_text,
            Vector2::new(530.0, 330.0),
            36.0,
            1.5,
            Color::WHITE,
        );

        // Draw initials input heading using title font
        d.draw_text_ex(
            title_font,
            "Enter your initials:",
            Vector2::new(520.0, 390.0),
            32.0,
            1.25,
            Color::WHITE,
        );

        // Draw initials box
        let box_width = 200; // Increased width
        let box_height = 60; // Increased height
        let box_x = SCREEN_WIDTH / 2 - box_width / 2;
        let box_y = 440; // Moved down

        d.draw_rectangle(box_x, box_y, box_width, box_height, Color::DARKGRAY);
        d.draw_rectangle_lines(box_x, box_y, box_width, box_height, Color::WHITE);

        // Draw entered initials
        let initials_text = if game.player_initials.is_empty() {
            "___".to_string()
        } else {
            format!("{:_<3}", game.player_initials)
        };

        d.draw_text_ex(
            font,
            &initials_text,
            Vector2::new((box_x + 65) as f32, (box_y + 15) as f32),
            36.0,
            1.5,
            Color::WHITE,
        );

        // Draw conditional instructions based on controller availability
        DrawingHelpers::draw_game_over_instructions(d, font, has_controller);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuitConfirm;

impl GameState for QuitConfirm {
    fn state_name(&self) -> &'static str {
        "QuitConfirm"
    }

    fn render(
        &self,
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        particle_system: &mut ParticleSystem,
        animated_background: &mut AnimatedBackground,
    ) {
        // Draw the start screen background first (which includes animated background)
        StartScreen.render(
            d,
            game,
            has_controller,
            title_font,
            font,
            card_atlas,
            particle_system,
            animated_background,
        );

        // Draw semi-transparent overlay
        d.draw_rectangle(0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, Color::new(0, 0, 0, 200));

        // Draw quit confirmation dialog
        d.draw_text_ex(
            title_font,
            "QUIT GAME?",
            Vector2::new(480.0, 300.0),
            60.0,
            2.5,
            Color::WHITE,
        );

        // Draw conditional quit confirmation based on controller availability
        DrawingHelpers::draw_quit_confirmation(d, font, has_controller);
    }
}
