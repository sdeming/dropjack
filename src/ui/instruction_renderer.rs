use crate::ui::config::InstructionsConfig;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::prelude::Font;

pub struct InstructionRenderer;

impl InstructionRenderer {
    pub fn draw_controls(
        d: &mut RaylibDrawHandle,
        title_font: &Font,
        font: &Font,
        info_panel_x: i32,
        board_offset_y: i32,
        has_controller: bool,
    ) {
        // Enhanced controls title with glow effect
        let controls_x = info_panel_x + InstructionsConfig::X_OFFSET;
        let controls_y = board_offset_y + InstructionsConfig::Y_OFFSET;

        // Glow effect for the title
        for glow in 1..=InstructionsConfig::GLOW_LAYERS {
            let glow_alpha = 40 / glow;
            d.draw_text_ex(
                title_font,
                "Controls:",
                Vector2::new((controls_x + glow) as f32, (controls_y + glow) as f32),
                InstructionsConfig::TITLE_SIZE,
                1.0,
                Color::new(255, 215, 0, glow_alpha as u8),
            );
        }

        // Shadow
        d.draw_text_ex(
            title_font,
            "Controls:",
            Vector2::new(
                (controls_x + InstructionsConfig::SHADOW_X_OFFSET) as f32,
                (controls_y + InstructionsConfig::SHADOW_Y_OFFSET) as f32,
            ),
            InstructionsConfig::TITLE_SIZE,
            1.0,
            InstructionsConfig::SHADOW_COLOR,
        );

        // Main title
        d.draw_text_ex(
            title_font,
            "Controls:",
            Vector2::new(controls_x as f32, controls_y as f32),
            InstructionsConfig::TITLE_SIZE,
            1.0,
            InstructionsConfig::TITLE_COLOR,
        );

        let instructions = match has_controller {
            true => [
                (
                    "D-Pad/Left Stick: Move card",
                    InstructionsConfig::MOVE_COLOR,
                ),
                (
                    "D-Pad Down/Stick Down: Soft drop",
                    InstructionsConfig::SOFT_DROP_COLOR,
                ),
                ("A Button: Hard drop", InstructionsConfig::HARD_DROP_COLOR),
                ("Start: Pause", InstructionsConfig::PAUSE_COLOR),
            ],
            false => [
                (
                    "Left/Right Arrow: Move card",
                    InstructionsConfig::KEYBOARD_COLOR,
                ),
                ("Down Arrow: Soft drop", InstructionsConfig::SOFT_DROP_COLOR),
                ("Space: Hard drop", InstructionsConfig::HARD_DROP_COLOR),
                ("Escape: Pause", InstructionsConfig::PAUSE_COLOR),
            ],
        };

        for (i, (text, color)) in instructions.iter().enumerate() {
            let y_pos = controls_y
                + InstructionsConfig::Y_START_OFFSET
                + i as i32 * InstructionsConfig::LINE_SPACING;

            // Subtle shadow for each instruction
            d.draw_text_ex(
                font,
                text,
                Vector2::new(
                    (controls_x + InstructionsConfig::TEXT_X_OFFSET) as f32,
                    (y_pos + InstructionsConfig::TEXT_Y_OFFSET) as f32,
                ),
                InstructionsConfig::TEXT_SIZE,
                1.0,
                InstructionsConfig::TEXT_SHADOW_COLOR,
            );

            // Main text with color coding
            d.draw_text_ex(
                font,
                text,
                Vector2::new(controls_x as f32, y_pos as f32),
                InstructionsConfig::TEXT_SIZE,
                1.0,
                *color,
            );
        }
    }

    pub fn draw_game_over_instructions(
        d: &mut RaylibDrawHandle,
        font: &Font,
        has_controller: bool,
    ) {
        if has_controller {
            d.draw_text_ex(
                font,
                "D-Pad: Cycle letters, A: Next/Accept, B: Backspace",
                Vector2::new(
                    InstructionsConfig::GAME_OVER_X,
                    InstructionsConfig::GAME_OVER_Y,
                ),
                InstructionsConfig::GAME_OVER_SIZE,
                1.0,
                InstructionsConfig::CONTROLLER_COLOR,
            );
        } else {
            d.draw_text_ex(
                font,
                "Type your initials, then press ENTER when done",
                Vector2::new(
                    InstructionsConfig::GAME_OVER_X_ALT,
                    InstructionsConfig::GAME_OVER_Y,
                ),
                InstructionsConfig::GAME_OVER_SIZE,
                1.0,
                Color::LIGHTGRAY,
            );
        }
    }

    pub fn draw_quit_confirmation(d: &mut RaylibDrawHandle, font: &Font, has_controller: bool) {
        if has_controller {
            d.draw_text_ex(
                font,
                "Press A to Quit",
                Vector2::new(
                    InstructionsConfig::QUIT_CONFIRM_QUIT_X,
                    InstructionsConfig::QUIT_CONFIRM_QUIT_Y,
                ),
                InstructionsConfig::QUIT_CONFIRM_SIZE,
                InstructionsConfig::QUIT_CONFIRM_SPACING,
                InstructionsConfig::QUIT_COLOR,
            );
            d.draw_text_ex(
                font,
                "Press B to Cancel",
                Vector2::new(
                    InstructionsConfig::QUIT_CONFIRM_CANCEL_X,
                    InstructionsConfig::QUIT_CONFIRM_CANCEL_Y,
                ),
                InstructionsConfig::QUIT_CONFIRM_SIZE,
                InstructionsConfig::QUIT_CONFIRM_SPACING,
                InstructionsConfig::RESUME_COLOR,
            );
        } else {
            d.draw_text_ex(
                font,
                "Press Y to Quit",
                Vector2::new(
                    InstructionsConfig::QUIT_CONFIRM_QUIT_X,
                    InstructionsConfig::QUIT_CONFIRM_QUIT_Y,
                ),
                InstructionsConfig::QUIT_CONFIRM_SIZE,
                InstructionsConfig::QUIT_CONFIRM_SPACING,
                InstructionsConfig::QUIT_COLOR,
            );
            d.draw_text_ex(
                font,
                "Press N or ESC to Cancel",
                Vector2::new(
                    InstructionsConfig::QUIT_CONFIRM_CANCEL_X_ALT,
                    InstructionsConfig::QUIT_CONFIRM_CANCEL_Y,
                ),
                InstructionsConfig::QUIT_CONFIRM_SIZE,
                InstructionsConfig::QUIT_CONFIRM_SPACING,
                InstructionsConfig::RESUME_COLOR,
            );
        }
    }

    pub fn draw_pause_instructions(d: &mut RaylibDrawHandle, font: &Font, has_controller: bool) {
        if has_controller {
            d.draw_text_ex(
                font,
                "Press A to Forfeit",
                Vector2::new(
                    InstructionsConfig::PAUSE_FORFEIT_X,
                    InstructionsConfig::PAUSE_FORFEIT_Y,
                ),
                InstructionsConfig::QUIT_CONFIRM_SIZE,
                InstructionsConfig::QUIT_CONFIRM_SPACING,
                InstructionsConfig::QUIT_COLOR,
            );
            d.draw_text_ex(
                font,
                "Press B to Resume",
                Vector2::new(
                    InstructionsConfig::PAUSE_RESUME_X,
                    InstructionsConfig::PAUSE_RESUME_Y,
                ),
                InstructionsConfig::QUIT_CONFIRM_SIZE,
                InstructionsConfig::QUIT_CONFIRM_SPACING,
                InstructionsConfig::RESUME_COLOR,
            );
        } else {
            d.draw_text_ex(
                font,
                "Press N or ESC to Resume",
                Vector2::new(
                    InstructionsConfig::PAUSE_RESUME_X_ALT,
                    InstructionsConfig::PAUSE_FORFEIT_Y,
                ),
                InstructionsConfig::QUIT_CONFIRM_SIZE,
                InstructionsConfig::QUIT_CONFIRM_SPACING,
                InstructionsConfig::RESUME_COLOR,
            );
            d.draw_text_ex(
                font,
                "Press Y to Quit to Menu",
                Vector2::new(
                    InstructionsConfig::PAUSE_QUIT_X,
                    InstructionsConfig::PAUSE_RESUME_Y,
                ),
                InstructionsConfig::QUIT_CONFIRM_SIZE,
                InstructionsConfig::QUIT_CONFIRM_SPACING,
                InstructionsConfig::QUIT_COLOR,
            );
        }
    }
}
