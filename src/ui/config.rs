/// UI Configuration Module
///
/// This module provides structured configuration for UI components,
/// improving maintainability and reducing magic numbers throughout the codebase.
use raylib::prelude::*;

/// Screen and layout configuration
pub struct ScreenConfig;

impl ScreenConfig {
    pub const WIDTH: i32 = 1280;
    pub const HEIGHT: i32 = 800;
}

/// Menu panel styling and layout
pub struct MenuConfig;

impl MenuConfig {
    // Panel dimensions and position
    pub const PANEL_X: i32 = 290;
    pub const PANEL_Y: i32 = 260;
    pub const PANEL_WIDTH: i32 = 700;
    pub const PANEL_HEIGHT: i32 = 380;
    pub const CORNER_SIZE: i32 = 15;
    pub const SHADOW_OFFSET_X: i32 = 4;
    pub const SHADOW_OFFSET_Y: i32 = 4;

    // Panel colors
    pub const PANEL_BG_COLOR: Color = Color::new(20, 30, 50, 200);
    pub const PANEL_BORDER_COLOR: Color = Color::new(100, 150, 255, 255);
    pub const PANEL_BORDER_GLOW_COLOR: Color = Color::new(100, 150, 255, 100);
    pub const CORNER_COLOR: Color = Color::new(255, 215, 0, 255);
    pub const SHADOW_COLOR: Color = Color::new(0, 0, 0, 50);
}

/// Text styling and typography
pub struct TextConfig;

impl TextConfig {
    // Title styling
    pub const TITLE_SIZE: f32 = 120.0;
    pub const TITLE_SPACING: f32 = 2.0;
    pub const TITLE_X_OFFSET: f32 = 160.0;
    pub const TITLE_Y: f32 = 60.0;
    pub const TITLE_MAIN_COLOR: Color = Color::new(255, 215, 0, 255);
    pub const TITLE_HIGHLIGHT_COLOR: Color = Color::new(255, 255, 255, 200);

    // Subtitle styling
    pub const SUBTITLE_SIZE: f32 = 32.0;
    pub const SUBTITLE_SPACING: f32 = 1.0;
    pub const SUBTITLE_X_OFFSET: f32 = 140.0;
    pub const SUBTITLE_Y: f32 = 200.0;
    pub const SUBTITLE_MAIN_COLOR: Color = Color::new(200, 200, 255, 255);

    // Shadow configuration
    pub const SHADOW_OFFSET_1: Vector2 = Vector2::new(6.0, 6.0);
    pub const SHADOW_OFFSET_2: Vector2 = Vector2::new(3.0, 3.0);
    pub const SHADOW_OFFSET_3: Vector2 = Vector2::new(1.5, 1.5);
    pub const SHADOW_OFFSET_SUBTITLE: Vector2 = Vector2::new(2.0, 2.0);
    pub const SHADOW_COLOR_1: Color = Color::new(0, 0, 0, 150);
    pub const SHADOW_COLOR_2: Color = Color::new(0, 0, 0, 100);
    pub const SHADOW_COLOR_3: Color = Color::new(0, 0, 0, 50);
    pub const SUBTITLE_SHADOW_COLOR: Color = Color::new(0, 0, 0, 80);
}

/// Animation configuration constants
pub struct AnimationConfig;

impl AnimationConfig {
    pub const CARD_SIZE: f32 = 40.0;
    pub const MAX_SPEED: f32 = 30.0;
    pub const ANGULAR_VELOCITY_RANGE: f32 = 60.0;
    pub const ALPHA: u8 = 40;
    pub const ROTATION_MAX: f32 = 360.0;
    pub const GRID_COLS: i32 = 3;
    pub const GRID_ROWS: i32 = 4;
    pub const RANDOMNESS: f32 = 50.0;
}

/// Difficulty selector configuration
pub struct DifficultyConfig;

impl DifficultyConfig {
    // Layout
    pub const BASE_X: i32 = 340;
    pub const BASE_Y: i32 = 300;
    pub const BUTTON_Y_OFFSET: i32 = 60;
    pub const BUTTON_WIDTH: i32 = 120;
    pub const BUTTON_HEIGHT: i32 = 50;
    pub const HARD_BUTTON_X_OFFSET: i32 = 140;
    pub const EASY_TEXT_X_OFFSET: i32 = 35;
    pub const EASY_TEXT_Y_OFFSET: i32 = 12;
    pub const HARD_TEXT_X_OFFSET: i32 = 35;
    pub const HARD_TEXT_Y_OFFSET: i32 = 12;
    pub const INSTRUCTION_X_OFFSET: i32 = 280;
    pub const INSTRUCTION_Y_OFFSET: i32 = 14;

    // Colors
    pub const EASY_SELECTED_BG: Color = Color::new(0, 150, 0, 255);
    pub const EASY_UNSELECTED_BG: Color = Color::new(40, 60, 40, 255);
    pub const HARD_SELECTED_BG: Color = Color::new(150, 0, 0, 255);
    pub const HARD_UNSELECTED_BG: Color = Color::new(60, 40, 40, 255);
    pub const SELECTED_TEXT_COLOR: Color = Color::WHITE;
    pub const UNSELECTED_TEXT_COLOR: Color = Color::new(180, 180, 180, 255);
    pub const CONTROLLER_INSTRUCTION_COLOR: Color = Color::new(150, 200, 255, 255);
    pub const KEYBOARD_INSTRUCTION_COLOR: Color = Color::new(200, 200, 200, 255);

    // Typography
    pub const TITLE_SIZE: f32 = 40.0;
    pub const TITLE_SPACING: f32 = 1.4;
    pub const BUTTON_TEXT_SIZE: f32 = 24.0;
    pub const BUTTON_TEXT_SPACING: f32 = 1.0;
    pub const INSTRUCTION_SIZE: f32 = 18.0;
    pub const INSTRUCTION_SPACING: f32 = 1.0;
    pub const TITLE_COLOR: Color = Color::new(255, 215, 0, 255);
}

/// High score display configuration
pub struct HighScoreConfig;

impl HighScoreConfig {
    // Layout
    pub const BASE_X: i32 = 340;
    pub const BASE_Y: i32 = 450;
    pub const Y_SPACING: i32 = 35;
    pub const CIRCLE_CENTER_X_OFFSET: i32 = 15;
    pub const CIRCLE_RADIUS: f32 = 12.0;
    pub const TITLE_Y_OFFSET: i32 = 50;
    pub const CIRCLE_Y_OFFSET: i32 = 15;

    // Colors
    pub const GOLD_COLOR: Color = Color::new(255, 215, 0, 255);
    pub const SILVER_COLOR: Color = Color::new(192, 192, 192, 255);
    pub const BRONZE_COLOR: Color = Color::new(205, 127, 50, 255);
    pub const TITLE_COLOR: Color = Color::new(255, 215, 0, 255);
    pub const TEXT_COLOR: Color = Color::new(240, 240, 240, 255);
    pub const NO_SCORES_COLOR: Color = Color::new(200, 200, 200, 255);
    pub const EASY_COLOR: Color = Color::new(0, 200, 0, 255);
    pub const HARD_COLOR: Color = Color::new(255, 100, 100, 255);
    pub const CIRCLE_OUTLINE_COLOR: Color = Color::new(0, 0, 0, 150);

    // Typography
    pub const TITLE_SIZE: f32 = 36.0;
    pub const TITLE_SPACING: f32 = 1.2;
    pub const TEXT_SIZE: f32 = 18.0;
    pub const TEXT_SPACING: f32 = 1.0;
    pub const NO_SCORES_SIZE: f32 = 20.0;
    pub const NO_SCORES_SPACING: f32 = 1.0;
    pub const DIFFICULTY_SIZE: f32 = 20.0;
    pub const DIFFICULTY_SPACING: f32 = 1.0;
    pub const SCORE_SIZE: f32 = 20.0;
    pub const SCORE_SPACING: f32 = 1.0;
}

/// Start button configuration
pub struct StartButtonConfig;

impl StartButtonConfig {
    // Layout
    pub const X: i32 = 440;
    pub const Y: i32 = 700;
    pub const WIDTH: i32 = 400;
    pub const HEIGHT: i32 = 80;
    pub const GLOW_LAYERS: i32 = 6;
    pub const GLOW_SIZE_MULTIPLIER: i32 = 3;
    pub const GLOW_ALPHA_BASE: i32 = 25;
    pub const GLOW_ALPHA_DECREMENT: i32 = 4;
    pub const CONTROLLER_TEXT_X_OFFSET: i32 = 85;
    pub const CONTROLLER_TEXT_Y_OFFSET: i32 = 25;
    pub const KEYBOARD_TEXT_X_OFFSET: i32 = 80;
    pub const KEYBOARD_TEXT_Y_OFFSET: i32 = 25;

    // Colors
    pub const MAIN_COLOR: Color = Color::new(0, 180, 0, 255);
    pub const HIGHLIGHT_COLOR: Color = Color::new(0, 220, 0, 100);
    pub const BORDER_COLOR: Color = Color::new(0, 255, 100, 255);
    pub const OUTER_BORDER_COLOR: Color = Color::new(255, 255, 255, 150);
    pub const TEXT_SHADOW_COLOR: Color = Color::new(0, 0, 0, 150);
    pub const TEXT_COLOR: Color = Color::WHITE;

    // Typography
    pub const TEXT_SIZE: f32 = 28.0;
    pub const TEXT_SPACING: f32 = 1.2;
    pub const SHADOW_OFFSET: f32 = 2.0;
}

/// Instructions and controls configuration
pub struct InstructionsConfig;

impl InstructionsConfig {
    // Layout
    pub const X_OFFSET: i32 = 30;
    pub const Y_OFFSET: i32 = 350;
    pub const LINE_SPACING: i32 = 25;
    pub const Y_START_OFFSET: i32 = 40;
    pub const GLOW_LAYERS: i32 = 3;
    pub const SHADOW_X_OFFSET: i32 = 2;
    pub const SHADOW_Y_OFFSET: i32 = 2;
    pub const TEXT_X_OFFSET: i32 = 1;
    pub const TEXT_Y_OFFSET: i32 = 1;

    // Typography
    pub const TITLE_SIZE: f32 = 28.0;
    pub const TEXT_SIZE: f32 = 18.0;

    // Colors
    pub const TITLE_COLOR: Color = Color::new(255, 215, 0, 255);
    pub const MOVE_COLOR: Color = Color::new(150, 255, 150, 255);
    pub const SOFT_DROP_COLOR: Color = Color::new(200, 200, 255, 255);
    pub const HARD_DROP_COLOR: Color = Color::new(255, 200, 150, 255);
    pub const PAUSE_COLOR: Color = Color::new(255, 150, 200, 255);
    pub const SHADOW_COLOR: Color = Color::new(0, 0, 0, 150);
    pub const TEXT_SHADOW_COLOR: Color = Color::new(0, 0, 0, 100);
    pub const CONTROLLER_COLOR: Color = Color::new(150, 200, 255, 255);
    pub const KEYBOARD_COLOR: Color = Color::new(255, 255, 150, 255);
    pub const QUIT_COLOR: Color = Color::new(255, 150, 150, 255);
    pub const RESUME_COLOR: Color = Color::new(150, 255, 150, 255);

    // Game Over instructions
    pub const GAME_OVER_X: f32 = 440.0;
    pub const GAME_OVER_Y: f32 = 530.0;
    pub const GAME_OVER_X_ALT: f32 = 420.0;
    pub const GAME_OVER_SIZE: f32 = 20.0;

    // Quit confirmation
    pub const QUIT_CONFIRM_QUIT_X: f32 = 560.0;
    pub const QUIT_CONFIRM_QUIT_Y: f32 = 400.0;
    pub const QUIT_CONFIRM_CANCEL_X: f32 = 545.0;
    pub const QUIT_CONFIRM_CANCEL_Y: f32 = 440.0;
    pub const QUIT_CONFIRM_CANCEL_X_ALT: f32 = 510.0;
    pub const QUIT_CONFIRM_SIZE: f32 = 24.0;
    pub const QUIT_CONFIRM_SPACING: f32 = 1.2;

    // Pause instructions
    pub const PAUSE_FORFEIT_X: f32 = 540.0;
    pub const PAUSE_FORFEIT_Y: f32 = 420.0;
    pub const PAUSE_RESUME_X: f32 = 535.0;
    pub const PAUSE_RESUME_Y: f32 = 460.0;
    pub const PAUSE_RESUME_X_ALT: f32 = 495.0;
    pub const PAUSE_QUIT_X: f32 = 505.0;
}

/// Background rendering configuration
pub struct BackgroundConfig;

impl BackgroundConfig {
    // Gradient settings
    pub const GRADIENT_STEPS: i32 = 40;
    pub const PARTICLE_COUNT: i32 = 25;
    pub const PARTICLE_ALPHA_BASE: i32 = 10;
    pub const PARTICLE_ALPHA_RANGE: i32 = 20;
    pub const PARTICLE_SIZE_BASE: f32 = 0.3;
    pub const PARTICLE_SIZE_RANGE: f32 = 0.1;
    pub const PARTICLE_SIZE_MULTIPLIER: i32 = 7;

    // Color gradient math constants
    pub const GRADIENT_R_BASE: f32 = 8.0;
    pub const GRADIENT_R_RANGE: f32 = 12.0;
    pub const GRADIENT_R_SIN_MULTIPLIER: f32 = 2.0;
    pub const GRADIENT_G_BASE: f32 = 15.0;
    pub const GRADIENT_G_RANGE: f32 = 15.0;
    pub const GRADIENT_G_SIN_MULTIPLIER: f32 = 3.0;
    pub const GRADIENT_G_SIN_FREQUENCY: f32 = 2.1;
    pub const GRADIENT_B_BASE: f32 = 25.0;
    pub const GRADIENT_B_RANGE: f32 = 20.0;
    pub const GRADIENT_B_SIN_MULTIPLIER: f32 = 4.0;
    pub const GRADIENT_B_SIN_FREQUENCY: f32 = 1.7;
    
    // Fabric weave pattern constants
    pub const VERTICAL_WEAVE_LINES: i32 = 15;
    pub const HORIZONTAL_WEAVE_LINES: i32 = 12;
    pub const WEAVE_LINE_VARIATIONS: i32 = 3;
    pub const WEAVE_BASE_ALPHA: i32 = 8;
    pub const WEAVE_ALPHA_STEP: i32 = 3;
}

/// Board background and frame configuration
pub struct BoardConfig;

impl BoardConfig {
    // Board positioning on screen
    pub const OFFSET_X: i32 = 100;
    pub const OFFSET_Y: i32 = 50;

    // Board rendering
    pub const GRADIENT_STEPS: i32 = 25;
    pub const TEXTURE_COUNT: i32 = 120;
    pub const SHADOW_SIZE: i32 = 24;

    // Frame sizes and offsets
    pub const OUTER_FRAME_OFFSET: i32 = 10;
    pub const OUTER_FRAME_SIZE: i32 = 20;
    pub const MIDDLE_FRAME_OFFSET: i32 = 8;
    pub const MIDDLE_FRAME_SIZE: i32 = 16;
    pub const INNER_FRAME_OFFSET: i32 = 6;
    pub const INNER_FRAME_SIZE: i32 = 12;
    pub const HIGHLIGHT_FRAME_OFFSET: i32 = 4;
    pub const HIGHLIGHT_FRAME_SIZE: i32 = 8;
    pub const GRAIN_LINES: i32 = 8;
    pub const GRAIN_SPACING: i32 = 2;

    // Frame colors
    pub const SHADOW_COLOR: Color = Color::new(0, 0, 0, 100);
    pub const OUTER_FRAME_COLOR: Color = Color::new(80, 40, 20, 255);
    pub const MIDDLE_FRAME_COLOR: Color = Color::new(139, 69, 19, 255);
    pub const GRAIN_COLOR: Color = Color::new(110, 55, 15, 100);
    pub const INNER_FRAME_COLOR: Color = Color::new(160, 82, 45, 255);
    pub const HIGHLIGHT_FRAME_COLOR: Color = Color::new(210, 180, 140, 255);
}

/// Info panel configuration
pub struct InfoPanelConfig;

impl InfoPanelConfig {
    // Panel positioning and dimensions
    pub const X: i32 = 700;
    pub const WIDTH: i32 = 520;
}

/// Particle system configuration
pub struct ParticleConfig;

impl ParticleConfig {
    pub const SYSTEM_CAPACITY: usize = 150;
    pub const EXPLOSION_COUNT: usize = 40;
    pub const SPARKLE_COUNT: usize = 8;
    pub const WAVE_SIZE: usize = 12;

    // Explosion speeds for different waves
    pub const EXPLOSION_SPEEDS: [f32; 4] = [80.0, 60.0, 40.0, 100.0];

    // Life times for different particle types
    pub const LIFE_TIMES: [f32; 4] = [1.0, 1.2, 1.5, 0.8];
    pub const LIFE_TIME_VARIATION: f32 = 0.05;

    // Particle sizes
    pub const SIZES: [f32; 4] = [3.0, 2.5, 2.0, 4.0];
    pub const SPARKLE_SIZE: f32 = 1.5;

    // Physics constants
    pub const ACCELERATION_Y: f32 = 200.0;
    pub const VELOCITY_VARIATION_RANGE: f32 = 30.0;
    pub const ANGULAR_VELOCITY_RANGE: f32 = 3.0;

    // Sparkle specific constants
    pub const SPARKLE_LIFE: f32 = 0.6;
    pub const SPARKLE_SPEED: f32 = 20.0;
    pub const SPARKLE_UPWARD_BIAS: f32 = -30.0;
    pub const SPARKLE_ACCELERATION_Y: f32 = 150.0;
    pub const SPARKLE_ANGULAR_VELOCITY_MULTIPLIER: f32 = 2.0;
    pub const SPARKLE_ANGULAR_VELOCITY_OFFSET: f32 = 8.0;

    // Particle colors
    pub const COLORS: [Color; 4] = [
        Color::WHITE,
        Color::YELLOW,
        Color::ORANGE,
        Color::new(200, 200, 200, 255), // Light gray
    ];
    pub const COLOR_YELLOW: Color = Color::YELLOW;
    pub const COLOR_BLACK: Color = Color::new(30, 30, 30, 255);
}

/// Performance optimization constants
pub struct PerformanceConfig;

impl PerformanceConfig {
    pub const TARGET_FPS: u32 = 60;
}

/// FPS counter display configuration
pub struct FPSConfig;

impl FPSConfig {
    // Layout
    pub const PANEL_WIDTH: i32 = 95;
    pub const PANEL_HEIGHT: i32 = 30;
    pub const PANEL_X_OFFSET: i32 = 10;
    pub const PANEL_Y: i32 = 10;
    pub const FONT_SIZE: f32 = 20.0;
    
    // Colors
    pub const GOOD_FPS_COLOR: Color = Color::new(0, 255, 0, 255);      // Green for 55+ FPS
    pub const MEDIUM_FPS_COLOR: Color = Color::new(255, 255, 0, 255);   // Yellow for 30-55 FPS
    pub const POOR_FPS_COLOR: Color = Color::new(255, 0, 0, 255);       // Red for <30 FPS
    pub const BACKGROUND_COLOR: Color = Color::new(0, 0, 0, 150);
    pub const BORDER_COLOR: Color = Color::new(255, 255, 255, 100);
    pub const SHADOW_COLOR: Color = Color::new(0, 0, 0, 150);
    
    // Thresholds
    pub const GOOD_FPS_THRESHOLD: f32 = 55.0;
    pub const MEDIUM_FPS_THRESHOLD: f32 = 30.0;
}

/// Fallback card renderer configuration (when atlas is not available)
pub struct CardRendererConfig;

impl CardRendererConfig {
    // Shadow layers
    pub const SHADOW_LAYER_1_COLOR: Color = Color::new(0, 0, 0, 40);
    pub const SHADOW_LAYER_2_COLOR: Color = Color::new(0, 0, 0, 60);
    pub const SHADOW_LAYER_3_COLOR: Color = Color::new(0, 0, 0, 80);
    
    // Card face colors
    pub const FACE_DARK_COLOR: Color = Color::new(101, 50, 14, 255);
    pub const FACE_MEDIUM_COLOR: Color = Color::new(139, 69, 19, 255);
    pub const FACE_LIGHT_COLOR: Color = Color::new(222, 184, 135, 255);
    
    // Highlight colors
    pub const TOP_HIGHLIGHT_COLOR: Color = Color::new(255, 255, 255, 80);
    pub const LEFT_HIGHLIGHT_COLOR: Color = Color::new(255, 255, 255, 50);
    pub const BORDER_HIGHLIGHT_COLOR: Color = Color::new(255, 255, 255, 30);
    
    // Shadow offsets
    pub const SHADOW_OFFSET_1: i32 = 6;
    pub const SHADOW_OFFSET_2: i32 = 4;
    pub const SHADOW_OFFSET_3: i32 = 2;
    pub const TOP_HIGHLIGHT_HEIGHT: i32 = 3;
    pub const LEFT_HIGHLIGHT_WIDTH: i32 = 2;
    pub const BORDER_THICKNESS: i32 = 2;
}
