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
        let controls_x = info_panel_x + 30;
        let controls_y = board_offset_y + 350;

        // Glow effect for the title
        for glow in 1..=3 {
            let glow_alpha = 40 / glow;
            d.draw_text_ex(
                title_font,
                "Controls:",
                Vector2::new((controls_x + glow) as f32, (controls_y + glow) as f32),
                28.0,
                1.0,
                Color::new(255, 215, 0, glow_alpha as u8),
            );
        }

        // Shadow
        d.draw_text_ex(
            title_font,
            "Controls:",
            Vector2::new((controls_x + 2) as f32, (controls_y + 2) as f32),
            28.0,
            1.0,
            Color::new(0, 0, 0, 150),
        );

        // Main title
        d.draw_text_ex(
            title_font,
            "Controls:",
            Vector2::new(controls_x as f32, controls_y as f32),
            28.0,
            1.0,
            Color::new(255, 215, 0, 255),
        );

        let instructions = match has_controller {
            true => [
                (
                    "D-Pad/Left Stick: Move card",
                    Color::new(150, 255, 150, 255),
                ),
                (
                    "D-Pad Down/Stick Down: Soft drop",
                    Color::new(200, 200, 255, 255),
                ),
                ("A Button: Hard drop", Color::new(255, 200, 150, 255)),
                ("Start: Pause", Color::new(255, 150, 200, 255)),
            ],
            false => [
                (
                    "Left/Right Arrow: Move card",
                    Color::new(255, 255, 150, 255),
                ),
                ("Down Arrow: Soft drop", Color::new(200, 200, 255, 255)),
                ("Space: Hard drop", Color::new(255, 200, 150, 255)),
                ("Escape: Pause", Color::new(255, 150, 200, 255)),
            ],
        };

        for (i, (text, color)) in instructions.iter().enumerate() {
            let y_pos = controls_y + 40 + i as i32 * 25;

            // Subtle shadow for each instruction
            d.draw_text_ex(
                font,
                text,
                Vector2::new((controls_x + 1) as f32, (y_pos + 1) as f32),
                18.0,
                1.0,
                Color::new(0, 0, 0, 100),
            );

            // Main text with color coding
            d.draw_text_ex(
                font,
                text,
                Vector2::new(controls_x as f32, y_pos as f32),
                18.0,
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
                Vector2::new(440.0, 530.0),
                20.0,
                1.0,
                Color::new(150, 200, 255, 255),
            );
        } else {
            d.draw_text_ex(
                font,
                "Type your initials, then press ENTER when done",
                Vector2::new(420.0, 530.0),
                20.0,
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
                Vector2::new(560.0, 400.0),
                24.0,
                1.2,
                Color::new(255, 150, 150, 255),
            );
            d.draw_text_ex(
                font,
                "Press B to Cancel",
                Vector2::new(545.0, 440.0),
                24.0,
                1.2,
                Color::new(150, 255, 150, 255),
            );
        } else {
            d.draw_text_ex(
                font,
                "Press Y to Quit",
                Vector2::new(560.0, 400.0),
                24.0,
                1.2,
                Color::new(255, 150, 150, 255),
            );
            d.draw_text_ex(
                font,
                "Press N or ESC to Cancel",
                Vector2::new(510.0, 440.0),
                24.0,
                1.2,
                Color::new(150, 255, 150, 255),
            );
        }
    }

    pub fn draw_pause_instructions(d: &mut RaylibDrawHandle, font: &Font, has_controller: bool) {
        if has_controller {
            d.draw_text_ex(
                font,
                "Press A to Forfeit",
                Vector2::new(540.0, 420.0),
                24.0,
                1.2,
                Color::new(255, 150, 150, 255),
            );
            d.draw_text_ex(
                font,
                "Press B to Resume",
                Vector2::new(535.0, 460.0),
                24.0,
                1.2,
                Color::new(150, 255, 150, 255),
            );
        } else {
            d.draw_text_ex(
                font,
                "Press N or ESC to Resume",
                Vector2::new(495.0, 420.0),
                24.0,
                1.2,
                Color::new(150, 255, 150, 255),
            );
            d.draw_text_ex(
                font,
                "Press Y to Quit to Menu",
                Vector2::new(505.0, 460.0),
                24.0,
                1.2,
                Color::new(255, 150, 150, 255),
            );
        }
    }
}
