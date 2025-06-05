use crate::game::Game;
use crate::ui::constants::*;
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
        let panel_x = MENU_PANEL_X;
        let panel_y = MENU_PANEL_Y;
        let panel_width = MENU_PANEL_WIDTH;
        let panel_height = MENU_PANEL_HEIGHT;
        let corner_size = MENU_CORNER_SIZE;

        let corner_positions = [
            (panel_x, panel_y),                              // Top-left
            (panel_x + panel_width - corner_size, panel_y),  // Top-right
            (panel_x, panel_y + panel_height - corner_size), // Bottom-left
            (
                panel_x + panel_width - corner_size,
                panel_y + panel_height - corner_size,
            ), // Bottom-right
        ];

        Self {
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            corner_size,
            corner_positions,
            shadow_offset: (MENU_SHADOW_OFFSET_X, MENU_SHADOW_OFFSET_Y),
            panel_bg_color: MENU_PANEL_BG_COLOR,
            panel_border_color: MENU_PANEL_BORDER_COLOR,
            panel_border_glow_color: MENU_PANEL_BORDER_GLOW_COLOR,
            corner_color: MENU_CORNER_COLOR,
            shadow_color: MENU_SHADOW_COLOR,
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
        let base_x = DIFFICULTY_BASE_X;
        let base_y = DIFFICULTY_BASE_Y;
        let button_y = base_y + DIFFICULTY_BUTTON_Y_OFFSET;
        let button_width = DIFFICULTY_BUTTON_WIDTH;
        let button_height = DIFFICULTY_BUTTON_HEIGHT;
        let hard_button_x = base_x + DIFFICULTY_HARD_BUTTON_X_OFFSET;

        Self {
            base_x,
            base_y,
            button_y,
            button_width,
            button_height,
            hard_button_x,
            easy_text_pos: Vector2::new(
                (base_x + DIFFICULTY_EASY_TEXT_X_OFFSET) as f32,
                (button_y + DIFFICULTY_EASY_TEXT_Y_OFFSET) as f32,
            ),
            hard_text_pos: Vector2::new(
                (hard_button_x + DIFFICULTY_HARD_TEXT_X_OFFSET) as f32,
                (button_y + DIFFICULTY_HARD_TEXT_Y_OFFSET) as f32,
            ),
            instruction_pos: Vector2::new(
                (base_x + DIFFICULTY_INSTRUCTION_X_OFFSET) as f32,
                (button_y + DIFFICULTY_INSTRUCTION_Y_OFFSET) as f32,
            ),
            easy_selected_bg: DIFFICULTY_EASY_SELECTED_BG,
            easy_unselected_bg: DIFFICULTY_EASY_UNSELECTED_BG,
            hard_selected_bg: DIFFICULTY_HARD_SELECTED_BG,
            hard_unselected_bg: DIFFICULTY_HARD_UNSELECTED_BG,
            selected_text_color: DIFFICULTY_SELECTED_TEXT_COLOR,
            unselected_text_color: DIFFICULTY_UNSELECTED_TEXT_COLOR,
            controller_instruction_color: DIFFICULTY_CONTROLLER_INSTRUCTION_COLOR,
            keyboard_instruction_color: DIFFICULTY_KEYBOARD_INSTRUCTION_COLOR,
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
        let base_x = HIGH_SCORE_BASE_X;
        let base_y = HIGH_SCORE_BASE_Y;

        Self {
            base_x,
            base_y,
            score_y_spacing: HIGH_SCORE_Y_SPACING,
            circle_center_x: base_x + HIGH_SCORE_CIRCLE_CENTER_X_OFFSET,
            circle_radius: HIGH_SCORE_CIRCLE_RADIUS,
            medal_colors: [
                HIGH_SCORE_GOLD_COLOR,
                HIGH_SCORE_SILVER_COLOR,
                HIGH_SCORE_BRONZE_COLOR,
            ],
            title_color: HIGH_SCORE_TITLE_COLOR,
            score_text_color: HIGH_SCORE_TEXT_COLOR,
            no_scores_color: HIGH_SCORE_NO_SCORES_COLOR,
            easy_color: HIGH_SCORE_EASY_COLOR,
            hard_color: HIGH_SCORE_HARD_COLOR,
            circle_outline_color: HIGH_SCORE_CIRCLE_OUTLINE_COLOR,
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
        let button_x = START_BUTTON_X;
        let button_y = START_BUTTON_Y;
        let button_width = START_BUTTON_WIDTH;
        let button_height = START_BUTTON_HEIGHT;

        let glow_configs: Vec<(i32, u8)> = (0..START_BUTTON_GLOW_LAYERS)
            .map(|i| {
                let glow_size = (i + 1) * START_BUTTON_GLOW_SIZE_MULTIPLIER;
                let alpha = START_BUTTON_GLOW_ALPHA_BASE - i * START_BUTTON_GLOW_ALPHA_DECREMENT;
                (glow_size, alpha as u8)
            })
            .collect();

        Self {
            button_x,
            button_y,
            button_width,
            button_height,
            glow_configs,
            controller_text_pos: Vector2::new(
                (button_x + START_BUTTON_CONTROLLER_TEXT_X_OFFSET) as f32,
                (button_y + START_BUTTON_CONTROLLER_TEXT_Y_OFFSET) as f32,
            ),
            keyboard_text_pos: Vector2::new(
                (button_x + START_BUTTON_KEYBOARD_TEXT_X_OFFSET) as f32,
                (button_y + START_BUTTON_KEYBOARD_TEXT_Y_OFFSET) as f32,
            ),
            main_button_color: START_BUTTON_MAIN_COLOR,
            highlight_color: START_BUTTON_HIGHLIGHT_COLOR,
            border_color: START_BUTTON_BORDER_COLOR,
            outer_border_color: START_BUTTON_OUTER_BORDER_COLOR,
            text_shadow_color: START_BUTTON_TEXT_SHADOW_COLOR,
            text_color: START_BUTTON_TEXT_COLOR,
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
            MENU_DIFFICULTY_TITLE_SIZE,
            MENU_DIFFICULTY_TITLE_SPACING,
            MENU_DIFFICULTY_TITLE_COLOR,
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

        d.draw_rectangle(
            layout.base_x,
            layout.button_y,
            layout.button_width,
            layout.button_height,
            easy_bg_color,
        );

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
            MENU_DIFFICULTY_BUTTON_TEXT_SIZE,
            MENU_DIFFICULTY_BUTTON_TEXT_SPACING,
            easy_text_color,
        );
        d.draw_text_ex(
            font,
            "Hard",
            layout.hard_text_pos,
            MENU_DIFFICULTY_BUTTON_TEXT_SIZE,
            MENU_DIFFICULTY_BUTTON_TEXT_SPACING,
            hard_text_color,
        );

        // Instructions with pre-computed colors
        let (instruction_text, instruction_color) = if has_controller {
            (
                "D-Pad Left/Right to change",
                layout.controller_instruction_color,
            )
        } else {
            (
                "Press Left/Right arrows to change",
                layout.keyboard_instruction_color,
            )
        };

        d.draw_text_ex(
            font,
            instruction_text,
            layout.instruction_pos,
            MENU_DIFFICULTY_INSTRUCTION_SIZE,
            MENU_DIFFICULTY_INSTRUCTION_SPACING,
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
            MENU_HIGH_SCORE_TITLE_SIZE,
            MENU_HIGH_SCORE_TITLE_SPACING,
            layout.title_color,
        );

        // Draw the top 3 scores
        for (i, score) in game.high_scores.iter().enumerate().take(3) {
            let y_offset = layout.base_y + HIGH_SCORE_TITLE_Y_OFFSET + i as i32 * layout.score_y_spacing;
            let medal_color = layout.medal_colors.get(i).copied().unwrap_or(Color::WHITE);

            // Medal circle
            let circle_center_y = y_offset + HIGH_SCORE_CIRCLE_Y_OFFSET;
            d.draw_circle(
                layout.circle_center_x,
                circle_center_y,
                layout.circle_radius,
                medal_color,
            );
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
                Vector2::new(
                    (layout.circle_center_x - 6) as f32,
                    (circle_center_y - 8) as f32,
                ),
                MENU_HIGH_SCORE_TEXT_SIZE,
                MENU_HIGH_SCORE_TEXT_SPACING,
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
                MENU_HIGH_SCORE_SCORE_SIZE,
                MENU_HIGH_SCORE_SCORE_SPACING,
                layout.score_text_color,
            );
            d.draw_text_ex(
                font,
                &format!("({})", score.difficulty),
                Vector2::new(
                    (layout.base_x + 45 + initials_and_score.len() as i32 * 10) as f32,
                    (y_offset + 8) as f32,
                ),
                MENU_HIGH_SCORE_DIFFICULTY_SIZE,
                MENU_HIGH_SCORE_DIFFICULTY_SPACING,
                difficulty_color,
            );
        }

        // Show a message if no scores
        if game.high_scores.is_empty() {
            d.draw_text_ex(
                font,
                "No high scores yet - be the first!",
                Vector2::new((layout.base_x + 45) as f32, (layout.base_y + HIGH_SCORE_TITLE_Y_OFFSET + 10) as f32),
                MENU_HIGH_SCORE_NO_SCORES_SIZE,
                MENU_HIGH_SCORE_NO_SCORES_SPACING,
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
            Vector2::new(
                text_pos.x + MENU_START_BUTTON_SHADOW_OFFSET,
                text_pos.y + MENU_START_BUTTON_SHADOW_OFFSET,
            ),
            MENU_START_BUTTON_TEXT_SIZE,
            MENU_START_BUTTON_TEXT_SPACING,
            layout.text_shadow_color,
        );

        // Main text
        d.draw_text_ex(
            title_font,
            text,
            text_pos,
            MENU_START_BUTTON_TEXT_SIZE,
            MENU_START_BUTTON_TEXT_SPACING,
            layout.text_color,
        );
    }
}
