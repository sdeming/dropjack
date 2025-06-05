use crate::game::Game;
use crate::ui::DrawingHelpers;
use crate::ui::animated_background::AnimatedBackground;
use crate::ui::config::ScreenConfig;
use crate::ui::particle_system::ParticleSystem;
use raylib::prelude::*;

use super::game_state::GameState;
use super::shared_renderer::{BackgroundRenderer, OverlayState, SharedRenderer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameOver;

impl GameOver {
    fn render_content(
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
    ) {
        // Draw game over text using title font - centered
        SharedRenderer::draw_centered_title(
            d,
            title_font,
            "GAME OVER",
            250.0,
            60.0,
            2.5,
            Color::WHITE,
        );

        // Draw final score
        let score_text = format!("Final Score: {}", game.score);
        SharedRenderer::draw_text(d, font, &score_text, 530.0, 330.0, 36.0, 1.5, Color::WHITE);

        // Draw initials input heading using title font
        SharedRenderer::draw_text(
            d,
            title_font,
            "Enter your initials:",
            520.0,
            390.0,
            32.0,
            1.25,
            Color::WHITE,
        );

        // Draw initials box
        let box_width = 200;
        let box_height = 60;
        let box_x = ScreenConfig::WIDTH / 2 - box_width / 2;
        let box_y = 440;

        SharedRenderer::draw_input_box(
            d,
            box_x,
            box_y,
            box_width,
            box_height,
            Color::DARKGRAY,
            Color::WHITE,
        );

        // Draw entered initials
        let initials_text = if game.player_initials.is_empty() {
            "___".to_string()
        } else {
            format!("{:_<3}", game.player_initials)
        };

        SharedRenderer::draw_text(
            d,
            font,
            &initials_text,
            (box_x + 65) as f32,
            (box_y + 15) as f32,
            36.0,
            1.5,
            Color::WHITE,
        );

        // Draw conditional instructions based on controller availability
        DrawingHelpers::draw_game_over_instructions(d, font, has_controller);
    }
}

impl OverlayState for GameOver {
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
        BackgroundRenderer::render_game_view
    }
}

impl GameState for GameOver {
    fn state_name(&self) -> &'static str {
        "GameOver"
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
