use crate::game::Game;
use crate::ui::animated_background::AnimatedBackground;
use crate::ui::config::ScreenConfig;
use crate::ui::particle_system::ParticleSystem;
use raylib::prelude::*;

use super::game_state::GameState;
use super::shared_renderer::{BackgroundRenderer, OverlayState, SharedRenderer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub previous_state_name: String, // Track what state we came from to return properly
    pub selected_option: usize,      // 0: Music, 1: Sound Effects, 2: VSync
}

impl Settings {
    pub fn new(previous_state_name: String) -> Self {
        Self {
            previous_state_name,
            selected_option: 0,
        }
    }

    fn render_content(
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        selected_option: usize,
    ) {
        // Draw settings title
        SharedRenderer::draw_centered_title(
            d,
            title_font,
            "SETTINGS",
            200.0,
            60.0,
            2.5,
            Color::WHITE,
        );

        // Draw settings panel background
        let panel_x = ScreenConfig::WIDTH / 2 - 200;
        let panel_y = 280;
        let panel_width = 400;
        let panel_height = 330; // Increased height for difficulty option

        // Semi-transparent background for settings panel
        d.draw_rectangle(
            panel_x - 10,
            panel_y - 10,
            panel_width + 20,
            panel_height + 20,
            Color::new(0, 0, 0, 150),
        );
        d.draw_rectangle(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            Color::new(40, 40, 60, 200),
        );
        d.draw_rectangle_lines(panel_x, panel_y, panel_width, panel_height, Color::WHITE);

        // Settings options
        let settings = &game.settings;
        let option_y_start = panel_y + 30;
        let option_spacing = 45;
        let label_x = (panel_x + 15) as f32;

        // Selected option is now passed as parameter

        // Music Volume
        let music_text = if settings.music_muted {
            "Music: MUTED".to_string()
        } else {
            format!("Music: {}%", (settings.music_volume * 100.0) as i32)
        };
        let music_color = if selected_option == 0 {
            Color::YELLOW
        } else if settings.music_muted {
            Color::GRAY
        } else {
            Color::WHITE
        };

        // Draw selection indicator for music
        if selected_option == 0 {
            d.draw_rectangle(
                panel_x + 5,
                option_y_start - 8,
                panel_width - 10,
                40,
                Color::new(255, 255, 0, 80),
            );
            d.draw_rectangle_lines(
                panel_x + 5,
                option_y_start - 8,
                panel_width - 10,
                40,
                Color::YELLOW,
            );
        }

        SharedRenderer::draw_text(
            d,
            font,
            &music_text,
            label_x,
            option_y_start as f32,
            24.0,
            1.2,
            music_color,
        );

        // Sound Effects Volume
        let sfx_text = if settings.sound_effects_muted {
            "Sound FX: MUTED".to_string()
        } else {
            format!(
                "Sound FX: {}%",
                (settings.sound_effects_volume * 100.0) as i32
            )
        };
        let sfx_color = if selected_option == 1 {
            Color::YELLOW
        } else if settings.sound_effects_muted {
            Color::GRAY
        } else {
            Color::WHITE
        };

        // Draw selection indicator for sound effects
        if selected_option == 1 {
            d.draw_rectangle(
                panel_x + 5,
                option_y_start + option_spacing - 8,
                panel_width - 10,
                40,
                Color::new(255, 255, 0, 80),
            );
            d.draw_rectangle_lines(
                panel_x + 5,
                option_y_start + option_spacing - 8,
                panel_width - 10,
                40,
                Color::YELLOW,
            );
        }

        SharedRenderer::draw_text(
            d,
            font,
            &sfx_text,
            label_x,
            (option_y_start + option_spacing) as f32,
            24.0,
            1.2,
            sfx_color,
        );

        // VSync
        let vsync_text = if settings.vsync_enabled {
            "VSync: ON"
        } else {
            "VSync: OFF"
        };
        let vsync_color = if selected_option == 2 {
            Color::YELLOW
        } else {
            Color::WHITE
        };

        // Draw selection indicator for vsync
        if selected_option == 2 {
            d.draw_rectangle(
                panel_x + 5,
                option_y_start + option_spacing * 2 - 8,
                panel_width - 10,
                40,
                Color::new(255, 255, 0, 80),
            );
            d.draw_rectangle_lines(
                panel_x + 5,
                option_y_start + option_spacing * 2 - 8,
                panel_width - 10,
                40,
                Color::YELLOW,
            );
        }

        SharedRenderer::draw_text(
            d,
            font,
            vsync_text,
            label_x,
            (option_y_start + option_spacing * 2) as f32,
            24.0,
            1.2,
            vsync_color,
        );

        // Difficulty
        let difficulty_text = match settings.difficulty {
            crate::models::Difficulty::Easy => "Difficulty: Easy",
            crate::models::Difficulty::Hard => "Difficulty: Hard",
        };
        let difficulty_color = if selected_option == 3 {
            Color::YELLOW
        } else {
            Color::WHITE
        };

        // Draw selection indicator for difficulty
        if selected_option == 3 {
            d.draw_rectangle(
                panel_x + 5,
                option_y_start + option_spacing * 3 - 8,
                panel_width - 10,
                40,
                Color::new(255, 255, 0, 80),
            );
            d.draw_rectangle_lines(
                panel_x + 5,
                option_y_start + option_spacing * 3 - 8,
                panel_width - 10,
                40,
                Color::YELLOW,
            );
        }

        SharedRenderer::draw_text(
            d,
            font,
            difficulty_text,
            label_x,
            (option_y_start + option_spacing * 3) as f32,
            24.0,
            1.2,
            difficulty_color,
        );

        // Volume sliders (visual representation)
        Self::draw_volume_slider(
            d,
            panel_x + 280,
            option_y_start,
            settings.music_volume,
            settings.music_muted,
        );
        Self::draw_volume_slider(
            d,
            panel_x + 280,
            option_y_start + option_spacing,
            settings.sound_effects_volume,
            settings.sound_effects_muted,
        );

        // Instructions
        Self::draw_settings_instructions(d, font, has_controller, panel_y + panel_height + 30);
    }

    fn draw_volume_slider(d: &mut RaylibDrawHandle, x: i32, y: i32, volume: f32, muted: bool) {
        let slider_width = 80;
        let slider_height = 8;
        let fill_width = if muted {
            0
        } else {
            (slider_width as f32 * volume) as i32
        };

        // Background
        d.draw_rectangle(x, y + 8, slider_width, slider_height, Color::DARKGRAY);

        // Fill
        if !muted && fill_width > 0 {
            d.draw_rectangle(x, y + 8, fill_width, slider_height, Color::GREEN);
        }

        // Border
        d.draw_rectangle_lines(x, y + 8, slider_width, slider_height, Color::WHITE);
    }

    fn draw_settings_instructions(
        d: &mut RaylibDrawHandle,
        font: &Font,
        has_controller: bool,
        y: i32,
    ) {
        let instruction_text = if has_controller {
            "D-Pad Up/Down: Navigate  |  Left/Right: Adjust/Change  |  A: Toggle  |  B: Back"
        } else {
            "Up/Down: Navigate  |  Left/Right: Adjust/Change  |  Space: Toggle  |  ESC: Back"
        };

        // Center the instruction text
        let text_width = d.measure_text(instruction_text, 18i32);
        let text_x = (ScreenConfig::WIDTH - text_width) / 2;

        SharedRenderer::draw_text(
            d,
            font,
            instruction_text,
            text_x as f32,
            y as f32,
            22.0,
            1.0,
            Color::LIGHTGRAY,
        );
    }
}

impl OverlayState for Settings {
    fn render_overlay_content(
        &self,
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
    ) {
        Self::render_content(
            d,
            game,
            has_controller,
            title_font,
            font,
            game.settings.selected_option,
        );
    }

    fn get_background_renderer() -> fn(
        &mut RaylibDrawHandle,
        &Game,
        bool,
        &Font,
        &Font,
        &Texture2D,
        &mut ParticleSystem,
        &mut AnimatedBackground,
    ) {
        // Use start screen background since settings can be accessed from multiple places
        // This provides a neutral, pleasant background for the settings overlay
        BackgroundRenderer::render_start_screen
    }
}

impl GameState for Settings {
    fn state_name(&self) -> &'static str {
        "Settings"
    }

    fn render(
        &self,
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        particle_system: &mut ParticleSystem,
        animated_background: &mut AnimatedBackground,
    ) {
        self.render_overlay(
            d,
            game,
            has_controller,
            title_font,
            font,
            card_atlas,
            particle_system,
            animated_background,
        );
    }
}
