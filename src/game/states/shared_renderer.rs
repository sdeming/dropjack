use crate::game::Game;
use crate::ui::animated_background::AnimatedBackground;
use crate::ui::config::ScreenConfig;
use crate::ui::particle_system::ParticleSystem;
use raylib::prelude::*;

use super::game_state::GameState;

// Shared rendering functionality
pub struct SharedRenderer;

impl SharedRenderer {
    /// Draw a semi-transparent overlay covering the entire screen
    pub fn draw_overlay(d: &mut RaylibDrawHandle, alpha: u8) {
        d.draw_rectangle(
            0,
            0,
            ScreenConfig::WIDTH,
            ScreenConfig::HEIGHT,
            Color::new(0, 0, 0, alpha),
        );
    }

    /// Draw centered text with consistent styling
    pub fn draw_centered_title(
        d: &mut RaylibDrawHandle,
        font: &Font,
        text: &str,
        y: f32,
        size: f32,
        spacing: f32,
        color: Color,
    ) {
        // Manual centering based on approximate character width
        let approx_char_width = size * 0.6; // Approximation for most fonts
        let text_width = text.len() as f32 * approx_char_width;
        let x = (ScreenConfig::WIDTH as f32 - text_width) / 2.0;

        d.draw_text_ex(font, text, Vector2::new(x, y), size, spacing, color);
    }

    /// Draw text with consistent positioning (not centered)
    pub fn draw_text(
        d: &mut RaylibDrawHandle,
        font: &Font,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        spacing: f32,
        color: Color,
    ) {
        d.draw_text_ex(font, text, Vector2::new(x, y), size, spacing, color);
    }

    /// Draw a styled input box for text entry
    pub fn draw_input_box(
        d: &mut RaylibDrawHandle,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        bg_color: Color,
        border_color: Color,
    ) {
        d.draw_rectangle(x, y, width, height, bg_color);
        d.draw_rectangle_lines(x, y, width, height, border_color);
    }

    /// Generic overlay renderer for states that need background + overlay
    pub fn render_with_overlay<F, C>(
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        particle_system: &mut ParticleSystem,
        animated_background: &mut AnimatedBackground,
        render_background: F,
        overlay_alpha: u8,
        render_content: C,
    ) where
        F: FnOnce(
            &mut RaylibDrawHandle,
            &Game,
            bool,
            &Font,
            &Font,
            &Texture2D,
            &mut ParticleSystem,
            &mut AnimatedBackground,
        ),
        C: FnOnce(&mut RaylibDrawHandle, &Game, bool, &Font, &Font),
    {
        // Render background
        render_background(
            d,
            game,
            has_controller,
            title_font,
            font,
            card_atlas,
            particle_system,
            animated_background,
        );

        // Draw overlay
        Self::draw_overlay(d, overlay_alpha);

        // Render content
        render_content(d, game, has_controller, title_font, font);
    }
}

// Helper functions for common background rendering
pub struct BackgroundRenderer;

impl BackgroundRenderer {
    pub fn render_game_view(
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        particle_system: &mut ParticleSystem,
        _animated_background: &mut AnimatedBackground,
    ) {
        use super::playing::Playing;
        Playing::draw_game_view(
            d,
            game,
            has_controller,
            title_font,
            font,
            card_atlas,
            particle_system,
            false,
        );
    }

    pub fn render_start_screen(
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        particle_system: &mut ParticleSystem,
        animated_background: &mut AnimatedBackground,
    ) {
        use super::start_screen::StartScreen;
        let start_screen = StartScreen;
        start_screen.render(
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

// Trait for states that render as overlays over a background
pub trait OverlayState {
    /// Render the content specific to this overlay state
    fn render_overlay_content(
        &self,
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
    );

    /// Get the background renderer function for this state
    fn get_background_renderer() -> fn(
        &mut RaylibDrawHandle,
        &Game,
        bool,
        &Font,
        &Font,
        &Texture2D,
        &mut ParticleSystem,
        &mut AnimatedBackground,
    );

    /// Get the overlay alpha value (default 200)
    fn get_overlay_alpha(&self) -> u8 {
        200
    }

    /// Default implementation for overlay rendering pattern
    fn render_overlay(
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
        SharedRenderer::render_with_overlay(
            d,
            game,
            has_controller,
            title_font,
            font,
            card_atlas,
            particle_system,
            animated_background,
            Self::get_background_renderer(),
            self.get_overlay_alpha(),
            |d, game, has_controller, title_font, font| {
                self.render_overlay_content(d, game, has_controller, title_font, font)
            },
        );
    }
}
