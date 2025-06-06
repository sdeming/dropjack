use crate::game::Game;
use crate::ui::config::{
    DifficultyConfig, HighScoreConfig, MainMenuConfig, MenuConfig, ScreenConfig, StartButtonConfig,
};
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
        let panel_x = MenuConfig::PANEL_X;
        let panel_y = MenuConfig::PANEL_Y;
        let panel_width = MenuConfig::PANEL_WIDTH;
        let panel_height = MenuConfig::PANEL_HEIGHT;
        let corner_size = MenuConfig::CORNER_SIZE;

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
            shadow_offset: (MenuConfig::SHADOW_OFFSET_X, MenuConfig::SHADOW_OFFSET_Y),
            panel_bg_color: MenuConfig::PANEL_BG_COLOR,
            panel_border_color: MenuConfig::PANEL_BORDER_COLOR,
            panel_border_glow_color: MenuConfig::PANEL_BORDER_GLOW_COLOR,
            corner_color: MenuConfig::CORNER_COLOR,
            shadow_color: MenuConfig::SHADOW_COLOR,
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
        let base_x = DifficultyConfig::BASE_X;
        let base_y = DifficultyConfig::BASE_Y;
        let button_y = base_y + DifficultyConfig::BUTTON_Y_OFFSET;
        let button_width = DifficultyConfig::BUTTON_WIDTH;
        let button_height = DifficultyConfig::BUTTON_HEIGHT;
        let hard_button_x = base_x + DifficultyConfig::HARD_BUTTON_X_OFFSET;

        Self {
            base_x,
            base_y,
            button_y,
            button_width,
            button_height,
            hard_button_x,
            easy_text_pos: Vector2::new(
                (base_x + DifficultyConfig::EASY_TEXT_X_OFFSET) as f32,
                (button_y + DifficultyConfig::EASY_TEXT_Y_OFFSET) as f32,
            ),
            hard_text_pos: Vector2::new(
                (hard_button_x + DifficultyConfig::HARD_TEXT_X_OFFSET) as f32,
                (button_y + DifficultyConfig::HARD_TEXT_Y_OFFSET) as f32,
            ),
            instruction_pos: Vector2::new(
                (base_x + DifficultyConfig::INSTRUCTION_X_OFFSET) as f32,
                (button_y + DifficultyConfig::INSTRUCTION_Y_OFFSET) as f32,
            ),
            easy_selected_bg: DifficultyConfig::EASY_SELECTED_BG,
            easy_unselected_bg: DifficultyConfig::EASY_UNSELECTED_BG,
            hard_selected_bg: DifficultyConfig::HARD_SELECTED_BG,
            hard_unselected_bg: DifficultyConfig::HARD_UNSELECTED_BG,
            selected_text_color: DifficultyConfig::SELECTED_TEXT_COLOR,
            unselected_text_color: DifficultyConfig::UNSELECTED_TEXT_COLOR,
            controller_instruction_color: DifficultyConfig::CONTROLLER_INSTRUCTION_COLOR,
            keyboard_instruction_color: DifficultyConfig::KEYBOARD_INSTRUCTION_COLOR,
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
            circle_center_x: base_x + HighScoreConfig::CIRCLE_CENTER_X_OFFSET,
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
        let button_x = StartButtonConfig::X;
        let button_y = StartButtonConfig::Y;
        let button_width = StartButtonConfig::WIDTH;
        let button_height = StartButtonConfig::HEIGHT;

        let glow_configs: Vec<(i32, u8)> = (0..StartButtonConfig::GLOW_LAYERS)
            .map(|i| {
                let glow_size = (i + 1) * StartButtonConfig::GLOW_SIZE_MULTIPLIER;
                let alpha = StartButtonConfig::GLOW_ALPHA_BASE
                    - i * StartButtonConfig::GLOW_ALPHA_DECREMENT;
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
                (button_x + StartButtonConfig::CONTROLLER_TEXT_X_OFFSET) as f32,
                (button_y + StartButtonConfig::CONTROLLER_TEXT_Y_OFFSET) as f32,
            ),
            keyboard_text_pos: Vector2::new(
                (button_x + StartButtonConfig::KEYBOARD_TEXT_X_OFFSET) as f32,
                (button_y + StartButtonConfig::KEYBOARD_TEXT_Y_OFFSET) as f32,
            ),
            main_button_color: StartButtonConfig::MAIN_COLOR,
            highlight_color: StartButtonConfig::HIGHLIGHT_COLOR,
            border_color: StartButtonConfig::BORDER_COLOR,
            outer_border_color: StartButtonConfig::OUTER_BORDER_COLOR,
            text_shadow_color: StartButtonConfig::TEXT_SHADOW_COLOR,
            text_color: StartButtonConfig::TEXT_COLOR,
        }
    }
}

// Thread-safe lazy static initialization
static PANEL_LAYOUT: LazyLock<PanelLayout> = LazyLock::new(PanelLayout::new);
static DIFFICULTY_LAYOUT: LazyLock<DifficultyLayout> = LazyLock::new(DifficultyLayout::new);
static HIGH_SCORE_LAYOUT: LazyLock<HighScoreLayout> = LazyLock::new(HighScoreLayout::new);
static MAIN_MENU_LAYOUT: LazyLock<MainMenuLayout> = LazyLock::new(MainMenuLayout::new);
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
            DifficultyConfig::TITLE_SIZE,
            DifficultyConfig::TITLE_SPACING,
            DifficultyConfig::TITLE_COLOR,
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
            DifficultyConfig::BUTTON_TEXT_SIZE,
            DifficultyConfig::BUTTON_TEXT_SPACING,
            easy_text_color,
        );
        d.draw_text_ex(
            font,
            "Hard",
            layout.hard_text_pos,
            DifficultyConfig::BUTTON_TEXT_SIZE,
            DifficultyConfig::BUTTON_TEXT_SPACING,
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
            DifficultyConfig::INSTRUCTION_SIZE,
            DifficultyConfig::INSTRUCTION_SPACING,
            instruction_color,
        );
    }

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
            "D-Pad Up/Down: Navigate | A: Select | B: Quit"
        } else {
            "Up/Down: Navigate | ENTER: Select | ESC: Quit"
        };

        // measure instruction_text to get offset to center the text
        let instruction_width =
            d.measure_text(instruction_text, MainMenuConfig::INSTRUCTION_SIZE as i32);
        let instruction_x: f32 = 60f32 + (ScreenConfig::WIDTH - instruction_width) as f32 / 2f32;

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
                text_pos.x + StartButtonConfig::SHADOW_OFFSET,
                text_pos.y + StartButtonConfig::SHADOW_OFFSET,
            ),
            StartButtonConfig::TEXT_SIZE,
            StartButtonConfig::TEXT_SPACING,
            layout.text_shadow_color,
        );

        // Main text
        d.draw_text_ex(
            title_font,
            text,
            text_pos,
            StartButtonConfig::TEXT_SIZE,
            StartButtonConfig::TEXT_SPACING,
            layout.text_color,
        );
    }
}
