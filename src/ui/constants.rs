// UI Constants - Centralized magic numbers for the UI system
use raylib::prelude::*;

// Screen dimensions
pub const SCREEN_WIDTH: i32 = 1280;
pub const SCREEN_HEIGHT: i32 = 800;

// Board positioning
pub const BOARD_OFFSET_X: i32 = 100;
pub const BOARD_OFFSET_Y: i32 = 50;
pub const INFO_PANEL_X: i32 = 700;
pub const INFO_PANEL_WIDTH: i32 = 520;

// Animation constants
pub const ANIMATION_CARD_SIZE: f32 = 40.0;
pub const ANIMATION_MAX_SPEED: f32 = 30.0;
pub const ANIMATION_ANGULAR_VELOCITY_RANGE: f32 = 60.0;
pub const ANIMATION_ALPHA: u8 = 40;
pub const ANIMATION_ROTATION_MAX: f32 = 360.0;
pub const ANIMATION_GRID_COLS: i32 = 3;
pub const ANIMATION_GRID_ROWS: i32 = 4;
pub const ANIMATION_RANDOMNESS: f32 = 50.0;

// Menu panel constants
pub const MENU_PANEL_X: i32 = 290;
pub const MENU_PANEL_Y: i32 = 260;
pub const MENU_PANEL_WIDTH: i32 = 700;
pub const MENU_PANEL_HEIGHT: i32 = 380;
pub const MENU_CORNER_SIZE: i32 = 15;
pub const MENU_SHADOW_OFFSET_X: i32 = 4;
pub const MENU_SHADOW_OFFSET_Y: i32 = 4;

// Menu colors
pub const MENU_PANEL_BG_COLOR: Color = Color::new(20, 30, 50, 200);
pub const MENU_PANEL_BORDER_COLOR: Color = Color::new(100, 150, 255, 255);
pub const MENU_PANEL_BORDER_GLOW_COLOR: Color = Color::new(100, 150, 255, 100);
pub const MENU_CORNER_COLOR: Color = Color::new(255, 215, 0, 255);
pub const MENU_SHADOW_COLOR: Color = Color::new(0, 0, 0, 50);

// Difficulty selector constants
pub const DIFFICULTY_BASE_X: i32 = 340;
pub const DIFFICULTY_BASE_Y: i32 = 300;
pub const DIFFICULTY_BUTTON_Y_OFFSET: i32 = 60;
pub const DIFFICULTY_BUTTON_WIDTH: i32 = 120;
pub const DIFFICULTY_BUTTON_HEIGHT: i32 = 50;
pub const DIFFICULTY_HARD_BUTTON_X_OFFSET: i32 = 140;
pub const DIFFICULTY_EASY_TEXT_X_OFFSET: i32 = 35;
pub const DIFFICULTY_EASY_TEXT_Y_OFFSET: i32 = 12;
pub const DIFFICULTY_HARD_TEXT_X_OFFSET: i32 = 35;
pub const DIFFICULTY_HARD_TEXT_Y_OFFSET: i32 = 12;
pub const DIFFICULTY_INSTRUCTION_X_OFFSET: i32 = 280;
pub const DIFFICULTY_INSTRUCTION_Y_OFFSET: i32 = 14;

// Difficulty colors
pub const DIFFICULTY_EASY_SELECTED_BG: Color = Color::new(0, 150, 0, 255);
pub const DIFFICULTY_EASY_UNSELECTED_BG: Color = Color::new(40, 60, 40, 255);
pub const DIFFICULTY_HARD_SELECTED_BG: Color = Color::new(150, 0, 0, 255);
pub const DIFFICULTY_HARD_UNSELECTED_BG: Color = Color::new(60, 40, 40, 255);
pub const DIFFICULTY_SELECTED_TEXT_COLOR: Color = Color::WHITE;
pub const DIFFICULTY_UNSELECTED_TEXT_COLOR: Color = Color::new(180, 180, 180, 255);
pub const DIFFICULTY_CONTROLLER_INSTRUCTION_COLOR: Color = Color::new(150, 200, 255, 255);
pub const DIFFICULTY_KEYBOARD_INSTRUCTION_COLOR: Color = Color::new(200, 200, 200, 255);

// High score constants
pub const HIGH_SCORE_BASE_X: i32 = 340;
pub const HIGH_SCORE_BASE_Y: i32 = 450;
pub const HIGH_SCORE_Y_SPACING: i32 = 35;
pub const HIGH_SCORE_CIRCLE_CENTER_X_OFFSET: i32 = 15;
pub const HIGH_SCORE_CIRCLE_RADIUS: f32 = 12.0;
pub const HIGH_SCORE_TITLE_Y_OFFSET: i32 = 50;
pub const HIGH_SCORE_CIRCLE_Y_OFFSET: i32 = 15;

// High score colors
pub const HIGH_SCORE_GOLD_COLOR: Color = Color::new(255, 215, 0, 255);
pub const HIGH_SCORE_SILVER_COLOR: Color = Color::new(192, 192, 192, 255);
pub const HIGH_SCORE_BRONZE_COLOR: Color = Color::new(205, 127, 50, 255);
pub const HIGH_SCORE_TITLE_COLOR: Color = Color::new(255, 215, 0, 255);
pub const HIGH_SCORE_TEXT_COLOR: Color = Color::new(240, 240, 240, 255);
pub const HIGH_SCORE_NO_SCORES_COLOR: Color = Color::new(200, 200, 200, 255);
pub const HIGH_SCORE_EASY_COLOR: Color = Color::new(0, 200, 0, 255);
pub const HIGH_SCORE_HARD_COLOR: Color = Color::new(255, 100, 100, 255);
pub const HIGH_SCORE_CIRCLE_OUTLINE_COLOR: Color = Color::new(0, 0, 0, 150);

// Start button constants
pub const START_BUTTON_X: i32 = 440;
pub const START_BUTTON_Y: i32 = 700;
pub const START_BUTTON_WIDTH: i32 = 400;
pub const START_BUTTON_HEIGHT: i32 = 80;
pub const START_BUTTON_GLOW_LAYERS: i32 = 6;
pub const START_BUTTON_GLOW_SIZE_MULTIPLIER: i32 = 3;
pub const START_BUTTON_GLOW_ALPHA_BASE: i32 = 25;
pub const START_BUTTON_GLOW_ALPHA_DECREMENT: i32 = 4;
pub const START_BUTTON_CONTROLLER_TEXT_X_OFFSET: i32 = 85;
pub const START_BUTTON_CONTROLLER_TEXT_Y_OFFSET: i32 = 25;
pub const START_BUTTON_KEYBOARD_TEXT_X_OFFSET: i32 = 80;
pub const START_BUTTON_KEYBOARD_TEXT_Y_OFFSET: i32 = 25;

// Start button colors
pub const START_BUTTON_MAIN_COLOR: Color = Color::new(0, 180, 0, 255);
pub const START_BUTTON_HIGHLIGHT_COLOR: Color = Color::new(0, 220, 0, 100);
pub const START_BUTTON_BORDER_COLOR: Color = Color::new(0, 255, 100, 255);
pub const START_BUTTON_OUTER_BORDER_COLOR: Color = Color::new(255, 255, 255, 150);
pub const START_BUTTON_TEXT_SHADOW_COLOR: Color = Color::new(0, 0, 0, 150);
pub const START_BUTTON_TEXT_COLOR: Color = Color::WHITE;

// Text rendering constants
pub const TEXT_TITLE_X_OFFSET: f32 = 160.0;
pub const TEXT_TITLE_Y: f32 = 60.0;
pub const TEXT_TITLE_SIZE: f32 = 120.0;
pub const TEXT_TITLE_SPACING: f32 = 2.0;
pub const TEXT_SUBTITLE_X_OFFSET: f32 = 140.0;
pub const TEXT_SUBTITLE_Y: f32 = 200.0;
pub const TEXT_SUBTITLE_SIZE: f32 = 32.0;
pub const TEXT_SUBTITLE_SPACING: f32 = 1.0;

// Text shadow constants
pub const TEXT_SHADOW_OFFSET_1: Vector2 = Vector2::new(6.0, 6.0);
pub const TEXT_SHADOW_OFFSET_2: Vector2 = Vector2::new(3.0, 3.0);
pub const TEXT_SHADOW_OFFSET_3: Vector2 = Vector2::new(1.5, 1.5);
pub const TEXT_SHADOW_OFFSET_SUBTITLE: Vector2 = Vector2::new(2.0, 2.0);

// Text colors
pub const TEXT_TITLE_MAIN_COLOR: Color = Color::new(255, 215, 0, 255);
pub const TEXT_TITLE_HIGHLIGHT_COLOR: Color = Color::new(255, 255, 255, 200);
pub const TEXT_SUBTITLE_MAIN_COLOR: Color = Color::new(200, 200, 255, 255);
pub const TEXT_SHADOW_COLOR_1: Color = Color::new(0, 0, 0, 150);
pub const TEXT_SHADOW_COLOR_2: Color = Color::new(0, 0, 0, 100);
pub const TEXT_SHADOW_COLOR_3: Color = Color::new(0, 0, 0, 50);
pub const TEXT_SUBTITLE_SHADOW_COLOR: Color = Color::new(0, 0, 0, 80);

// Instructions constants
pub const INSTRUCTIONS_X_OFFSET: i32 = 30;
pub const INSTRUCTIONS_Y_OFFSET: i32 = 350;
pub const INSTRUCTIONS_TITLE_SIZE: f32 = 28.0;
pub const INSTRUCTIONS_TEXT_SIZE: f32 = 18.0;
pub const INSTRUCTIONS_LINE_SPACING: i32 = 25;
pub const INSTRUCTIONS_Y_START_OFFSET: i32 = 40;
pub const INSTRUCTIONS_GLOW_LAYERS: i32 = 3;
pub const INSTRUCTIONS_SHADOW_X_OFFSET: i32 = 2;
pub const INSTRUCTIONS_SHADOW_Y_OFFSET: i32 = 2;
pub const INSTRUCTIONS_TEXT_X_OFFSET: i32 = 1;
pub const INSTRUCTIONS_TEXT_Y_OFFSET: i32 = 1;

// Instruction colors
pub const INSTRUCTIONS_TITLE_COLOR: Color = Color::new(255, 215, 0, 255);
pub const INSTRUCTIONS_MOVE_COLOR: Color = Color::new(150, 255, 150, 255);
pub const INSTRUCTIONS_SOFT_DROP_COLOR: Color = Color::new(200, 200, 255, 255);
pub const INSTRUCTIONS_HARD_DROP_COLOR: Color = Color::new(255, 200, 150, 255);
pub const INSTRUCTIONS_PAUSE_COLOR: Color = Color::new(255, 150, 200, 255);
pub const INSTRUCTIONS_SHADOW_COLOR: Color = Color::new(0, 0, 0, 150);
pub const INSTRUCTIONS_TEXT_SHADOW_COLOR: Color = Color::new(0, 0, 0, 100);

// Game Over instruction positions
pub const GAME_OVER_INSTRUCTION_X: f32 = 440.0;
pub const GAME_OVER_INSTRUCTION_Y: f32 = 530.0;
pub const GAME_OVER_INSTRUCTION_X_ALT: f32 = 420.0;
pub const GAME_OVER_INSTRUCTION_SIZE: f32 = 20.0;

// Quit confirmation positions
pub const QUIT_CONFIRM_QUIT_X: f32 = 560.0;
pub const QUIT_CONFIRM_QUIT_Y: f32 = 400.0;
pub const QUIT_CONFIRM_CANCEL_X: f32 = 545.0;
pub const QUIT_CONFIRM_CANCEL_Y: f32 = 440.0;
pub const QUIT_CONFIRM_CANCEL_X_ALT: f32 = 510.0;
pub const QUIT_CONFIRM_SIZE: f32 = 24.0;
pub const QUIT_CONFIRM_SPACING: f32 = 1.2;

// Pause instruction positions
pub const PAUSE_FORFEIT_X: f32 = 540.0;
pub const PAUSE_FORFEIT_Y: f32 = 420.0;
pub const PAUSE_RESUME_X: f32 = 535.0;
pub const PAUSE_RESUME_Y: f32 = 460.0;
pub const PAUSE_RESUME_X_ALT: f32 = 495.0;
pub const PAUSE_QUIT_X: f32 = 505.0;

// Instruction text colors
pub const INSTRUCTION_CONTROLLER_COLOR: Color = Color::new(150, 200, 255, 255);
pub const INSTRUCTION_KEYBOARD_COLOR: Color = Color::new(255, 255, 150, 255);
pub const INSTRUCTION_QUIT_COLOR: Color = Color::new(255, 150, 150, 255);
pub const INSTRUCTION_RESUME_COLOR: Color = Color::new(150, 255, 150, 255);

// Background gradient constants
pub const BACKGROUND_GRADIENT_STEPS: i32 = 40;
pub const BACKGROUND_PARTICLE_COUNT: i32 = 25;
pub const BACKGROUND_PARTICLE_ALPHA_BASE: i32 = 10;
pub const BACKGROUND_PARTICLE_ALPHA_RANGE: i32 = 35;
pub const BACKGROUND_PARTICLE_SIZE_BASE: f32 = 0.3;
pub const BACKGROUND_PARTICLE_SIZE_RANGE: f32 = 0.1;
pub const BACKGROUND_PARTICLE_SIZE_MULTIPLIER: i32 = 7;

// Background colors - base values for gradient calculation
pub const BACKGROUND_GRADIENT_R_BASE: f32 = 8.0;
pub const BACKGROUND_GRADIENT_R_RANGE: f32 = 12.0;
pub const BACKGROUND_GRADIENT_R_SIN_MULTIPLIER: f32 = 2.0;
pub const BACKGROUND_GRADIENT_G_BASE: f32 = 15.0;
pub const BACKGROUND_GRADIENT_G_RANGE: f32 = 15.0;
pub const BACKGROUND_GRADIENT_G_SIN_MULTIPLIER: f32 = 3.0;
pub const BACKGROUND_GRADIENT_G_SIN_FREQUENCY: f32 = 2.1;
pub const BACKGROUND_GRADIENT_B_BASE: f32 = 25.0;
pub const BACKGROUND_GRADIENT_B_RANGE: f32 = 20.0;
pub const BACKGROUND_GRADIENT_B_SIN_MULTIPLIER: f32 = 4.0;
pub const BACKGROUND_GRADIENT_B_SIN_FREQUENCY: f32 = 1.7;

// Board background constants
pub const BOARD_GRADIENT_STEPS: i32 = 25;
pub const BOARD_TEXTURE_COUNT: i32 = 120;
pub const BOARD_SHADOW_OFFSET: i32 = 12;
pub const BOARD_SHADOW_SIZE: i32 = 24;
pub const BOARD_OUTER_FRAME_OFFSET: i32 = 10;
pub const BOARD_OUTER_FRAME_SIZE: i32 = 20;
pub const BOARD_MIDDLE_FRAME_OFFSET: i32 = 8;
pub const BOARD_MIDDLE_FRAME_SIZE: i32 = 16;
pub const BOARD_INNER_FRAME_OFFSET: i32 = 6;
pub const BOARD_INNER_FRAME_SIZE: i32 = 12;
pub const BOARD_HIGHLIGHT_FRAME_OFFSET: i32 = 4;
pub const BOARD_HIGHLIGHT_FRAME_SIZE: i32 = 8;
pub const BOARD_GRAIN_LINES: i32 = 8;
pub const BOARD_GRAIN_SPACING: i32 = 2;

// Board frame colors
pub const BOARD_SHADOW_COLOR: Color = Color::new(0, 0, 0, 100);
pub const BOARD_OUTER_FRAME_COLOR: Color = Color::new(80, 40, 20, 255);
pub const BOARD_MIDDLE_FRAME_COLOR: Color = Color::new(139, 69, 19, 255);
pub const BOARD_GRAIN_COLOR: Color = Color::new(110, 55, 15, 100);
pub const BOARD_INNER_FRAME_COLOR: Color = Color::new(160, 82, 45, 255);
pub const BOARD_HIGHLIGHT_FRAME_COLOR: Color = Color::new(210, 180, 140, 255);

// Particle system constants
pub const PARTICLE_SYSTEM_CAPACITY: usize = 100;
pub const PARTICLE_EXPLOSION_COUNT: usize = 35;
pub const PARTICLE_SPARKLE_COUNT: usize = 8;
pub const PARTICLE_WAVE_SIZE: usize = 12;
pub const PARTICLE_EXPLOSION_SPEED_1: f32 = 80.0;
pub const PARTICLE_EXPLOSION_SPEED_2: f32 = 60.0;
pub const PARTICLE_EXPLOSION_SPEED_3: f32 = 40.0;
pub const PARTICLE_EXPLOSION_SPEED_4: f32 = 100.0;
pub const PARTICLE_LIFE_TIME_1: f32 = 1.0;
pub const PARTICLE_LIFE_TIME_2: f32 = 1.2;
pub const PARTICLE_LIFE_TIME_3: f32 = 1.5;
pub const PARTICLE_LIFE_TIME_4: f32 = 0.8;
pub const PARTICLE_SIZE_1: f32 = 3.0;
pub const PARTICLE_SIZE_2: f32 = 2.5;
pub const PARTICLE_SIZE_3: f32 = 2.0;
pub const PARTICLE_SIZE_4: f32 = 4.0;
pub const PARTICLE_SPARKLE_SIZE: f32 = 1.5;
pub const PARTICLE_SPARKLE_LIFE: f32 = 0.6;
pub const PARTICLE_SPARKLE_SPEED: f32 = 20.0;
pub const PARTICLE_SPARKLE_UPWARD_BIAS: f32 = -30.0;
pub const PARTICLE_VELOCITY_VARIATION_RANGE: f32 = 30.0;
pub const PARTICLE_ACCELERATION_Y: f32 = 200.0;
pub const PARTICLE_SPARKLE_ACCELERATION_Y: f32 = 150.0;
pub const PARTICLE_LIFE_TIME_VARIATION: f32 = 0.05;
pub const PARTICLE_ANGULAR_VELOCITY_RANGE: f32 = 3.0;
pub const PARTICLE_SPARKLE_ANGULAR_VELOCITY_MULTIPLIER: f32 = 2.0;
pub const PARTICLE_SPARKLE_ANGULAR_VELOCITY_OFFSET: f32 = 8.0;

// Particle colors
pub const PARTICLE_COLOR_WHITE: Color = Color::WHITE;
pub const PARTICLE_COLOR_YELLOW: Color = Color::YELLOW;
pub const PARTICLE_COLOR_ORANGE: Color = Color::ORANGE;
pub const PARTICLE_COLOR_LIGHTGRAY: Color = Color::LIGHTGRAY;
pub const PARTICLE_COLOR_BLACK: Color = Color::new(30, 30, 30, 255);

// Font sizes for menu text
pub const MENU_DIFFICULTY_TITLE_SIZE: f32 = 40.0;
pub const MENU_DIFFICULTY_TITLE_SPACING: f32 = 1.4;
pub const MENU_DIFFICULTY_BUTTON_TEXT_SIZE: f32 = 24.0;
pub const MENU_DIFFICULTY_BUTTON_TEXT_SPACING: f32 = 1.0;
pub const MENU_DIFFICULTY_INSTRUCTION_SIZE: f32 = 18.0;
pub const MENU_DIFFICULTY_INSTRUCTION_SPACING: f32 = 1.0;
pub const MENU_HIGH_SCORE_TITLE_SIZE: f32 = 36.0;
pub const MENU_HIGH_SCORE_TITLE_SPACING: f32 = 1.2;
pub const MENU_HIGH_SCORE_TEXT_SIZE: f32 = 18.0;
pub const MENU_HIGH_SCORE_TEXT_SPACING: f32 = 1.0;
pub const MENU_HIGH_SCORE_NO_SCORES_SIZE: f32 = 20.0;
pub const MENU_HIGH_SCORE_NO_SCORES_SPACING: f32 = 1.0;
pub const MENU_HIGH_SCORE_DIFFICULTY_SIZE: f32 = 20.0;
pub const MENU_HIGH_SCORE_DIFFICULTY_SPACING: f32 = 1.0;
pub const MENU_HIGH_SCORE_SCORE_SIZE: f32 = 20.0;
pub const MENU_HIGH_SCORE_SCORE_SPACING: f32 = 1.0;
pub const MENU_START_BUTTON_TEXT_SIZE: f32 = 28.0;
pub const MENU_START_BUTTON_TEXT_SPACING: f32 = 1.2;
pub const MENU_START_BUTTON_SHADOW_OFFSET: f32 = 2.0;

// Menu title color constant
pub const MENU_DIFFICULTY_TITLE_COLOR: Color = Color::new(255, 215, 0, 255);
