// Sub-modules
mod atlas_card_renderer;
pub mod drawing;
mod drawing_helpers;
mod card_renderer;
mod background_renderer;
mod text_renderer;
mod menu_renderer;
mod instruction_renderer;
pub mod input_handler;
pub mod particle_system;
pub mod animated_background;

// Re-export for easy access
pub use drawing_helpers::DrawingHelpers;

use self::animated_background::AnimatedBackground;
use self::drawing::{BOARD_OFFSET_X, BOARD_OFFSET_Y, SCREEN_HEIGHT, SCREEN_WIDTH};
use self::input_handler::InputHandler;
use self::particle_system::ParticleSystem;
use crate::game::Game;
use crate::audio::AudioSystem;
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
            self.current_fps = if delta_time > 0.0 { 1.0 / delta_time } else { 0.0 };
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

        // Initialize audio system
        let audio_system = AudioSystem::new(&mut rl, &thread);

        GameUI {
            rl,
            thread,
            font,
            title_font,
            card_atlas,
            particle_system: ParticleSystem::new(),
            input_handler: InputHandler::new(),
            last_frame_time: std::time::Instant::now(),
            fps_counter: FPSCounter::new(),
            animated_background: AnimatedBackground::new(),
            audio_system,
        }
    }

    pub fn run(&mut self, game: &mut Game) {
        while !self.rl.window_should_close() {
            // Calculate delta time
            let now = std::time::Instant::now();
            let delta_time = now.duration_since(self.last_frame_time).as_secs_f32();
            self.last_frame_time = now;

            // Update FPS counter
            self.fps_counter.update(delta_time);

            // Detect controller availability
            let has_controller = InputHandler::is_controller_connected(&self.rl);

            // Handle input
            self.input_handler.handle_input(&mut self.rl, game);

            // Update game state (only when not paused)
            if !game.is_paused() {
                game.update();
            }

            // Update animated background for title and quit screens
            if game.is_start_screen() || game.is_quit_confirm() {
                self.animated_background.update(delta_time);
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

            // Process audio events
            let audio_events = game.take_pending_audio_events();
            for _event in audio_events {
                // For now, play the click sound for all events
                self.audio_system.play_click(&mut self.rl);
            }

            // Update particle system
            self.particle_system.update(delta_time);

            // Render the game
            {
                let mut d = self.rl.begin_drawing(&self.thread);
                // Use elegant gradient background instead of flat DARKGREEN
                DrawingHelpers::draw_gradient_background(&mut d);

                game.state.render(
                    &mut d,
                    game,
                    has_controller,
                    &self.title_font,
                    &self.font,
                    self.card_atlas.as_ref().expect("Card atlas must be loaded!"),
                    &mut self.particle_system,
                    &mut self.animated_background,
                );

                // Draw FPS counter inline
                let fps = self.fps_counter.get_fps();
                let fps_text = format!("FPS: {:.1}", fps);
                
                // Position in top-right corner
                let text_x = SCREEN_WIDTH - 100;
                let text_y = 10;
                let font_size = 20.0;
                
                // Choose color based on FPS performance
                let fps_color = if fps >= 55.0 {
                    Color::new(0, 255, 0, 255)   // Green for good FPS
                } else if fps >= 30.0 {
                    Color::new(255, 255, 0, 255) // Yellow for medium FPS
                } else {
                    Color::new(255, 0, 0, 255)   // Red for poor FPS
                };
                
                // Draw background panel for better visibility
                d.draw_rectangle(
                    text_x - 10,
                    text_y - 5,
                    95,
                    30,
                    Color::new(0, 0, 0, 150),
                );
                
                // Draw border
                d.draw_rectangle_lines(
                    text_x - 10,
                    text_y - 5,
                    95,
                    30,
                    Color::new(255, 255, 255, 100),
                );
                
                // Draw shadow
                d.draw_text_ex(
                    &self.font,
                    &fps_text,
                    Vector2::new((text_x + 1) as f32, (text_y + 1) as f32),
                    font_size,
                    1.0,
                    Color::new(0, 0, 0, 150),
                );
                
                // Draw main text
                d.draw_text_ex(
                    &self.font,
                    &fps_text,
                    Vector2::new(text_x as f32, text_y as f32),
                    font_size,
                    1.0,
                    fps_color,
                );
            }
        }
    }
}
