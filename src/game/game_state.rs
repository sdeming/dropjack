// Game states
use crate::game::Game;
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
        card_atlas: &Option<Texture2D>,
        particle_system: &mut ParticleSystem,
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
        card_atlas: &Option<Texture2D>,
        _particle_system: &mut ParticleSystem,
    ) {
        // Draw a sophisticated gradient background
        DrawingHelpers::draw_gradient_background(d);

        // Draw decorative card elements in the background
        DrawingHelpers::draw_background_cards(d, card_atlas);

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
        card_atlas: &Option<Texture2D>,
        particle_system: &mut ParticleSystem,
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
        card_atlas: &Option<Texture2D>,
        particle_system: &mut ParticleSystem,
        show_dynamic_cards: bool,
    ) {
        Self::draw_game_board(d, game, card_atlas, show_dynamic_cards);
        Self::draw_info_panel(d, game, has_controller, title_font, font, card_atlas);

        // Draw particle effects on top of everything
        particle_system.draw(d);
    }

    fn draw_game_board(d: &mut RaylibDrawHandle, game: &Game, card_atlas: &Option<Texture2D>, show_dynamic_cards: bool) {
        // Draw board outline
        let board_width = game.board.width * game.board.cell_size;
        let board_height = game.board.height * game.board.cell_size;
        d.draw_rectangle(
            BOARD_OFFSET_X - 2,
            BOARD_OFFSET_Y - 2,
            board_width + 4,
            board_height + 4,
            Color::WHITE,
        );

        // Draw board background
        d.draw_rectangle(
            BOARD_OFFSET_X,
            BOARD_OFFSET_Y,
            board_width,
            board_height,
            Color::BLACK,
        );

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
        card_atlas: &Option<Texture2D>,
    ) {
        // Draw panel background
        d.draw_rectangle(
            INFO_PANEL_X,
            BOARD_OFFSET_Y,
            INFO_PANEL_WIDTH,
            SCREEN_HEIGHT - 2 * BOARD_OFFSET_Y,
            Color::DARKBLUE,
        );

        // Draw panel title using title font - larger
        d.draw_text_ex(
            title_font,
            "DropJack",
            Vector2::new((INFO_PANEL_X + 30) as f32, (BOARD_OFFSET_Y + 30) as f32),
            40.0,
            1.5,
            Color::WHITE,
        );

        // Draw difficulty - larger
        let difficulty_text = format!("Difficulty: {}", game.difficulty.to_string());
        d.draw_text_ex(
            font,
            &difficulty_text,
            Vector2::new((INFO_PANEL_X + 30) as f32, (BOARD_OFFSET_Y + 90) as f32),
            24.0,
            1.0,
            Color::WHITE,
        );

        // Draw score - larger
        let score_text = format!("Score: {}", game.score);
        d.draw_text_ex(
            font,
            &score_text,
            Vector2::new((INFO_PANEL_X + 30) as f32, (BOARD_OFFSET_Y + 130) as f32),
            30.0,
            1.25,
            Color::WHITE,
        );

        // Draw next card preview heading using title font - larger
        d.draw_text_ex(
            title_font,
            "Next Card:",
            Vector2::new((INFO_PANEL_X + 30) as f32, (BOARD_OFFSET_Y + 190) as f32),
            28.0,
            1.0,
            Color::WHITE,
        );

        if let Some(card) = game.next_card {
            DrawingHelpers::draw_card_inline(
                d,
                card_atlas,
                card,
                INFO_PANEL_X + 60,
                BOARD_OFFSET_Y + 230,
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
        card_atlas: &Option<Texture2D>,
        particle_system: &mut ParticleSystem,
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
        card_atlas: &Option<Texture2D>,
        particle_system: &mut ParticleSystem,
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
        card_atlas: &Option<Texture2D>,
        particle_system: &mut ParticleSystem,
    ) {
        // Draw the start screen background first
        StartScreen.render(
            d,
            game,
            has_controller,
            title_font,
            font,
            card_atlas,
            particle_system,
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
