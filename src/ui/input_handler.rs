use crate::game::Game;
use raylib::prelude::*;

pub struct InputHandler {
    last_move_time: std::time::Instant,
    move_delay: std::time::Duration,
}

/// Input mapping for different controllers and keyboards
struct InputMapping;

impl InputMapping {
    /// Check if any "left" input is pressed
    fn is_left_pressed(rl: &RaylibHandle, has_controller: bool) -> bool {
        rl.is_key_pressed(KeyboardKey::KEY_LEFT)
            || (has_controller
                && (rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT)
                    || rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X) < -0.3))
    }

    /// Check if any "right" input is pressed
    fn is_right_pressed(rl: &RaylibHandle, has_controller: bool) -> bool {
        rl.is_key_pressed(KeyboardKey::KEY_RIGHT)
            || (has_controller
                && (rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT)
                    || rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X) > 0.3))
    }

    /// Check if any "left" input is held down
    fn is_left_down(rl: &RaylibHandle, has_controller: bool) -> bool {
        rl.is_key_down(KeyboardKey::KEY_LEFT)
            || (has_controller
                && (rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT)
                    || rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X) < -0.3))
    }

    /// Check if any "right" input is held down
    fn is_right_down(rl: &RaylibHandle, has_controller: bool) -> bool {
        rl.is_key_down(KeyboardKey::KEY_RIGHT)
            || (has_controller
                && (rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT)
                    || rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_X) > 0.3))
    }

    /// Check if any "up" input is pressed
    fn is_up_pressed(rl: &RaylibHandle, has_controller: bool) -> bool {
        rl.is_key_pressed(KeyboardKey::KEY_UP)
            || (has_controller
                && (rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP)
                    || rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_Y) < -0.3))
    }

    /// Check if any "down" input is pressed
    fn is_down_pressed(rl: &RaylibHandle, has_controller: bool) -> bool {
        rl.is_key_pressed(KeyboardKey::KEY_DOWN)
            || (has_controller
                && (rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN)
                    || rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_Y) > 0.3))
    }

    /// Check if any "down" input is held down
    fn is_down_down(rl: &RaylibHandle, has_controller: bool) -> bool {
        rl.is_key_down(KeyboardKey::KEY_DOWN)
            || (has_controller
                && (rl.is_gamepad_button_down(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN)
                    || rl.get_gamepad_axis_movement(0, GamepadAxis::GAMEPAD_AXIS_LEFT_Y) > 0.3))
    }

    /// Check if any "action/space" input is pressed
    fn is_action_pressed(rl: &RaylibHandle, has_controller: bool) -> bool {
        rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN))
    }

    /// Check if any "escape/menu" input is pressed
    fn is_escape_pressed(rl: &RaylibHandle, has_controller: bool) -> bool {
        rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_LEFT))
    }

    /// Check if any "confirm/enter" input is pressed
    fn is_confirm_pressed(rl: &RaylibHandle, has_controller: bool) -> bool {
        rl.is_key_pressed(KeyboardKey::KEY_ENTER)
            || rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_MIDDLE_RIGHT))
    }
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
        } else if game.is_settings() {
            self.handle_settings_input(rl, game, has_controller);
        }
    }

    fn handle_start_screen_input(
        &self,
        rl: &mut RaylibHandle,
        game: &mut Game,
        has_controller: bool,
    ) {
        // Handle navigation in main menu
        if InputMapping::is_up_pressed(rl, has_controller) {
            if game.selected_main_option > 0 {
                game.selected_main_option -= 1;
            } else {
                game.selected_main_option = 2;
            }
            game.add_audio_event(crate::game::AudioEvent::DifficultyChange);
        }

        if InputMapping::is_down_pressed(rl, has_controller) {
            if game.selected_main_option < 2 {
                game.selected_main_option += 1;
            } else {
                game.selected_main_option = 0;
            }
            game.add_audio_event(crate::game::AudioEvent::DifficultyChange);
        }

        // Handle selection
        if InputMapping::is_confirm_pressed(rl, has_controller) {
            match game.selected_main_option {
                0 => {
                    // Start New Game
                    game.start_game(game.settings.difficulty);
                }
                1 => {
                    // Settings
                    game.transition_to_settings("StartScreen".to_string());
                }
                2 => {
                    // Quit
                    game.transition_to_quit_confirm();
                }
                _ => {}
            }
        }

        // Handle quit confirmation directly with ESC
        if InputMapping::is_escape_pressed(rl, has_controller) {
            game.transition_to_quit_confirm();
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
            if InputMapping::is_left_down(rl, has_controller) {
                game.move_current_card_left();
                self.last_move_time = now;
            } else if InputMapping::is_right_down(rl, has_controller) {
                game.move_current_card_right();
                self.last_move_time = now;
            }
        }

        // Handle soft drop (down key)
        if InputMapping::is_down_down(rl, has_controller) {
            game.move_current_card_down();
        }

        // Handle hard drop (space key)
        if InputMapping::is_action_pressed(rl, has_controller) {
            game.hard_drop();
        }

        // Handle pause
        if InputMapping::is_escape_pressed(rl, has_controller) {
            game.transition_to_paused();
        }
    }

    fn handle_paused_input(&self, rl: &mut RaylibHandle, game: &mut Game, has_controller: bool) {
        // Handle settings screen
        if rl.is_key_pressed(KeyboardKey::KEY_S)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP))
        {
            game.transition_to_settings("Paused".to_string());
            return;
        }

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
            game.add_audio_event(crate::game::AudioEvent::ForfeitGame);
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
            game.add_audio_event(crate::game::AudioEvent::QuitGame);
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

    fn handle_settings_input(&self, rl: &mut RaylibHandle, game: &mut Game, has_controller: bool) {
        const TOTAL_OPTIONS: usize = 4; // Music, SFX, VSync, Difficulty

        // Back to previous screen
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_RIGHT))
        {
            game.transition_to_start_screen();
            return;
        }

        // Navigation (Up/Down)
        if rl.is_key_pressed(KeyboardKey::KEY_UP)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_UP))
        {
            if game.settings.selected_option > 0 {
                game.settings.selected_option -= 1;
            } else {
                game.settings.selected_option = TOTAL_OPTIONS - 1; // Wrap to bottom
            }
            if !game.settings.sound_effects_muted {
                game.add_audio_event(crate::game::AudioEvent::MoveLeft);
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_DOWN)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_DOWN))
        {
            game.settings.selected_option = (game.settings.selected_option + 1) % TOTAL_OPTIONS;
            if !game.settings.sound_effects_muted {
                game.add_audio_event(crate::game::AudioEvent::MoveRight);
            }
        }

        // Adjust values based on current selection (Left/Right)
        let left_pressed = rl.is_key_pressed(KeyboardKey::KEY_LEFT)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_LEFT));
        let right_pressed = rl.is_key_pressed(KeyboardKey::KEY_RIGHT)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_LEFT_FACE_RIGHT));

        match game.settings.selected_option {
            0 => {
                // Music Volume
                if left_pressed {
                    game.settings.music_volume = (game.settings.music_volume - 0.1).max(0.0);
                    if !game.settings.sound_effects_muted {
                        game.add_audio_event(crate::game::AudioEvent::DifficultyChange);
                    }
                    game.save_settings();
                }
                if right_pressed {
                    game.settings.music_volume = (game.settings.music_volume + 0.1).min(1.0);
                    if !game.settings.sound_effects_muted {
                        game.add_audio_event(crate::game::AudioEvent::DifficultyChange);
                    }
                    game.save_settings();
                }
            }
            1 => {
                // Sound Effects Volume
                if left_pressed {
                    game.settings.sound_effects_volume =
                        (game.settings.sound_effects_volume - 0.1).max(0.0);
                    game.add_audio_event(crate::game::AudioEvent::DifficultyChange);
                    game.save_settings();
                }
                if right_pressed {
                    game.settings.sound_effects_volume =
                        (game.settings.sound_effects_volume + 0.1).min(1.0);
                    game.add_audio_event(crate::game::AudioEvent::DifficultyChange);
                    game.save_settings();
                }
            }
            2 => { // VSync - no left/right adjustment, only toggle
                // VSync doesn't have adjustable values, only toggle
            }
            3 => {
                // Difficulty
                if left_pressed || right_pressed {
                    game.settings.difficulty = match game.settings.difficulty {
                        crate::models::Difficulty::Easy => crate::models::Difficulty::Hard,
                        crate::models::Difficulty::Hard => crate::models::Difficulty::Easy,
                    };
                    // Also update the main game difficulty for consistency
                    game.difficulty = game.settings.difficulty;
                    if !game.settings.sound_effects_muted {
                        game.add_audio_event(crate::game::AudioEvent::DifficultyChange);
                    }
                    game.save_settings();
                }
            }
            _ => {}
        }

        // Toggle actions (Space/A button)
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE)
            || (has_controller
                && rl.is_gamepad_button_pressed(0, GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN))
        {
            match game.settings.selected_option {
                0 => {
                    // Music Mute Toggle
                    game.settings.music_muted = !game.settings.music_muted;
                    if !game.settings.sound_effects_muted {
                        game.add_audio_event(crate::game::AudioEvent::PauseGame);
                    }
                    game.save_settings();
                }
                1 => {
                    // Sound Effects Mute Toggle
                    let was_muted = game.settings.sound_effects_muted;
                    game.settings.sound_effects_muted = !game.settings.sound_effects_muted;
                    if was_muted && !game.settings.sound_effects_muted {
                        game.add_audio_event(crate::game::AudioEvent::ResumeGame);
                    }
                    game.save_settings();
                }
                2 => {
                    // VSync Toggle
                    game.settings.vsync_enabled = !game.settings.vsync_enabled;
                    if !game.settings.sound_effects_muted {
                        game.add_audio_event(crate::game::AudioEvent::StartGame);
                    }
                    game.save_settings();
                }
                3 => {
                    // Difficulty Toggle (same as left/right)
                    game.settings.difficulty = match game.settings.difficulty {
                        crate::models::Difficulty::Easy => crate::models::Difficulty::Hard,
                        crate::models::Difficulty::Hard => crate::models::Difficulty::Easy,
                    };
                    // Also update the main game difficulty for consistency
                    game.difficulty = game.settings.difficulty;
                    if !game.settings.sound_effects_muted {
                        game.add_audio_event(crate::game::AudioEvent::DifficultyChange);
                    }
                    game.save_settings();
                }
                _ => {}
            }
        }
    }
}
