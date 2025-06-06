//! UI Module
//!
//! This module handles all user interface rendering and interaction for the DropJack game.
//! It provides a clean abstraction over raylib graphics operations and maintains a structured
//! configuration system for consistent UI styling and behavior.
//!
//! Key components:
//! - Configuration system with organized constants for each UI component
//! - Particle system for visual effects
//! - Input handling with both keyboard and controller support
//! - Modular rendering components for different game states
//! - Performance monitoring with FPS counter

// Sub-modules
pub mod animated_background;
mod atlas_card_renderer;
mod background_renderer;
mod card_renderer;
pub mod config;
mod drawing_helpers;
pub mod input_handler;
mod instruction_renderer;
mod menu_renderer;
pub mod particle_system;
mod text_renderer;

// Re-export for easy access
pub use drawing_helpers::DrawingHelpers;

use self::animated_background::AnimatedBackground;
use self::config::{BoardConfig, FPSConfig, ParticleConfig, PerformanceConfig, ScreenConfig};
// Board offset constants are now in ScreenConfig
use self::input_handler::InputHandler;
use self::particle_system::ParticleSystem;
use crate::audio::AudioSystem;
use crate::game::Game;
use raylib::prelude::*;

/// Font collection for different size ranges
#[derive(Debug)]
pub struct FontCollection {
    /// Small text (12-24px) - loaded at base size 24
    small: Font,
    /// Medium text (24-48px) - loaded at base size 48  
    medium: Font,
    /// Large text (48-96px) - loaded at base size 96
    large: Font,
    /// Extra large text (96px+) - loaded at base size 120
    extra_large: Font,
}

impl FontCollection {
    /// Create a new font collection from a single font file
    fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        font_path: &str,
        description: &str,
    ) -> Self {
        println!(
            "Loading optimized font collection for {}: {}",
            description, font_path
        );

        // Load fonts at their optimal base sizes using LoadFontEx for crystal clear rendering
        let small = Self::load_font_ex(
            rl,
            thread,
            font_path,
            24,
            &format!("{} (small)", description),
        );
        let medium = Self::load_font_ex(
            rl,
            thread,
            font_path,
            48,
            &format!("{} (medium)", description),
        );
        let large = Self::load_font_ex(
            rl,
            thread,
            font_path,
            96,
            &format!("{} (large)", description),
        );
        // For title font, load at 120px which is the exact size used (TextConfig::TITLE_SIZE)
        let extra_large = Self::load_font_ex(
            rl,
            thread,
            font_path,
            120,
            &format!("{} (extra large)", description),
        );

        FontCollection {
            small,
            medium,
            large,
            extra_large,
        }
    }

    /// Load a font at a specific base size using LoadFontEx for optimal quality
    fn load_font_ex(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        base_size: i32,
        description: &str,
    ) -> Font {
        use std::ffi::CString;
        use std::ptr;

        // Convert path to C string
        let c_path = CString::new(path).expect("Failed to create CString for font path");

        // Use raylib's LoadFontEx to load font at exact base size
        let raylib_font =
            unsafe { raylib::ffi::LoadFontEx(c_path.as_ptr(), base_size, ptr::null_mut(), 0) };

        // Check if font loaded successfully
        if raylib_font.texture.id == 0 {
            eprintln!(
                "Warning: Failed to load font {} with LoadFontEx, falling back to default loading",
                path
            );
            return Self::load_font_fallback(rl, thread, path, description);
        }

        // Convert raylib font to raylib-rs Font
        let font = unsafe { Font::from_raw(raylib_font) };

        // Apply texture filtering for even smoother rendering
        Self::apply_font_filtering(&font);

        println!(
            "  ✓ Loaded {} at exact size {}px using LoadFontEx",
            description, base_size
        );
        font
    }

    /// Fallback font loading method if LoadFontEx fails
    fn load_font_fallback(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        description: &str,
    ) -> Font {
        let font = rl.load_font(thread, path).unwrap_or_else(|e| {
            panic!(
                "Critical error: Could not load font {} for {}: {:?}",
                path, description, e
            );
        });

        Self::apply_font_filtering(&font);
        font
    }

    /// Apply texture filtering to font for smoother rendering
    fn apply_font_filtering(font: &Font) {
        // Apply bilinear filtering to the font texture for smoother scaling
        unsafe {
            use raylib::ffi::{SetTextureFilter, TextureFilter};
            let texture_id = raylib::ffi::Texture2D {
                id: font.texture().id,
                width: font.texture().width,
                height: font.texture().height,
                mipmaps: font.texture().mipmaps,
                format: font.texture().format as i32,
            };
            SetTextureFilter(texture_id, TextureFilter::TEXTURE_FILTER_BILINEAR as i32);
        }
    }

    /// Get the most appropriate font for a given text size
    pub fn get_font_for_size(&self, size: f32) -> &Font {
        match size {
            s if s <= 24.0 => &self.small,
            s if s <= 48.0 => &self.medium,
            s if s <= 96.0 => &self.large,
            _ => &self.extra_large, // This will be perfect for 120px title text
        }
    }

    /// Get the default/medium font for backward compatibility
    pub fn default(&self) -> &Font {
        &self.medium
    }
}

pub struct GameUI {
    rl: RaylibHandle,
    thread: RaylibThread,
    // Enhanced font system with multiple sizes for optimal rendering
    default_fonts: FontCollection,
    title_fonts: FontCollection,
    card_atlas: Option<Texture2D>,
    particle_system: ParticleSystem,
    input_handler: InputHandler,
    last_frame_time: std::time::Instant,
    fps_counter: FPSCounter,
    animated_background: AnimatedBackground,
    audio_system: AudioSystem,
}

struct FPSCounter {
    current_fps: f32,
    last_update: std::time::Instant,
}

impl FPSCounter {
    fn new() -> Self {
        FPSCounter {
            current_fps: 60.0,
            last_update: std::time::Instant::now(),
        }
    }

    fn update(&mut self, delta_time: f32) {
        // Update FPS calculation every 100ms for stable display
        let now = std::time::Instant::now();
        if now.duration_since(self.last_update).as_millis() > 100 {
            self.current_fps = if delta_time > 0.0 {
                1.0 / delta_time
            } else {
                0.0
            };
            self.last_update = now;
        }
    }

    fn get_fps(&self) -> f32 {
        self.current_fps
    }
}

impl GameUI {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(ScreenConfig::WIDTH, ScreenConfig::HEIGHT)
            .title("DropJack")
            .build();

        rl.set_target_fps(PerformanceConfig::TARGET_FPS);
        rl.set_exit_key(None); // Disable ESC from closing the window

        // Load enhanced font collections with multiple sizes for optimal rendering
        println!("Initializing enhanced font system...");
        let default_fonts =
            FontCollection::new(&mut rl, &thread, "assets/fonts/default.ttf", "default");
        let title_fonts = FontCollection::new(&mut rl, &thread, "assets/fonts/title.ttf", "title");
        println!("✓ Font system initialized with bilinear filtering");

        // Load the card atlas
        let card_atlas = rl.load_texture(&thread, "assets/cards/atlas.png").ok();
        if card_atlas.is_none() {
            eprintln!(
                "Warning: Could not load card atlas assets/cards/atlas.png, using fallback rendering"
            );
        }

        // Initialize audio system
        let audio_system = AudioSystem::new();

        // Print audio status for debugging/information
        audio_system.print_audio_status();

        GameUI {
            rl,
            thread,
            default_fonts,
            title_fonts,
            card_atlas,
            particle_system: ParticleSystem::builder()
                .particle_capacity(ParticleConfig::SYSTEM_CAPACITY)
                .explosion_particle_count(ParticleConfig::EXPLOSION_COUNT)
                .build(),
            input_handler: InputHandler::new(),
            last_frame_time: std::time::Instant::now(),
            fps_counter: FPSCounter::new(),
            animated_background: AnimatedBackground::new(),
            audio_system,
        }
    }

    /// Get the optimal font for a given text size (default font family)
    pub fn get_font(&self, size: f32) -> &Font {
        self.default_fonts.get_font_for_size(size)
    }

    /// Get the optimal title font for a given text size
    pub fn get_title_font(&self, size: f32) -> &Font {
        self.title_fonts.get_font_for_size(size)
    }

    /// Get the default font (for backward compatibility)
    pub fn font(&self) -> &Font {
        self.default_fonts.default()
    }

    /// Get the title font (for backward compatibility)
    pub fn title_font(&self) -> &Font {
        self.title_fonts.default()
    }

    pub fn run(&mut self, game: &mut Game) {
        while !self.rl.window_should_close() {
            self.update_frame(game);
            self.render_frame(game);
        }
    }

    /// Separated update logic for better organization
    fn update_frame(&mut self, game: &mut Game) {
        // Calculate delta time
        let now = std::time::Instant::now();
        let delta_time = now.duration_since(self.last_frame_time).as_secs_f32();
        self.last_frame_time = now;

        // Update FPS counter
        self.fps_counter.update(delta_time);

        // Handle input
        self.input_handler.handle_input(&mut self.rl, game);

        // Apply VSync setting if it changed
        self.apply_vsync_setting(game);

        // Apply music settings
        self.apply_music_settings(game);

        // Update game state (only when not paused and not in settings)
        if !game.is_paused() && !game.is_settings() {
            game.update();
        }

        // Update animated background for title and quit screens
        if game.is_start_screen() || game.is_quit_confirm() {
            self.animated_background.update(delta_time);
        }

        // Process explosions
        self.process_explosions(game);

        // Process audio events
        self.process_audio_events(game);

        // Update particle system
        self.particle_system.update(delta_time);
    }

    /// Separated render logic for better organization
    fn render_frame(&mut self, game: &Game) {
        let has_controller = InputHandler::is_controller_connected(&self.rl);

        let mut d = self.rl.begin_drawing(&self.thread);

        // Use elegant gradient background instead of flat DARKGREEN
        DrawingHelpers::draw_gradient_background(&mut d);

        // Render game state with optimized font selection
        // Use the extra large title font (120px) for crystal clear title rendering
        game.state.render(
            &mut d,
            game,
            has_controller,
            &self.title_fonts.extra_large, // Use 120px font for title
            &self.default_fonts.medium,    // Use 48px font for default text
            self.card_atlas
                .as_ref()
                .expect("Card atlas must be loaded!"),
            &mut self.particle_system,
            &mut self.animated_background,
        );

        // Render FPS counter with small font (20px) using 24px base
        Self::render_fps_counter_static(
            &mut d,
            &self.default_fonts.small,
            self.fps_counter.get_fps(),
        );
    }

    /// Renders FPS counter with improved styling (static method to avoid borrowing issues)
    fn render_fps_counter_static(d: &mut RaylibDrawHandle, font: &Font, fps: f32) {
        let fps_panel_x = ScreenConfig::WIDTH - FPSConfig::PANEL_WIDTH - FPSConfig::PANEL_X_OFFSET;
        let fps_text = format!("FPS: {:.1}", fps);

        // Choose color based on FPS performance using configuration
        let fps_color = match fps {
            f if f >= FPSConfig::GOOD_FPS_THRESHOLD => FPSConfig::GOOD_FPS_COLOR,
            f if f >= FPSConfig::MEDIUM_FPS_THRESHOLD => FPSConfig::MEDIUM_FPS_COLOR,
            _ => FPSConfig::POOR_FPS_COLOR,
        };

        // Draw background panel for better visibility
        d.draw_rectangle(
            fps_panel_x - 10,
            FPSConfig::PANEL_Y - 5,
            FPSConfig::PANEL_WIDTH,
            FPSConfig::PANEL_HEIGHT,
            FPSConfig::BACKGROUND_COLOR,
        );

        // Draw border
        d.draw_rectangle_lines(
            fps_panel_x - 10,
            FPSConfig::PANEL_Y - 5,
            FPSConfig::PANEL_WIDTH,
            FPSConfig::PANEL_HEIGHT,
            FPSConfig::BORDER_COLOR,
        );

        // Draw shadow
        d.draw_text_ex(
            font,
            &fps_text,
            Vector2::new((fps_panel_x + 1) as f32, (FPSConfig::PANEL_Y + 1) as f32),
            FPSConfig::FONT_SIZE,
            1.0,
            FPSConfig::SHADOW_COLOR,
        );

        // Draw main text
        d.draw_text_ex(
            font,
            &fps_text,
            Vector2::new(fps_panel_x as f32, FPSConfig::PANEL_Y as f32),
            FPSConfig::FONT_SIZE,
            1.0,
            fps_color,
        );
    }

    /// Process game explosions and create particle effects
    fn process_explosions(&mut self, game: &mut Game) {
        let explosions = game.take_pending_explosions();
        for (x, y, card) in explosions {
            let position = Vector2::new(
                (BoardConfig::OFFSET_X + x * game.board.cell_size + game.board.cell_size / 2)
                    as f32,
                (BoardConfig::OFFSET_Y + y * game.board.cell_size + game.board.cell_size / 2)
                    as f32,
            );

            self.particle_system.add_card_explosion(
                card,
                position,
                game.board.cell_size as f32,
                &self.card_atlas,
            );
        }
    }

    /// Process audio events from the game
    fn process_audio_events(&mut self, game: &mut Game) {
        let audio_events = game.take_pending_audio_events();
        for event in audio_events {
            // Play the appropriate sound for each specific event with volume settings
            let settings = &game.settings;
            self.audio_system.play_event(
                event,
                settings.sound_effects_volume,
                settings.sound_effects_muted,
                &mut self.rl,
            );
        }
    }

    /// Apply VSync setting changes
    fn apply_vsync_setting(&mut self, game: &Game) {
        // Note: Raylib doesn't provide runtime VSync control, so we'll just track the setting
        // In a real implementation, this might require recreation of the window or other measures
        // For now, we'll just acknowledge the setting exists and could be applied on restart
        if game.settings.vsync_enabled {
            // VSync would be enabled - in practice this might require window recreation
        } else {
            // VSync would be disabled
        }
    }

    /// Apply music settings changes
    fn apply_music_settings(&mut self, game: &Game) {
        let settings = &game.settings;

        if settings.music_muted {
            self.audio_system.stop_music();
        } else {
            self.audio_system.set_music_volume(settings.music_volume);
            if !self.audio_system.is_music_playing() {
                self.audio_system.start_music(settings.music_volume, false);
            }
        }
    }
}
