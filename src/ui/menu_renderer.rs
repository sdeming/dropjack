use crate::game::Game;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::prelude::Font;

pub struct MenuRenderer;

impl MenuRenderer {
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
        let easy_selected = game.difficulty == crate::models::Difficulty::Easy;
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
        let hard_selected = game.difficulty == crate::models::Difficulty::Hard;
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
} 