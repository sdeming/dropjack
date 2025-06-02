use crate::game::{Difficulty, Game};
use raylib::prelude::*;

pub struct InputHandler {
    last_move_time: std::time::Instant,
    move_delay: std::time::Duration,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            last_move_time: std::time::Instant::now(),
            move_delay: std::time::Duration::from_millis(150), // 150ms delay between moves
        }
    }

    pub fn is_controller_connected(rl: &RaylibHandle) -> bool {
        rl.is_gamepad_available(0)
    }

    pub fn handle_input(&mut self, rl: &mut RaylibHandle, game: &mut Game) {
        let has_controller = Self::is_controller_connected(rl);

        if game.is_start_screen() {
            self.handle_start_screen_input(rl, game, has_controller);
        } else if game.is_playing() {
            self.handle_playing_input(rl, game, has_controller);
        } else if game.is_paused() {
            self.handle_paused_input(rl, game, has_controller);
        } else if game.is_game_over() {
            self.handle_game_over_input(rl, game, has_controller);
        } else if game.is_quit_confirm() {
            self.handle_quit_confirm_input(rl, game, has_controller);
        }
    }

    fn handle_start_screen_input(
        &self,
        rl: &mut RaylibHandle,
        game: &mut Game,
        has_controller: bool,
    ) {
        // Handle difficulty selection
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) || rl.is_key_pressed(KeyboardKey::KEY_RIGHT)
            || (has_controller
                && (rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT)
                    || rl.is_gamepad_button_pressed(
                        0,
                        GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT,
                    )))
        {
            game.difficulty = match game.difficulty {
                Difficulty::Hard => Difficulty::Easy,
                Difficulty::Easy => Difficulty::Hard,
            };
        }

        // Handle quit confirmation
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT))
        {
            game.transition_to_quit_confirm();
        }

        // Start game
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            || (has_controller
                && (rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT)))
        {
            game.start_game(game.difficulty);
        }
    }

    fn handle_playing_input(
        &mut self,
        rl: &mut RaylibHandle,
        game: &mut Game,
        has_controller: bool,
    ) {
        let now = std::time::Instant::now();
        let can_move = now.duration_since(self.last_move_time) >= self.move_delay;

        // Handle movement (left/right)
        if can_move {
            if rl.is_key_down(KeyboardKey::KEY_LEFT)
                || (has_controller
                    && rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X) < -0.3)
                || (has_controller
                    && rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT))
            {
                game.move_current_card_left();
                self.last_move_time = now;
            } else if rl.is_key_down(KeyboardKey::KEY_RIGHT)
                || (has_controller
                    && rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X) > 0.3)
                || (has_controller
                    && rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT))
            {
                game.move_current_card_right();
                self.last_move_time = now;
            }
        }

        // Handle soft drop (down key)
        if rl.is_key_down(KeyboardKey::KEY_DOWN)
            || (has_controller
                && rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_Y) > 0.3)
            || (has_controller
                && rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN))
        {
            game.move_current_card_down();
        }

        // Handle hard drop (space key)
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN))
        {
            game.hard_drop();
        }

        // Handle pause
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT))
        {
            game.transition_to_paused();
        }
    }

    fn handle_paused_input(&self, rl: &mut RaylibHandle, game: &mut Game, has_controller: bool) {
        // Resume game
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
            || rl.is_key_pressed(KeyboardKey::KEY_N)
            || (has_controller
                && (rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT)
                    || rl.is_gamepad_button_pressed(
                        0,
                        GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,
                    )))
        {
            game.transition_to_playing();
        }

        // Quit to menu
        if rl.is_key_pressed(KeyboardKey::KEY_Y)
            || (has_controller
                && (rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN)))
        {
            game.transition_to_start_screen();
        }
    }

    fn handle_game_over_input(&self, rl: &mut RaylibHandle, game: &mut Game, has_controller: bool) {
        // Handle initial input
        if let Some(key_pressed) = rl.get_key_pressed() {
            if let Some(c) = Self::key_to_char(key_pressed) {
                game.add_initial(c);
            }
        }

        // Handle controller input for initials
        if has_controller {
            if rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT)
                || rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP)
            {
                Self::add_next_letter(game);
            }

            if rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT)
                || rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN)
            {
                Self::add_prev_letter(game);
            }

            if rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN) {
                game.add_initial(game.player_initials.chars().last().unwrap_or('@'));
            }
        }

        // Handle backspace
        if rl.is_key_pressed(KeyboardKey::KEY_BACKSPACE)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT))
        {
            game.remove_initial();
        }

        // Submit and return to menu
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT))
        {
            if !game.player_initials.is_empty() {
                game.save_high_score();
            }
            game.transition_to_start_screen();
        }
    }

    fn handle_quit_confirm_input(
        &self,
        rl: &mut RaylibHandle,
        game: &mut Game,
        has_controller: bool,
    ) {
        // Cancel quit (go back to start screen)
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
            || rl.is_key_pressed(KeyboardKey::KEY_N)
            || (has_controller
                && (rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT)
                    || rl.is_gamepad_button_pressed(
                        0,
                        GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT,
                    )))
        {
            game.transition_to_start_screen();
        }

        // Confirm quit - actually exit the application
        if rl.is_key_pressed(KeyboardKey::KEY_Y)
            || rl.is_key_pressed(KeyboardKey::KEY_ENTER)
            || (has_controller
                && (rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN)))
        {
            std::process::exit(0);
        }
    }

    // Helper functions for gamepad support
    fn add_next_letter(game: &mut Game) {
        if game.player_initials.len() <= 3 {
            // Get the last character or start with 'A'
            let last_char = game.player_initials.chars().last().unwrap_or('@');
            let next_char = if last_char == 'Z' {
                'A'
            } else {
                (last_char as u8 + 1) as char
            };

            // Remove last character if exists and add the new one
            if !game.player_initials.is_empty() {
                game.remove_initial();
            }
            game.add_initial(next_char);
        }
    }

    fn add_prev_letter(game: &mut Game) {
        if game.player_initials.len() <= 3 {
            // Get the last character or start with 'B'
            let last_char = game.player_initials.chars().last().unwrap_or('B');
            let prev_char = if last_char == 'A' {
                'Z'
            } else {
                (last_char as u8 - 1) as char
            };

            // Remove last character if exists and add the new one
            if !game.player_initials.is_empty() {
                game.remove_initial();
            }
            game.add_initial(prev_char);
        }
    }

    fn key_to_char(key: KeyboardKey) -> Option<char> {
        match key {
            KeyboardKey::KEY_A => Some('A'),
            KeyboardKey::KEY_B => Some('B'),
            KeyboardKey::KEY_C => Some('C'),
            KeyboardKey::KEY_D => Some('D'),
            KeyboardKey::KEY_E => Some('E'),
            KeyboardKey::KEY_F => Some('F'),
            KeyboardKey::KEY_G => Some('G'),
            KeyboardKey::KEY_H => Some('H'),
            KeyboardKey::KEY_I => Some('I'),
            KeyboardKey::KEY_J => Some('J'),
            KeyboardKey::KEY_K => Some('K'),
            KeyboardKey::KEY_L => Some('L'),
            KeyboardKey::KEY_M => Some('M'),
            KeyboardKey::KEY_N => Some('N'),
            KeyboardKey::KEY_O => Some('O'),
            KeyboardKey::KEY_P => Some('P'),
            KeyboardKey::KEY_Q => Some('Q'),
            KeyboardKey::KEY_R => Some('R'),
            KeyboardKey::KEY_S => Some('S'),
            KeyboardKey::KEY_T => Some('T'),
            KeyboardKey::KEY_U => Some('U'),
            KeyboardKey::KEY_V => Some('V'),
            KeyboardKey::KEY_W => Some('W'),
            KeyboardKey::KEY_X => Some('X'),
            KeyboardKey::KEY_Y => Some('Y'),
            KeyboardKey::KEY_Z => Some('Z'),
            _ => None,
        }
    }
}
