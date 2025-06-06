use crate::game::Game;
use crate::ui::config::{HighScoreConfig, MainMenuConfig, ScreenConfig};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::prelude::Font;
use std::sync::LazyLock;

pub struct MenuRenderer;

// Pre-computed high scores layout
struct HighScoreLayout {
    base_x: i32,
    base_y: i32,
    score_y_spacing: i32,

    circle_radius: f32,
    // Background rectangle properties
    background_x: i32,
    background_y: i32,
    background_width: i32,
    background_height: i32,
    background_color: Color,
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
        let base_x = HighScoreConfig::BASE_X;
        let base_y = HighScoreConfig::BASE_Y;

        Self {
            base_x,
            base_y,
            score_y_spacing: HighScoreConfig::Y_SPACING,

            circle_radius: HighScoreConfig::CIRCLE_RADIUS,
            background_x: base_x + HighScoreConfig::BACKGROUND_X_OFFSET,
            background_y: base_y + HighScoreConfig::BACKGROUND_Y_OFFSET,
            background_width: HighScoreConfig::BACKGROUND_WIDTH,
            background_height: HighScoreConfig::BACKGROUND_HEIGHT,
            background_color: HighScoreConfig::BACKGROUND_COLOR,
            medal_colors: [
                HighScoreConfig::GOLD_COLOR,
                HighScoreConfig::SILVER_COLOR,
                HighScoreConfig::BRONZE_COLOR,
            ],
            title_color: HighScoreConfig::TITLE_COLOR,
            score_text_color: HighScoreConfig::TEXT_COLOR,
            no_scores_color: HighScoreConfig::NO_SCORES_COLOR,
            easy_color: HighScoreConfig::EASY_COLOR,
            hard_color: HighScoreConfig::HARD_COLOR,
            circle_outline_color: HighScoreConfig::CIRCLE_OUTLINE_COLOR,
        }
    }
}

// Pre-computed main menu layout
struct MainMenuLayout {
    base_x: i32,
    base_y: i32,
    option_spacing: i32,
    option_width: i32,
    option_height: i32,
    text_x_offset: i32,
    text_y_offset: i32,
    instruction_y: i32,
    // Pre-computed colors
    selected_bg: Color,
    unselected_bg: Color,
    selected_text_color: Color,
    unselected_text_color: Color,
    border_color: Color,
    instruction_color: Color,
}

impl MainMenuLayout {
    fn new() -> Self {
        let base_x = MainMenuConfig::BASE_X;
        let base_y = MainMenuConfig::BASE_Y;

        Self {
            base_x,
            base_y,
            option_spacing: MainMenuConfig::OPTION_SPACING,
            option_width: MainMenuConfig::OPTION_WIDTH,
            option_height: MainMenuConfig::OPTION_HEIGHT,
            text_x_offset: MainMenuConfig::TEXT_X_OFFSET,
            text_y_offset: MainMenuConfig::TEXT_Y_OFFSET,
            instruction_y: base_y + MainMenuConfig::INSTRUCTION_Y_OFFSET,
            selected_bg: MainMenuConfig::SELECTED_BG,
            unselected_bg: MainMenuConfig::UNSELECTED_BG,
            selected_text_color: MainMenuConfig::SELECTED_TEXT_COLOR,
            unselected_text_color: MainMenuConfig::UNSELECTED_TEXT_COLOR,
            border_color: MainMenuConfig::BORDER_COLOR,
            instruction_color: MainMenuConfig::INSTRUCTION_COLOR,
        }
    }
}

// Thread-safe lazy static initialization
static HIGH_SCORE_LAYOUT: LazyLock<HighScoreLayout> = LazyLock::new(HighScoreLayout::new);
static MAIN_MENU_LAYOUT: LazyLock<MainMenuLayout> = LazyLock::new(MainMenuLayout::new);

impl MenuRenderer {
    pub fn draw_main_menu(
        d: &mut RaylibDrawHandle,
        font: &Font,
        game: &Game,
        has_controller: bool,
    ) {
        let layout = &*MAIN_MENU_LAYOUT;
        let options = ["Start New Game", "Settings", "Quit"];

        for (i, &option_text) in options.iter().enumerate() {
            let option_y = layout.base_y + i as i32 * layout.option_spacing;
            let is_selected = game.selected_main_option == i;

            // Draw selection background
            let bg_color = if is_selected {
                layout.selected_bg
            } else {
                layout.unselected_bg
            };

            d.draw_rectangle(
                layout.base_x,
                option_y,
                layout.option_width,
                layout.option_height,
                bg_color,
            );

            // Draw border for selected option
            if is_selected {
                d.draw_rectangle_lines(
                    layout.base_x,
                    option_y,
                    layout.option_width,
                    layout.option_height,
                    layout.border_color,
                );
            }

            // Draw option text
            let text_color = if is_selected {
                layout.selected_text_color
            } else {
                layout.unselected_text_color
            };

            d.draw_text_ex(
                font,
                option_text,
                Vector2::new(
                    (layout.base_x + layout.text_x_offset) as f32,
                    (option_y + layout.text_y_offset) as f32,
                ),
                MainMenuConfig::TEXT_SIZE,
                MainMenuConfig::TEXT_SPACING,
                text_color,
            );
        }

        // Draw instructions
        let instruction_text = if has_controller {
            "D-Pad Up/Down: Navigate | A: Select"
        } else {
            "Up/Down: Navigate | ENTER: Select"
        };

        // measure instruction_text to get offset to center the text
        let instruction_width =
            d.measure_text(instruction_text, MainMenuConfig::INSTRUCTION_SIZE as i32);
        let instruction_x: f32 = 30f32 + (ScreenConfig::WIDTH - instruction_width) as f32 / 2f32;

        d.draw_text_ex(
            font,
            instruction_text,
            Vector2::new(instruction_x, layout.instruction_y as f32),
            MainMenuConfig::INSTRUCTION_SIZE,
            MainMenuConfig::INSTRUCTION_SPACING,
            layout.instruction_color,
        );
    }

    pub fn draw_high_scores_panel(
        d: &mut RaylibDrawHandle,
        title_font: &Font,
        font: &Font,
        game: &Game,
    ) {
        let layout = &*HIGH_SCORE_LAYOUT;

        // Draw background rectangle
        d.draw_rectangle(
            layout.background_x,
            layout.background_y,
            layout.background_width,
            layout.background_height,
            layout.background_color,
        );

        // High scores title - centered above both columns
        let title_text = "High Scores";
        let title_width = d.measure_text(title_text, HighScoreConfig::TITLE_SIZE as i32);
        let title_x = layout.base_x + HighScoreConfig::COLUMN_WIDTH - title_width / 2;

        d.draw_text_ex(
            title_font,
            title_text,
            Vector2::new(title_x as f32, layout.base_y as f32),
            HighScoreConfig::TITLE_SIZE,
            HighScoreConfig::TITLE_SPACING,
            layout.title_color,
        );

        // Split scores by difficulty
        let easy_scores: Vec<_> = game
            .high_scores
            .iter()
            .filter(|s| s.difficulty == "Easy")
            .take(3)
            .collect();
        let hard_scores: Vec<_> = game
            .high_scores
            .iter()
            .filter(|s| s.difficulty == "Hard")
            .take(3)
            .collect();

        // Draw Easy column
        d.draw_text_ex(
            title_font,
            "Easy",
            Vector2::new(
                layout.base_x as f32,
                (layout.base_y + HighScoreConfig::COLUMN_TITLE_Y_OFFSET) as f32,
            ),
            HighScoreConfig::DIFFICULTY_SIZE,
            HighScoreConfig::DIFFICULTY_SPACING,
            layout.easy_color,
        );

        Self::draw_scores_column(
            d,
            font,
            &easy_scores,
            layout.base_x,
            layout.base_y + HighScoreConfig::TITLE_Y_OFFSET,
            layout,
        );

        // Draw Hard column
        let hard_column_x = layout.base_x + HighScoreConfig::COLUMN_WIDTH;
        d.draw_text_ex(
            title_font,
            "Hard",
            Vector2::new(
                hard_column_x as f32,
                (layout.base_y + HighScoreConfig::COLUMN_TITLE_Y_OFFSET) as f32,
            ),
            HighScoreConfig::DIFFICULTY_SIZE,
            HighScoreConfig::DIFFICULTY_SPACING,
            layout.hard_color,
        );

        Self::draw_scores_column(
            d,
            font,
            &hard_scores,
            hard_column_x,
            layout.base_y + HighScoreConfig::TITLE_Y_OFFSET,
            layout,
        );

        // Show a message if no scores at all
        if game.high_scores.is_empty() {
            d.draw_text_ex(
                font,
                "No high scores yet - be the first!",
                Vector2::new(
                    (layout.base_x + HighScoreConfig::COLUMN_WIDTH / 4) as f32,
                    (layout.base_y + HighScoreConfig::TITLE_Y_OFFSET + 30) as f32,
                ),
                HighScoreConfig::NO_SCORES_SIZE,
                HighScoreConfig::NO_SCORES_SPACING,
                layout.no_scores_color,
            );
        }
    }

    fn draw_scores_column(
        d: &mut RaylibDrawHandle,
        font: &Font,
        scores: &[&crate::models::HighScore],
        column_x: i32,
        start_y: i32,
        layout: &HighScoreLayout,
    ) {
        for (i, score) in scores.iter().enumerate() {
            let y_offset = start_y + i as i32 * layout.score_y_spacing;
            let medal_color = layout.medal_colors.get(i).copied().unwrap_or(Color::WHITE);

            // Medal circle
            let circle_center_x = column_x + HighScoreConfig::CIRCLE_CENTER_X_OFFSET;
            let circle_center_y = y_offset + HighScoreConfig::CIRCLE_Y_OFFSET;
            d.draw_circle(
                circle_center_x,
                circle_center_y,
                layout.circle_radius,
                medal_color,
            );
            d.draw_circle_lines(
                circle_center_x,
                circle_center_y,
                layout.circle_radius,
                layout.circle_outline_color,
            );

            // Rank number
            let rank_text = &(i + 1).to_string();
            d.draw_text_ex(
                font,
                rank_text,
                Vector2::new((circle_center_x - 6) as f32, (circle_center_y - 8) as f32),
                HighScoreConfig::TEXT_SIZE,
                HighScoreConfig::TEXT_SPACING,
                Color::BLACK,
            );

            // Score details
            let initials_and_score = format!("{} - {} pts", score.player_initials, score.score);
            d.draw_text_ex(
                font,
                &initials_and_score,
                Vector2::new((column_x + 45) as f32, (y_offset + 8) as f32),
                HighScoreConfig::SCORE_SIZE,
                HighScoreConfig::SCORE_SPACING,
                layout.score_text_color,
            );
        }

        // Show message if this difficulty has no scores
        if scores.is_empty() {
            d.draw_text_ex(
                font,
                "No scores yet",
                Vector2::new((column_x + 45) as f32, (start_y + 10) as f32),
                HighScoreConfig::TEXT_SIZE,
                HighScoreConfig::TEXT_SPACING,
                layout.no_scores_color,
            );
        }
    }
}
