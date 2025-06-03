use crate::game::Game;
use crate::ui::animated_background::AnimatedBackground;
use crate::ui::DrawingHelpers;
use crate::ui::particle_system::ParticleSystem;
use raylib::prelude::*;

use super::game_state::GameState;

// Individual game state implementations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartScreen;

impl GameState for StartScreen {
    fn state_name(&self) -> &'static str {
        "StartScreen"
    }

    fn render(
        &self,
        d: &mut RaylibDrawHandle,
        game: &Game,
        has_controller: bool,
        title_font: &Font,
        font: &Font,
        card_atlas: &Texture2D,
        _particle_system: &mut ParticleSystem,
        animated_background: &mut AnimatedBackground,
    ) {
        // Draw a sophisticated gradient background
        DrawingHelpers::draw_gradient_background(d);

        // Draw animated background cards instead of static ones
        animated_background.draw(d, card_atlas);

        // Main title with shadow effect
        DrawingHelpers::draw_title_with_shadow(d, title_font);

        // Subtitle with elegant styling
        DrawingHelpers::draw_subtitle(d, font);

        // Central game panel with rounded corners and shadow
        DrawingHelpers::draw_main_panel(d);

        // Difficulty selection with nice styling
        DrawingHelpers::draw_difficulty_selector(d, title_font, font, game, has_controller);

        // High scores in a nice panel
        DrawingHelpers::draw_high_scores_panel(d, title_font, font, game);

        // Call-to-action button with glow effect
        DrawingHelpers::draw_start_button(d, title_font, has_controller);
    }
} 