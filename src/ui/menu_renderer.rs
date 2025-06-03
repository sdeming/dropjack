use crate::game::Game;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::prelude::Font;
use std::sync::LazyLock;

pub struct MenuRenderer;

// Pre-computed panel layout
struct PanelLayout {
    panel_x: i32,
    panel_y: i32,
    panel_width: i32,
    panel_height: i32,
    corner_size: i32,
    // Pre-computed corner positions
    corner_positions: [(i32, i32); 4],
    // Pre-computed shadow offset
    shadow_offset: (i32, i32),
    // Pre-computed colors
    panel_bg_color: Color,
    panel_border_color: Color,
    panel_border_glow_color: Color,
    corner_color: Color,
    shadow_color: Color,
}

impl PanelLayout {
    fn new() -> Self {
        let panel_x = 290;
        let panel_y = 260;
        let panel_width = 700;
        let panel_height = 380;
        let corner_size = 15;

        let corner_positions = [
            (panel_x, panel_y), // Top-left
            (panel_x + panel_width - corner_size, panel_y), // Top-right
            (panel_x, panel_y + panel_height - corner_size), // Bottom-left
            (panel_x + panel_width - corner_size, panel_y + panel_height - corner_size), // Bottom-right
        ];

        Self {
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            corner_size,
            corner_positions,
            shadow_offset: (4, 4),
            panel_bg_color: Color::new(20, 30, 50, 200),
            panel_border_color: Color::new(100, 150, 255, 255),
            panel_border_glow_color: Color::new(100, 150, 255, 100),
            corner_color: Color::new(255, 215, 0, 255),
            shadow_color: Color::new(0, 0, 0, 50),
        }
    }
}

// Pre-computed difficulty button layout
struct DifficultyLayout {
    base_x: i32,
    base_y: i32,
    button_y: i32,
    button_width: i32,
    button_height: i32,
    hard_button_x: i32,
    // Pre-computed text positions
    easy_text_pos: Vector2,
    hard_text_pos: Vector2,
    instruction_pos: Vector2,
    // Pre-computed colors
    easy_selected_bg: Color,
    easy_unselected_bg: Color,
    hard_selected_bg: Color,
    hard_unselected_bg: Color,
    selected_text_color: Color,
    unselected_text_color: Color,
    controller_instruction_color: Color,
    keyboard_instruction_color: Color,
}

impl DifficultyLayout {
    fn new() -> Self {
        let base_x = 340;
        let base_y = 300;
        let button_y = base_y + 60;
        let button_width = 120;
        let button_height = 50;
        let hard_button_x = base_x + 140;

        Self {
            base_x,
            base_y,
            button_y,
            button_width,
            button_height,
            hard_button_x,
            easy_text_pos: Vector2::new((base_x + 35) as f32, (button_y + 12) as f32),
            hard_text_pos: Vector2::new((hard_button_x + 35) as f32, (button_y + 12) as f32),
            instruction_pos: Vector2::new((base_x + 280) as f32, (button_y + 14) as f32),
            easy_selected_bg: Color::new(0, 150, 0, 255),
            easy_unselected_bg: Color::new(40, 60, 40, 255),
            hard_selected_bg: Color::new(150, 0, 0, 255),
            hard_unselected_bg: Color::new(60, 40, 40, 255),
            selected_text_color: Color::WHITE,
            unselected_text_color: Color::new(180, 180, 180, 255),
            controller_instruction_color: Color::new(150, 200, 255, 255),
            keyboard_instruction_color: Color::new(200, 200, 200, 255),
        }
    }
}

// Pre-computed high scores layout
struct HighScoreLayout {
    base_x: i32,
    base_y: i32,
    score_y_spacing: i32,
    circle_center_x: i32,
    circle_radius: f32,
    // Pre-computed medal colors
    medal_colors: [Color; 3],
    // Pre-computed text colors
    title_color: Color,
    score_text_color: Color,
    no_scores_color: Color,
    easy_color: Color,
    hard_color: Color,
    circle_outline_color: Color,
}

impl HighScoreLayout {
    fn new() -> Self {
        let base_x = 340;
        let base_y = 450;

        Self {
            base_x,
            base_y,
            score_y_spacing: 35,
            circle_center_x: base_x + 15,
            circle_radius: 12.0,
            medal_colors: [
                Color::new(255, 215, 0, 255),   // Gold
                Color::new(192, 192, 192, 255), // Silver
                Color::new(205, 127, 50, 255),  // Bronze
            ],
            title_color: Color::new(255, 215, 0, 255),
            score_text_color: Color::new(240, 240, 240, 255),
            no_scores_color: Color::new(200, 200, 200, 255),
            easy_color: Color::new(0, 200, 0, 255),
            hard_color: Color::new(255, 100, 100, 255),
            circle_outline_color: Color::new(0, 0, 0, 150),
        }
    }
}

// Pre-computed start button layout
struct StartButtonLayout {
    button_x: i32,
    button_y: i32,
    button_width: i32,
    button_height: i32,
    // Pre-computed glow effects
    glow_configs: Vec<(i32, u8)>, // (glow_size, alpha)
    // Pre-computed text positions
    controller_text_pos: Vector2,
    keyboard_text_pos: Vector2,
    // Pre-computed colors
    main_button_color: Color,
    highlight_color: Color,
    border_color: Color,
    outer_border_color: Color,
    text_shadow_color: Color,
    text_color: Color,
}

impl StartButtonLayout {
    fn new() -> Self {
        let button_x = 440;
        let button_y = 700;
        let button_width = 400;
        let button_height = 80;

        let glow_configs: Vec<(i32, u8)> = (0..6).map(|i| {
            let glow_size = (i + 1) * 3;
            let alpha = 25 - i * 4;
            (glow_size, alpha as u8)
        }).collect();

        Self {
            button_x,
            button_y,
            button_width,
            button_height,
            glow_configs,
            controller_text_pos: Vector2::new((button_x + 85) as f32, (button_y + 25) as f32),
            keyboard_text_pos: Vector2::new((button_x + 80) as f32, (button_y + 25) as f32),
            main_button_color: Color::new(0, 180, 0, 255),
            highlight_color: Color::new(0, 220, 0, 100),
            border_color: Color::new(0, 255, 100, 255),
            outer_border_color: Color::new(255, 255, 255, 150),
            text_shadow_color: Color::new(0, 0, 0, 150),
            text_color: Color::WHITE,
        }
    }
}

// Thread-safe lazy static initialization
static PANEL_LAYOUT: LazyLock<PanelLayout> = LazyLock::new(PanelLayout::new);
static DIFFICULTY_LAYOUT: LazyLock<DifficultyLayout> = LazyLock::new(DifficultyLayout::new);
static HIGH_SCORE_LAYOUT: LazyLock<HighScoreLayout> = LazyLock::new(HighScoreLayout::new);
static START_BUTTON_LAYOUT: LazyLock<StartButtonLayout> = LazyLock::new(StartButtonLayout::new);

impl MenuRenderer {
    pub fn draw_main_panel(d: &mut RaylibDrawHandle) {
        let layout = &*PANEL_LAYOUT;
        
        // Draw panel shadow
        d.draw_rectangle(
            layout.panel_x + layout.shadow_offset.0,
            layout.panel_y + layout.shadow_offset.1,
            layout.panel_width,
            layout.panel_height,
            layout.shadow_color,
        );

        // Draw the main panel
        d.draw_rectangle(
            layout.panel_x,
            layout.panel_y,
            layout.panel_width,
            layout.panel_height,
            layout.panel_bg_color,
        );

        // Draw panel borders
        d.draw_rectangle_lines(
            layout.panel_x,
            layout.panel_y,
            layout.panel_width,
            layout.panel_height,
            layout.panel_border_color,
        );
        d.draw_rectangle_lines(
            layout.panel_x - 1,
            layout.panel_y - 1,
            layout.panel_width + 2,
            layout.panel_height + 2,
            layout.panel_border_glow_color,
        );

        // Add corner decorations using pre-computed positions
        for &(corner_x, corner_y) in &layout.corner_positions {
            d.draw_rectangle(
                corner_x,
                corner_y,
                layout.corner_size,
                layout.corner_size,
                layout.corner_color,
            );
        }
    }

    pub fn draw_difficulty_selector(
        d: &mut RaylibDrawHandle,
        title_font: &Font,
        font: &Font,
        game: &Game,
        has_controller: bool,
    ) {
        let layout = &*DIFFICULTY_LAYOUT;
        
        // Difficulty label
        d.draw_text_ex(
            title_font,
            "Difficulty",
            Vector2::new(layout.base_x as f32, layout.base_y as f32),
            40.0,
            1.4,
            Color::new(255, 215, 0, 255),
        );

        // Easy button
        let easy_selected = game.difficulty == crate::models::Difficulty::Easy;
        let easy_bg_color = if easy_selected {
            layout.easy_selected_bg
        } else {
            layout.easy_unselected_bg
        };
        let easy_text_color = if easy_selected {
            layout.selected_text_color
        } else {
            layout.unselected_text_color
        };

        d.draw_rectangle(layout.base_x, layout.button_y, layout.button_width, layout.button_height, easy_bg_color);

        // Hard button
        let hard_selected = game.difficulty == crate::models::Difficulty::Hard;
        let hard_bg_color = if hard_selected {
            layout.hard_selected_bg
        } else {
            layout.hard_unselected_bg
        };
        let hard_text_color = if hard_selected {
            layout.selected_text_color
        } else {
            layout.unselected_text_color
        };

        d.draw_rectangle(
            layout.hard_button_x,
            layout.button_y,
            layout.button_width,
            layout.button_height,
            hard_bg_color,
        );

        // Button text using pre-computed positions
        d.draw_text_ex(
            font,
            "Easy",
            layout.easy_text_pos,
            24.0,
            1.0,
            easy_text_color,
        );
        d.draw_text_ex(
            font,
            "Hard",
            layout.hard_text_pos,
            24.0,
            1.0,
            hard_text_color,
        );

        // Instructions with pre-computed colors
        let (instruction_text, instruction_color) = if has_controller {
            ("D-Pad Left/Right to change", layout.controller_instruction_color)
        } else {
            ("Press Left/Right arrows to change", layout.keyboard_instruction_color)
        };
        
        d.draw_text_ex(
            font,
            instruction_text,
            layout.instruction_pos,
            18.0,
            1.0,
            instruction_color,
        );
    }

    pub fn draw_high_scores_panel(
        d: &mut RaylibDrawHandle,
        title_font: &Font,
        font: &Font,
        game: &Game,
    ) {
        let layout = &*HIGH_SCORE_LAYOUT;
        
        // High scores title
        d.draw_text_ex(
            title_font,
            "High Scores",
            Vector2::new(layout.base_x as f32, layout.base_y as f32),
            36.0,
            1.2,
            layout.title_color,
        );

        // Draw the top 3 scores
        for (i, score) in game.high_scores.iter().enumerate().take(3) {
            let y_offset = layout.base_y + 50 + i as i32 * layout.score_y_spacing;
            let medal_color = layout.medal_colors.get(i).copied().unwrap_or(Color::WHITE);

            // Medal circle
            let circle_center_y = y_offset + 15;
            d.draw_circle(layout.circle_center_x, circle_center_y, layout.circle_radius, medal_color);
            d.draw_circle_lines(
                layout.circle_center_x,
                circle_center_y,
                layout.circle_radius,
                layout.circle_outline_color,
            );

            // Rank number
            let rank_text = &(i + 1).to_string();
            d.draw_text_ex(
                font,
                rank_text,
                Vector2::new((layout.circle_center_x - 6) as f32, (circle_center_y - 8) as f32),
                18.0,
                1.0,
                Color::BLACK,
            );

            // Score details
            let difficulty_color = match score.difficulty.as_str() {
                "Easy" => layout.easy_color,
                "Hard" => layout.hard_color,
                _ => Color::WHITE,
            };

            let initials_and_score = format!("{} - {} pts", score.player_initials, score.score);
            d.draw_text_ex(
                font,
                &initials_and_score,
                Vector2::new((layout.base_x + 45) as f32, (y_offset + 8) as f32),
                20.0,
                1.0,
                layout.score_text_color,
            );
            d.draw_text_ex(
                font,
                &format!("({})", score.difficulty),
                Vector2::new(
                    (layout.base_x + 45 + initials_and_score.len() as i32 * 10) as f32,
                    (y_offset + 8) as f32,
                ),
                20.0,
                1.0,
                difficulty_color,
            );
        }

        // Show a message if no scores
        if game.high_scores.is_empty() {
            d.draw_text_ex(
                font,
                "No high scores yet - be the first!",
                Vector2::new((layout.base_x + 45) as f32, (layout.base_y + 60) as f32),
                20.0,
                1.0,
                layout.no_scores_color,
            );
        }
    }

    pub fn draw_start_button(d: &mut RaylibDrawHandle, title_font: &Font, has_controller: bool) {
        let layout = &*START_BUTTON_LAYOUT;
        
        // Draw glow effects using pre-computed values
        for &(glow_size, alpha) in &layout.glow_configs {
            d.draw_rectangle(
                layout.button_x - glow_size,
                layout.button_y - glow_size,
                layout.button_width + glow_size * 2,
                layout.button_height + glow_size * 2,
                Color::new(0, 255, 100, alpha),
            );
        }

        // Main button
        d.draw_rectangle(
            layout.button_x,
            layout.button_y,
            layout.button_width,
            layout.button_height,
            layout.main_button_color,
        );
        
        // Top highlight
        d.draw_rectangle(
            layout.button_x,
            layout.button_y,
            layout.button_width,
            layout.button_height / 2,
            layout.highlight_color,
        );

        // Borders
        d.draw_rectangle_lines(
            layout.button_x,
            layout.button_y,
            layout.button_width,
            layout.button_height,
            layout.border_color,
        );
        d.draw_rectangle_lines(
            layout.button_x - 1,
            layout.button_y - 1,
            layout.button_width + 2,
            layout.button_height + 2,
            layout.outer_border_color,
        );

        // Text using pre-computed positions
        let (text, text_pos) = if has_controller {
            ("PRESS START BUTTON", layout.controller_text_pos)
        } else {
            ("PRESS SPACE TO START", layout.keyboard_text_pos)
        };

        // Shadow
        d.draw_text_ex(
            title_font,
            text,
            Vector2::new(text_pos.x + 2.0, text_pos.y + 2.0),
            28.0,
            1.2,
            layout.text_shadow_color,
        );
        
        // Main text
        d.draw_text_ex(
            title_font,
            text,
            text_pos,
            28.0,
            1.2,
            layout.text_color,
        );
    }
} 