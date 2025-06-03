use crate::game::Game;
use crate::ui::animated_background::AnimatedBackground;
use crate::ui::DrawingHelpers;
use crate::ui::particle_system::ParticleSystem;
use raylib::prelude::*;

use super::game_state::GameState;
use super::shared_renderer::{BackgroundRenderer, OverlayState, SharedRenderer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuitConfirm;

impl QuitConfirm {
    fn render_content(
        d: &mut RaylibDrawHandle,
        _game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
    ) {
        // Draw quit confirmation dialog
        SharedRenderer::draw_centered_title(
            d,
            title_font,
            "QUIT GAME?",
            300.0,
            60.0,
            2.5,
            Color::WHITE,
        );

        // Draw conditional quit confirmation based on controller availability
        DrawingHelpers::draw_quit_confirmation(d, font, has_controller);
    }
}

impl OverlayState for QuitConfirm {
    fn render_overlay_content(
        &self,
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
    ) {
        Self::render_content(d, game, has_controller, title_font, font);
    }

    fn get_background_renderer() -> fn(&mut RaylibDrawHandle, &Game, bool, &Font, &Font, &Texture2D, &mut ParticleSystem, &mut AnimatedBackground) {
        BackgroundRenderer::render_start_screen
    }
}

impl GameState for QuitConfirm {
    fn state_name(&self) -> &'static str {
        "QuitConfirm"
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
        self.render_overlay(d, game, has_controller, title_font, font, card_atlas, particle_system, animated_background);
    }
} 