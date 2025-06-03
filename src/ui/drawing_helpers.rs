use crate::cards::Card;
use crate::game::Game;
use crate::ui::drawing::{AtlasCardRenderer, DrawingHelpers, SCREEN_HEIGHT, SCREEN_WIDTH, BOARD_OFFSET_X, BOARD_OFFSET_Y};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::prelude::{Font, Texture2D};

impl DrawingHelpers {
    pub fn draw_card_inline(
        d: &mut RaylibDrawHandle,
        atlas: &Texture2D,
        card: Card,
        card_x: i32,
        card_y: i32,
        size: i32,
    ) {
        // Enhanced multi-layer shadow system for dramatic depth
        // Outer shadow (most diffuse)
        d.draw_rectangle(card_x + 6, card_y + 6, size, size, Color::new(0, 0, 0, 40));
        // Middle shadow
        d.draw_rectangle(card_x + 4, card_y + 4, size, size, Color::new(0, 0, 0, 60));
        // Inner shadow (sharpest)
        d.draw_rectangle(card_x + 2, card_y + 2, size, size, Color::new(0, 0, 0, 80));
        
        // Enhanced border system with beveled edges
        // Outer dark frame
        d.draw_rectangle(card_x - 3, card_y - 3, size + 6, size + 6, Color::new(101, 50, 14, 255));
        // Middle frame with lighter brown
        d.draw_rectangle(card_x - 2, card_y - 2, size + 4, size + 4, Color::new(139, 69, 19, 255));
        // Inner highlight frame
        d.draw_rectangle(card_x - 1, card_y - 1, size + 2, size + 2, Color::new(222, 184, 135, 255));

        // Use atlas card renderer
        AtlasCardRenderer::draw_card_from_card(d, atlas, card, card_x, card_y, size);
        
        // Enhanced lighting effects
        // Top highlight (simulating overhead light)
        d.draw_rectangle(card_x, card_y, size, 3, Color::new(255, 255, 255, 80));
        // Left edge highlight 
        d.draw_rectangle(card_x, card_y, 2, size, Color::new(255, 255, 255, 50));
        // Subtle inner glow
        d.draw_rectangle_lines(card_x + 1, card_y + 1, size - 2, size - 2, Color::new(255, 255, 255, 30));
    }

    pub fn draw_gradient_background(d: &mut RaylibDrawHandle) {
        // Create a sophisticated atmospheric background with subtle variations
        let gradient_steps = 40;
        let step_height = SCREEN_HEIGHT / gradient_steps;

        for i in 0..gradient_steps {
            let ratio = i as f32 / gradient_steps as f32;
            // More sophisticated color transitions with subtle color shifts
            let r = (8.0 + ratio * 12.0 + (ratio * std::f32::consts::PI).sin() * 2.0) as u8;
            let g = (15.0 + ratio * 15.0 + (ratio * 2.1).sin() * 3.0) as u8;
            let b = (25.0 + ratio * 20.0 + (ratio * 1.7).sin() * 4.0) as u8;

            let color = Color::new(r, g, b, 255);
            d.draw_rectangle(0, i * step_height, SCREEN_WIDTH, step_height + 1, color);
        }

        // Add subtle atmospheric particles for ambiance
        for i in 0..25 {
            let x = (i * 127) % SCREEN_WIDTH;
            let y = (i * 211) % SCREEN_HEIGHT;
            let alpha = ((i * 17) % 35 + 10) as u8;
            let size = 0.3 + ((i * 13) % 7) as f32 * 0.1;
            d.draw_circle(x, y, size, Color::new(255, 255, 255, alpha));
        }
    }

    pub fn draw_title_with_shadow(d: &mut RaylibDrawHandle, title_font: &Font) {
        let title = "DropJack";
        let title_x = 600.0 - 160.0; // Centered position for larger title
        let title_y = 60.0; // Moved down slightly
        let title_size = 120.0; // Increased from 80.0

        // Draw shadow layers for depth
        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(title_x + 6.0, title_y + 6.0),
            title_size,
            2.0,
            Color::new(0, 0, 0, 150),
        );
        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(title_x + 3.0, title_y + 3.0),
            title_size,
            2.0,
            Color::new(0, 0, 0, 100),
        );
        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(title_x + 1.5, title_y + 1.5),
            title_size,
            2.0,
            Color::new(0, 0, 0, 50),
        );

        // Main title with gradient effect (simulate by drawing slightly offset)
        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(title_x, title_y),
            title_size,
            2.0,
            Color::new(255, 215, 0, 255),
        ); // Gold
        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(title_x, title_y - 1.0),
            title_size,
            2.0,
            Color::new(255, 255, 255, 200),
        ); // White highlight
    }

    pub fn draw_subtitle(d: &mut RaylibDrawHandle, font: &Font) {
        let subtitle = "A Strategic Card-Falling Puzzle";
        let subtitle_x = 600.0 - 140.0;
        let subtitle_y = 200.0;
        let subtitle_size = 32.0;

        d.draw_text_ex(
            font,
            subtitle,
            Vector2::new(subtitle_x + 2.0, subtitle_y + 2.0),
            subtitle_size,
            1.0,
            Color::new(0, 0, 0, 80),
        );
        d.draw_text_ex(
            font,
            subtitle,
            Vector2::new(subtitle_x, subtitle_y),
            subtitle_size,
            1.0,
            Color::new(200, 200, 255, 255),
        );
    }

    pub fn draw_main_panel(d: &mut RaylibDrawHandle) {
        // Main content panel - much larger to fill space better
        let panel_x = 290; // Adjusted for larger panel (1280/2 - 700/2 = 290)
        let panel_y = 260; // Moved down below larger title/subtitle
        let panel_width = 700; // Increased from 400
        let panel_height = 380; // Increased from 280

        // Draw panel shadow
        d.draw_rectangle(
            panel_x + 4,
            panel_y + 4,
            panel_width,
            panel_height,
            Color::new(0, 0, 0, 50),
        );

        // Draw the main panel with a semi-transparent background
        d.draw_rectangle(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            Color::new(20, 30, 50, 200),
        );

        // Draw a panel border with a nice blue glow
        d.draw_rectangle_lines(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            Color::new(100, 150, 255, 255),
        );
        d.draw_rectangle_lines(
            panel_x - 1,
            panel_y - 1,
            panel_width + 2,
            panel_height + 2,
            Color::new(100, 150, 255, 100),
        );

        // Add corner decorations
        let corner_size = 15; // Increased corner size
        d.draw_rectangle(
            panel_x,
            panel_y,
            corner_size,
            corner_size,
            Color::new(255, 215, 0, 255),
        ); // Top-left
        d.draw_rectangle(
            panel_x + panel_width - corner_size,
            panel_y,
            corner_size,
            corner_size,
            Color::new(255, 215, 0, 255),
        ); // Top-right
        d.draw_rectangle(
            panel_x,
            panel_y + panel_height - corner_size,
            corner_size,
            corner_size,
            Color::new(255, 215, 0, 255),
        ); // Bottom-left
        d.draw_rectangle(
            panel_x + panel_width - corner_size,
            panel_y + panel_height - corner_size,
            corner_size,
            corner_size,
            Color::new(255, 215, 0, 255),
        ); // Bottom-right
    }

    pub fn draw_difficulty_selector(
        d: &mut RaylibDrawHandle,
        title_font: &Font,
        font: &Font,
        game: &Game,
        has_controller: bool,
    ) {
        let base_x = 340; // Adjusted for larger panel
        let base_y = 300; // Adjusted for new panel position

        // Difficulty label - larger
        d.draw_text_ex(
            title_font,
            "Difficulty",
            Vector2::new(base_x as f32, base_y as f32),
            40.0,
            1.4,
            Color::new(255, 215, 0, 255),
        );

        // Draw difficulty options as properly styled buttons - larger
        let button_y = base_y + 60;
        let button_width = 120; // Increased width
        let button_height = 50; // Increased height

        // Easy button
        let easy_selected = game.difficulty == crate::game::Difficulty::Easy;
        let easy_bg_color = if easy_selected {
            Color::new(0, 150, 0, 255) // Dark green background
        } else {
            Color::new(40, 60, 40, 255) // Dark gray-green
        };
        let easy_text_color = if easy_selected {
            Color::WHITE // White text when selected
        } else {
            Color::new(180, 180, 180, 255) // Light gray when not selected
        };

        d.draw_rectangle(base_x, button_y, button_width, button_height, easy_bg_color);

        // Hard button
        let hard_selected = game.difficulty == crate::game::Difficulty::Hard;
        let hard_bg_color = if hard_selected {
            Color::new(150, 0, 0, 255) // Dark red background
        } else {
            Color::new(60, 40, 40, 255) // Dark gray-red
        };
        let hard_text_color = if hard_selected {
            Color::WHITE // White text when selected
        } else {
            Color::new(180, 180, 180, 255) // Light gray when not selected
        };

        d.draw_rectangle(
            base_x + 140,
            button_y,
            button_width,
            button_height,
            hard_bg_color,
        );

        // Button text - properly centered and larger
        d.draw_text_ex(
            font,
            "Easy",
            Vector2::new((base_x + 35) as f32, (button_y + 12) as f32),
            24.0,
            1.0,
            easy_text_color,
        );
        d.draw_text_ex(
            font,
            "Hard",
            Vector2::new((base_x + 175) as f32, (button_y + 12) as f32),
            24.0,
            1.0,
            hard_text_color,
        );

        // Enhanced instruction with controller support
        if has_controller {
            d.draw_text_ex(
                font,
                "D-Pad Left/Right to change",
                Vector2::new((base_x + 280) as f32, (button_y + 14) as f32),
                18.0,
                1.0,
                Color::new(150, 200, 255, 255),
            );
        } else {
            d.draw_text_ex(
                font,
                "Press Left/Right arrows to change",
                Vector2::new((base_x + 280) as f32, (button_y + 14) as f32),
                18.0,
                1.0,
                Color::new(200, 200, 200, 255),
            );
        }
    }

    pub fn draw_high_scores_panel(
        d: &mut RaylibDrawHandle,
        title_font: &Font,
        font: &Font,
        game: &Game,
    ) {
        let base_x = 340; // Adjusted for larger panel
        let base_y = 450; // Moved down in the larger panel

        // High scores title - larger
        d.draw_text_ex(
            title_font,
            "High Scores",
            Vector2::new(base_x as f32, base_y as f32),
            36.0,
            1.2,
            Color::new(255, 215, 0, 255),
        );

        // Draw the top 3 scores with medal colors
        let medal_colors = [
            Color::new(255, 215, 0, 255),   // Gold
            Color::new(192, 192, 192, 255), // Silver
            Color::new(205, 127, 50, 255),  // Bronze
        ];

        for (i, score) in game.high_scores.iter().enumerate().take(3) {
            let y_offset = base_y + 50 + i as i32 * 35; // Increased spacing
            let medal_color = medal_colors.get(i).copied().unwrap_or(Color::WHITE);

            // Medal circle - larger
            let circle_center_x = base_x + 15;
            let circle_center_y = y_offset + 15;
            d.draw_circle(circle_center_x, circle_center_y, 12.0, medal_color); // Increased size
            d.draw_circle_lines(
                circle_center_x,
                circle_center_y,
                12.0,
                Color::new(0, 0, 0, 150),
            );

            // Rank number - perfectly centered and larger
            let rank_text = &(i + 1).to_string();
            d.draw_text_ex(
                font,
                rank_text,
                Vector2::new((circle_center_x - 6) as f32, (circle_center_y - 8) as f32),
                18.0,
                1.0,
                Color::BLACK,
            );

            // Score details - improved formatting and contrast with larger text
            let difficulty_color = match score.difficulty.as_str() {
                "Easy" => Color::new(0, 200, 0, 255),     // Bright green
                "Hard" => Color::new(255, 100, 100, 255), // Bright red
                _ => Color::WHITE,
            };

            let initials_and_score = format!("{} - {} pts", score.player_initials, score.score);
            d.draw_text_ex(
                font,
                &initials_and_score,
                Vector2::new((base_x + 45) as f32, (y_offset + 8) as f32),
                20.0,
                1.0,
                Color::new(240, 240, 240, 255),
            );
            d.draw_text_ex(
                font,
                &format!("({})", score.difficulty),
                Vector2::new(
                    (base_x + 45 + initials_and_score.len() as i32 * 10) as f32,
                    (y_offset + 8) as f32,
                ),
                20.0,
                1.0,
                difficulty_color,
            );
        }

        // Show a message if no scores - larger text
        if game.high_scores.is_empty() {
            d.draw_text_ex(
                font,
                "No high scores yet - be the first!",
                Vector2::new((base_x + 45) as f32, (base_y + 60) as f32),
                20.0,
                1.0,
                Color::new(200, 200, 200, 255),
            );
        }
    }

    pub fn draw_start_button(d: &mut RaylibDrawHandle, title_font: &Font, has_controller: bool) {
        let button_x = 440; // Centered on 1280px width (1280/2 - 400/2 = 440)
        let button_y = 700; // Moved to bottom of screen (800 - 80 - 20 = 700)
        let button_width = 400; // Increased from 300
        let button_height = 80; // Increased from 60

        for i in 0..6 {
            let glow_size = (i + 1) * 3;
            let alpha = 25 - i * 4;
            d.draw_rectangle(
                button_x - glow_size,
                button_y - glow_size,
                button_width + glow_size * 2,
                button_height + glow_size * 2,
                Color::new(0, 255, 100, alpha as u8),
            );
        }

        d.draw_rectangle(
            button_x,
            button_y,
            button_width,
            button_height,
            Color::new(0, 180, 0, 255),
        );
        d.draw_rectangle(
            button_x,
            button_y,
            button_width,
            button_height / 2,
            Color::new(0, 220, 0, 100),
        ); // Top highlight

        d.draw_rectangle_lines(
            button_x,
            button_y,
            button_width,
            button_height,
            Color::new(0, 255, 100, 255),
        );
        d.draw_rectangle_lines(
            button_x - 1,
            button_y - 1,
            button_width + 2,
            button_height + 2,
            Color::new(255, 255, 255, 150),
        );

        if has_controller {
            let text = "PRESS START BUTTON";
            let text_x = button_x + 85;
            let text_y = button_y + 25;
            let text_size = 28.0;

            d.draw_text_ex(
                title_font,
                text,
                Vector2::new((text_x + 2) as f32, (text_y + 2) as f32),
                text_size,
                1.2,
                Color::new(0, 0, 0, 150),
            );
            d.draw_text_ex(
                title_font,
                text,
                Vector2::new(text_x as f32, text_y as f32),
                text_size,
                1.2,
                Color::WHITE,
            );
        } else {
            let text = "PRESS SPACE TO START";
            let text_x = button_x + 80;
            let text_y = button_y + 25;
            let text_size = 28.0;

            d.draw_text_ex(
                title_font,
                text,
                Vector2::new((text_x + 2) as f32, (text_y + 2) as f32),
                text_size,
                1.2,
                Color::new(0, 0, 0, 150),
            );
            d.draw_text_ex(
                title_font,
                text,
                Vector2::new(text_x as f32, text_y as f32),
                text_size,
                1.2,
                Color::WHITE,
            );
        }
    }

    pub fn draw_controls(
        d: &mut RaylibDrawHandle,
        title_font: &Font,
        font: &Font,
        info_panel_x: i32,
        board_offset_y: i32,
        has_controller: bool,
    ) {
        // Enhanced controls title with glow effect
        let controls_x = info_panel_x + 30;
        let controls_y = board_offset_y + 350;
        
        // Glow effect for the title
        for glow in 1..=3 {
            let glow_alpha = 40 / glow;
            d.draw_text_ex(
                title_font,
                "Controls:",
                Vector2::new((controls_x + glow) as f32, (controls_y + glow) as f32),
                28.0,
                1.0,
                Color::new(255, 215, 0, glow_alpha as u8),
            );
        }
        
        // Shadow
        d.draw_text_ex(
            title_font,
            "Controls:",
            Vector2::new((controls_x + 2) as f32, (controls_y + 2) as f32),
            28.0,
            1.0,
            Color::new(0, 0, 0, 150),
        );
        
        // Main title
        d.draw_text_ex(
            title_font,
            "Controls:",
            Vector2::new(controls_x as f32, controls_y as f32),
            28.0,
            1.0,
            Color::new(255, 215, 0, 255),
        );
        
        let instructions = match has_controller {
            true => [
                ("D-Pad/Left Stick: Move card", Color::new(150, 255, 150, 255)),
                ("D-Pad Down/Stick Down: Soft drop", Color::new(200, 200, 255, 255)),
                ("A Button: Hard drop", Color::new(255, 200, 150, 255)),
                ("Start: Pause", Color::new(255, 150, 200, 255)),
            ],
            false => [
                ("Left/Right Arrow: Move card", Color::new(255, 255, 150, 255)),
                ("Down Arrow: Soft drop", Color::new(200, 200, 255, 255)),
                ("Space: Hard drop", Color::new(255, 200, 150, 255)),
                ("Escape: Pause", Color::new(255, 150, 200, 255)),
            ],
        };

        for (i, (text, color)) in instructions.iter().enumerate() {
            let y_pos = controls_y + 40 + i as i32 * 25;
            
            // Subtle shadow for each instruction
            d.draw_text_ex(
                font,
                text,
                Vector2::new((controls_x + 1) as f32, (y_pos + 1) as f32),
                18.0,
                1.0,
                Color::new(0, 0, 0, 100),
            );
            
            // Main text with color coding
            d.draw_text_ex(
                font,
                text,
                Vector2::new(controls_x as f32, y_pos as f32),
                18.0,
                1.0,
                *color,
            );
        }
    }

    pub fn draw_game_over_instructions(
        d: &mut RaylibDrawHandle,
        font: &Font,
        has_controller: bool,
    ) {
        if has_controller {
            d.draw_text_ex(
                font,
                "D-Pad: Cycle letters, A: Next/Accept, B: Backspace",
                Vector2::new(440.0, 530.0),
                20.0,
                1.0,
                Color::new(150, 200, 255, 255),
            );
        } else {
            d.draw_text_ex(
                font,
                "Type your initials, then press ENTER when done",
                Vector2::new(420.0, 530.0),
                20.0,
                1.0,
                Color::LIGHTGRAY,
            );
        }
    }

    pub fn draw_quit_confirmation(d: &mut RaylibDrawHandle, font: &Font, has_controller: bool) {
        if has_controller {
            d.draw_text_ex(
                font,
                "Press A to Quit",
                Vector2::new(560.0, 400.0),
                24.0,
                1.2,
                Color::new(255, 150, 150, 255),
            );
            d.draw_text_ex(
                font,
                "Press B to Cancel",
                Vector2::new(545.0, 440.0),
                24.0,
                1.2,
                Color::new(150, 255, 150, 255),
            );
        } else {
            d.draw_text_ex(
                font,
                "Press Y to Quit",
                Vector2::new(560.0, 400.0),
                24.0,
                1.2,
                Color::new(255, 150, 150, 255),
            );
            d.draw_text_ex(
                font,
                "Press N or ESC to Cancel",
                Vector2::new(510.0, 440.0),
                24.0,
                1.2,
                Color::new(150, 255, 150, 255),
            );
        }
    }

    pub fn draw_pause_instructions(d: &mut RaylibDrawHandle, font: &Font, has_controller: bool) {
        if has_controller {
            d.draw_text_ex(
                font,
                "Press A to Forfeit",
                Vector2::new(540.0, 420.0),
                24.0,
                1.2,
                Color::new(255, 150, 150, 255),
            );
            d.draw_text_ex(
                font,
                "Press B to Resume",
                Vector2::new(535.0, 460.0),
                24.0,
                1.2,
                Color::new(150, 255, 150, 255),
            );
        } else {
            d.draw_text_ex(
                font,
                "Press N or ESC to Resume",
                Vector2::new(495.0, 420.0),
                24.0,
                1.2,
                Color::new(150, 255, 150, 255),
            );
            d.draw_text_ex(
                font,
                "Press Y to Quit to Menu",
                Vector2::new(505.0, 460.0),
                24.0,
                1.2,
                Color::new(255, 150, 150, 255),
            );
        }
    }

    /// Draw a beautiful game board with realistic casino table lighting and depth
    pub fn draw_game_board_background(d: &mut RaylibDrawHandle, board_width: i32, board_height: i32, cell_size: i32) {
        let board_pixel_width = board_width * cell_size;
        let board_pixel_height = board_height * cell_size;
        let center_x = BOARD_OFFSET_X + board_pixel_width / 2;
        let center_y = BOARD_OFFSET_Y + board_pixel_height / 2;
        
        // Enhanced decorative frame system with more depth
        // Outermost shadow
        d.draw_rectangle(
            BOARD_OFFSET_X - 12,
            BOARD_OFFSET_Y - 12,
            board_pixel_width + 24,
            board_pixel_height + 24,
            Color::new(0, 0, 0, 100),
        );
        
        // Outer dark wood frame
        d.draw_rectangle(
            BOARD_OFFSET_X - 10,
            BOARD_OFFSET_Y - 10,
            board_pixel_width + 20,
            board_pixel_height + 20,
            Color::new(80, 40, 20, 255),
        );
        
        // Middle wood frame with grain effect
        d.draw_rectangle(
            BOARD_OFFSET_X - 8,
            BOARD_OFFSET_Y - 8,
            board_pixel_width + 16,
            board_pixel_height + 16,
            Color::new(139, 69, 19, 255),
        );
        
        // Add wood grain lines for realism
        for i in 0..8 {
            let grain_offset = i * 2;
            d.draw_line(
                BOARD_OFFSET_X - 8 + grain_offset,
                BOARD_OFFSET_Y - 8,
                BOARD_OFFSET_X - 8 + grain_offset,
                BOARD_OFFSET_Y + board_pixel_height + 8,
                Color::new(110, 55, 15, 100),
            );
        }
        
        // Inner bevel frame
        d.draw_rectangle(
            BOARD_OFFSET_X - 6,
            BOARD_OFFSET_Y - 6,
            board_pixel_width + 12,
            board_pixel_height + 12,
            Color::new(160, 82, 45, 255),
        );
        
        // Innermost highlight frame
        d.draw_rectangle(
            BOARD_OFFSET_X - 4,
            BOARD_OFFSET_Y - 4,
            board_pixel_width + 8,
            board_pixel_height + 8,
            Color::new(210, 180, 140, 255),
        );

        // Create realistic radial lighting on green felt (like casino table lighting) - OPTIMIZED
        let max_radius = ((board_pixel_width * board_pixel_width + board_pixel_height * board_pixel_height) as f32).sqrt() / 2.0;
        
        // Use efficient overlapping rectangles for smooth gradient - NO GAPS
        let gradient_steps = 25; // Reduced for performance but still smooth
        let step_width = (board_pixel_width as f32 / gradient_steps as f32).ceil() as i32;
        let step_height = (board_pixel_height as f32 / gradient_steps as f32).ceil() as i32;
        
        for y in 0..gradient_steps {
            for x in 0..gradient_steps {
                let rect_x = BOARD_OFFSET_X + x * step_width;
                let rect_y = BOARD_OFFSET_Y + y * step_height;
                
                // Make rectangles overlap slightly to eliminate gaps
                let rect_width = if x == gradient_steps - 1 { 
                    board_pixel_width - x * step_width + 2 
                } else { 
                    step_width + 2 
                };
                let rect_height = if y == gradient_steps - 1 { 
                    board_pixel_height - y * step_height + 2 
                } else { 
                    step_height + 2 
                };
                
                // Calculate center of this rectangle for distance calculation
                let center_x_offset = (rect_x + rect_width / 2) - center_x;
                let center_y_offset = (rect_y + rect_height / 2) - center_y;
                let distance = ((center_x_offset * center_x_offset + center_y_offset * center_y_offset) as f32).sqrt();
                let distance_ratio = (distance / max_radius).min(1.0);
                let light_factor = 1.0 - (distance_ratio * distance_ratio * 0.6);
                
                // Rich green felt with subtle variations
                let x_ratio = x as f32 / gradient_steps as f32;
                let y_ratio = y as f32 / gradient_steps as f32;
                let base_r = 20.0 + y_ratio * 15.0;
                let base_g = 80.0 + x_ratio * 30.0;
                let base_b = 30.0 + (x_ratio + y_ratio) * 10.0;
                
                let r = (base_r * light_factor) as u8;
                let g = (base_g * light_factor + 10.0) as u8;
                let b = (base_b * light_factor) as u8;
                
                let color = Color::new(r, g, b, 255);
                d.draw_rectangle(rect_x, rect_y, rect_width, rect_height, color);
            }
        }
        
        // Add realistic felt texture with more sophisticated pattern
        for i in 0..120 {
            let x = BOARD_OFFSET_X + (i * 47) % board_pixel_width;
            let y = BOARD_OFFSET_Y + (i * 83) % board_pixel_height;
            
            // Distance from center affects texture visibility
            let dx = x - center_x;
            let dy = y - center_y;
            let distance_from_center = ((dx * dx + dy * dy) as f32).sqrt();
            let max_distance = (board_pixel_width / 2) as f32;
            let distance_ratio = (distance_from_center / max_distance).min(1.0);
            
            // Texture is more visible in lit areas, less in shadows
            let base_alpha = 25.0 * (1.0 - distance_ratio * 0.7);
            let alpha = ((i * 19) % 15 + base_alpha as i32) as u8;
            
            let size = 0.2 + ((i * 11) % 5) as f32 * 0.1;
            d.draw_circle(x, y, size, Color::new(255, 255, 255, alpha));
        }
        
        // Add subtle fabric weave pattern
        for i in 0..15 {
            let spacing = board_pixel_width / 15;
            let x = BOARD_OFFSET_X + i * spacing;
            for j in 0..3 {
                d.draw_line(
                    x + j,
                    BOARD_OFFSET_Y,
                    x + j,
                    BOARD_OFFSET_Y + board_pixel_height,
                    Color::new(0, 0, 0, (8 + j * 3) as u8),
                );
            }
        }
        
        for i in 0..12 {
            let spacing = board_pixel_height / 12;
            let y = BOARD_OFFSET_Y + i * spacing;
            for j in 0..3 {
                d.draw_line(
                    BOARD_OFFSET_X,
                    y + j,
                    BOARD_OFFSET_X + board_pixel_width,
                    y + j,
                    Color::new(0, 0, 0, (8 + j * 3) as u8),
                );
            }
        }
        
        // Enhanced grid lines with depth and lighting awareness
        for x in 0..=board_width {
            let line_x = BOARD_OFFSET_X + x * cell_size;
            let distance_from_center = (line_x - center_x).abs() as f32;
            let max_distance = (board_pixel_width / 2) as f32;
            let distance_ratio = distance_from_center / max_distance;
            
            // Grid lines are more visible in the center (lit area)
            let alpha = (50.0 * (1.0 - distance_ratio * 0.6)) as u8;
            
            d.draw_line(
                line_x,
                BOARD_OFFSET_Y,
                line_x,
                BOARD_OFFSET_Y + board_pixel_height,
                Color::new(0, 0, 0, alpha),
            );
        }
        
        for y in 0..=board_height {
            let line_y = BOARD_OFFSET_Y + y * cell_size;
            let distance_from_center = (line_y - center_y).abs() as f32;
            let max_distance = (board_pixel_height / 2) as f32;
            let distance_ratio = distance_from_center / max_distance;
            
            // Grid lines are more visible in the center (lit area)
            let alpha = (50.0 * (1.0 - distance_ratio * 0.6)) as u8;
            
            d.draw_line(
                BOARD_OFFSET_X,
                line_y,
                BOARD_OFFSET_X + board_pixel_width,
                line_y,
                Color::new(0, 0, 0, alpha),
            );
        }
        
        // Add subtle corner accent lighting
        let corner_glow_size = 30;
        for corner in 0..4 {
            let (corner_x, corner_y) = match corner {
                0 => (BOARD_OFFSET_X, BOARD_OFFSET_Y), // Top-left
                1 => (BOARD_OFFSET_X + board_pixel_width, BOARD_OFFSET_Y), // Top-right
                2 => (BOARD_OFFSET_X, BOARD_OFFSET_Y + board_pixel_height), // Bottom-left
                _ => (BOARD_OFFSET_X + board_pixel_width, BOARD_OFFSET_Y + board_pixel_height), // Bottom-right
            };
            
            for i in 0..corner_glow_size {
                let alpha = 15 - i / 2;
                if alpha > 0 {
                    d.draw_circle(corner_x, corner_y, i as f32, Color::new(255, 255, 200, alpha as u8));
                }
            }
        }
    }
}
