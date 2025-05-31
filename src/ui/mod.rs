// Sub-modules
mod atlas_card_renderer;
pub mod drawing;
mod drawing_helpers;
pub mod input_handler;
pub mod particle;
pub mod particle_system;

use self::drawing::{BOARD_OFFSET_X, BOARD_OFFSET_Y, SCREEN_HEIGHT, SCREEN_WIDTH};
use self::input_handler::InputHandler;
use self::particle_system::ParticleSystem;
use crate::game::Game;
use raylib::prelude::*;

pub struct GameUI {
    rl: RaylibHandle,
    thread: RaylibThread,
    font: Font,
    title_font: Font,
    card_atlas: Option<Texture2D>,
    particle_system: ParticleSystem,
    input_handler: InputHandler,
    last_frame_time: std::time::Instant,
}

impl GameUI {
    pub fn new() -> Self {
        let (mut rl, thread) = raylib::init()
            .size(SCREEN_WIDTH, SCREEN_HEIGHT)
            .title("DropJack")
            .build();

        rl.set_target_fps(60);
        rl.set_exit_key(None); // Disable ESC from closing the window

        // Load the custom fonts
        let font = rl
            .load_font(&thread, "assets/fonts/default.ttf")
            .expect("Warning: Could not load font assets/fonts/default.ttf, using default font");

        let title_font = rl.load_font(&thread, "assets/fonts/title.ttf").expect(
            "Warning: Could not load font assets/fonts/title.ttf, using default font for titles",
        );

        // Load the card atlas
        let card_atlas = rl.load_texture(&thread, "assets/cards/atlas.png").ok();
        if card_atlas.is_none() {
            eprintln!(
                "Warning: Could not load card atlas assets/cards/atlas.png, using fallback rendering"
            );
        }

        GameUI {
            rl,
            thread,
            font,
            title_font,
            card_atlas,
            particle_system: ParticleSystem::new(),
            input_handler: InputHandler::new(),
            last_frame_time: std::time::Instant::now(),
        }
    }

    pub fn run(&mut self, game: &mut Game) {
        while !self.rl.window_should_close() {
            // Calculate delta time
            let now = std::time::Instant::now();
            let delta_time = now.duration_since(self.last_frame_time).as_secs_f32();
            self.last_frame_time = now;

            // Detect controller availability
            let has_controller = InputHandler::is_controller_connected(&self.rl);

            // Handle input
            self.input_handler.handle_input(&mut self.rl, game);

            // Update game state (only when not paused)
            if !game.is_paused() {
                game.update();
            }

            // Check for explosions and trigger them
            let explosions = game.take_pending_explosions();
            for (x, y, card) in explosions {
                let position = Vector2::new(
                    (BOARD_OFFSET_X + x * game.board.cell_size + game.board.cell_size / 2) as f32,
                    (BOARD_OFFSET_Y + y * game.board.cell_size + game.board.cell_size / 2) as f32,
                );

                self.particle_system.add_card_explosion(
                    card,
                    position,
                    game.board.cell_size as f32,
                    &self.card_atlas,
                );
            }

            // Update particle system
            self.particle_system.update(delta_time);

            // Render the game
            {
                let mut d = self.rl.begin_drawing(&self.thread);
                d.clear_background(Color::DARKGREEN);

                game.state.render(
                    &mut d,
                    game,
                    has_controller,
                    &self.title_font,
                    &self.font,
                    &self.card_atlas,
                    &mut self.particle_system,
                );
            }
        }
    }
}
