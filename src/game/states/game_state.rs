use crate::game::Game;
use crate::ui::animated_background::AnimatedBackground;
use crate::ui::particle_system::ParticleSystem;
use raylib::prelude::*;
use std::any::Any;

// Trait that all game states must implement
pub trait GameState {
    fn should_update(&self) -> bool {
        false // Default: most states don't update
    }

    fn state_name(&self) -> &'static str;

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
    );

    // Enable downcasting for accessing specific state data
    fn as_any(&self) -> &dyn Any;
}
