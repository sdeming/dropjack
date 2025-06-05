use crate::ui::constants::*;
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
        let controls_x = info_panel_x + INSTRUCTIONS_X_OFFSET;
        let controls_y = board_offset_y + INSTRUCTIONS_Y_OFFSET;

        // Glow effect for the title
        for glow in 1..=INSTRUCTIONS_GLOW_LAYERS {
            let glow_alpha = 40 / glow;
            d.draw_text_ex(
                title_font,
                "Controls:",
                Vector2::new((controls_x + glow) as f32, (controls_y + glow) as f32),
                INSTRUCTIONS_TITLE_SIZE,
                1.0,
                Color::new(255, 215, 0, glow_alpha as u8),
            );
        }

        // Shadow
        d.draw_text_ex(
            title_font,
            "Controls:",
            Vector2::new(
                (controls_x + INSTRUCTIONS_SHADOW_X_OFFSET) as f32,
                (controls_y + INSTRUCTIONS_SHADOW_Y_OFFSET) as f32,
            ),
            INSTRUCTIONS_TITLE_SIZE,
            1.0,
            INSTRUCTIONS_SHADOW_COLOR,
        );

        // Main title
        d.draw_text_ex(
            title_font,
            "Controls:",
            Vector2::new(controls_x as f32, controls_y as f32),
            INSTRUCTIONS_TITLE_SIZE,
            1.0,
            INSTRUCTIONS_TITLE_COLOR,
        );

        let instructions = match has_controller {
            true => [
                ("D-Pad/Left Stick: Move card", INSTRUCTIONS_MOVE_COLOR),
                (
                    "D-Pad Down/Stick Down: Soft drop",
                    INSTRUCTIONS_SOFT_DROP_COLOR,
                ),
                ("A Button: Hard drop", INSTRUCTIONS_HARD_DROP_COLOR),
                ("Start: Pause", INSTRUCTIONS_PAUSE_COLOR),
            ],
            false => [
                (
                    "Left/Right Arrow: Move card",
                    Color::new(255, 255, 150, 255),
                ),
                ("Down Arrow: Soft drop", INSTRUCTIONS_SOFT_DROP_COLOR),
                ("Space: Hard drop", INSTRUCTIONS_HARD_DROP_COLOR),
                ("Escape: Pause", INSTRUCTIONS_PAUSE_COLOR),
            ],
        };

        for (i, (text, color)) in instructions.iter().enumerate() {
            let y_pos =
                controls_y + INSTRUCTIONS_Y_START_OFFSET + i as i32 * INSTRUCTIONS_LINE_SPACING;

            // Subtle shadow for each instruction
            d.draw_text_ex(
                font,
                text,
                Vector2::new(
                    (controls_x + INSTRUCTIONS_TEXT_X_OFFSET) as f32,
                    (y_pos + INSTRUCTIONS_TEXT_Y_OFFSET) as f32,
                ),
                INSTRUCTIONS_TEXT_SIZE,
                1.0,
                INSTRUCTIONS_TEXT_SHADOW_COLOR,
            );

            // Main text with color coding
            d.draw_text_ex(
                font,
                text,
                Vector2::new(controls_x as f32, y_pos as f32),
                INSTRUCTIONS_TEXT_SIZE,
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
                Vector2::new(GAME_OVER_INSTRUCTION_X, GAME_OVER_INSTRUCTION_Y),
                GAME_OVER_INSTRUCTION_SIZE,
                1.0,
                INSTRUCTION_CONTROLLER_COLOR,
            );
        } else {
            d.draw_text_ex(
                font,
                "Type your initials, then press ENTER when done",
                Vector2::new(GAME_OVER_INSTRUCTION_X_ALT, GAME_OVER_INSTRUCTION_Y),
                GAME_OVER_INSTRUCTION_SIZE,
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
                Vector2::new(QUIT_CONFIRM_QUIT_X, QUIT_CONFIRM_QUIT_Y),
                QUIT_CONFIRM_SIZE,
                QUIT_CONFIRM_SPACING,
                INSTRUCTION_QUIT_COLOR,
            );
            d.draw_text_ex(
                font,
                "Press B to Cancel",
                Vector2::new(QUIT_CONFIRM_CANCEL_X, QUIT_CONFIRM_CANCEL_Y),
                QUIT_CONFIRM_SIZE,
                QUIT_CONFIRM_SPACING,
                INSTRUCTION_RESUME_COLOR,
            );
        } else {
            d.draw_text_ex(
                font,
                "Press Y to Quit",
                Vector2::new(QUIT_CONFIRM_QUIT_X, QUIT_CONFIRM_QUIT_Y),
                QUIT_CONFIRM_SIZE,
                QUIT_CONFIRM_SPACING,
                INSTRUCTION_QUIT_COLOR,
            );
            d.draw_text_ex(
                font,
                "Press N or ESC to Cancel",
                Vector2::new(QUIT_CONFIRM_CANCEL_X_ALT, QUIT_CONFIRM_CANCEL_Y),
                QUIT_CONFIRM_SIZE,
                QUIT_CONFIRM_SPACING,
                INSTRUCTION_RESUME_COLOR,
            );
        }
    }

    pub fn draw_pause_instructions(d: &mut RaylibDrawHandle, font: &Font, has_controller: bool) {
        if has_controller {
            d.draw_text_ex(
                font,
                "Press A to Forfeit",
                Vector2::new(PAUSE_FORFEIT_X, PAUSE_FORFEIT_Y),
                QUIT_CONFIRM_SIZE,
                QUIT_CONFIRM_SPACING,
                INSTRUCTION_QUIT_COLOR,
            );
            d.draw_text_ex(
                font,
                "Press B to Resume",
                Vector2::new(PAUSE_RESUME_X, PAUSE_RESUME_Y),
                QUIT_CONFIRM_SIZE,
                QUIT_CONFIRM_SPACING,
                INSTRUCTION_RESUME_COLOR,
            );
        } else {
            d.draw_text_ex(
                font,
                "Press N or ESC to Resume",
                Vector2::new(PAUSE_RESUME_X_ALT, PAUSE_FORFEIT_Y),
                QUIT_CONFIRM_SIZE,
                QUIT_CONFIRM_SPACING,
                INSTRUCTION_RESUME_COLOR,
            );
            d.draw_text_ex(
                font,
                "Press Y to Quit to Menu",
                Vector2::new(PAUSE_QUIT_X, PAUSE_RESUME_Y),
                QUIT_CONFIRM_SIZE,
                QUIT_CONFIRM_SPACING,
                INSTRUCTION_QUIT_COLOR,
            );
        }
    }
}
