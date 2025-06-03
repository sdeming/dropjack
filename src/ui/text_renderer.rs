use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::prelude::Font;

pub struct TextRenderer;

impl TextRenderer {
    pub fn draw_title_with_shadow(d: &mut RaylibDrawHandle, title_font: &Font) {
        let title = "DropJack";
        let title_x = 600.0 - 160.0; // Centered position for larger title
        let title_y = 60.0; // Moved down slightly
        let title_size = 120.0; // Increased from 80.0

        // Draw shadow layers for depth
        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(title_x + 6.0, title_y + 6.0),
            title_size,
            2.0,
            Color::new(0, 0, 0, 150),
        );
        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(title_x + 3.0, title_y + 3.0),
            title_size,
            2.0,
            Color::new(0, 0, 0, 100),
        );
        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(title_x + 1.5, title_y + 1.5),
            title_size,
            2.0,
            Color::new(0, 0, 0, 50),
        );

        // Main title with gradient effect (simulate by drawing slightly offset)
        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(title_x, title_y),
            title_size,
            2.0,
            Color::new(255, 215, 0, 255),
        ); // Gold
        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(title_x, title_y - 1.0),
            title_size,
            2.0,
            Color::new(255, 255, 255, 200),
        ); // White highlight
    }

    pub fn draw_subtitle(d: &mut RaylibDrawHandle, font: &Font) {
        let subtitle = "A Strategic Card-Falling Puzzle";
        let subtitle_x = 600.0 - 140.0;
        let subtitle_y = 200.0;
        let subtitle_size = 32.0;

        d.draw_text_ex(
            font,
            subtitle,
            Vector2::new(subtitle_x + 2.0, subtitle_y + 2.0),
            subtitle_size,
            1.0,
            Color::new(0, 0, 0, 80),
        );
        d.draw_text_ex(
            font,
            subtitle,
            Vector2::new(subtitle_x, subtitle_y),
            subtitle_size,
            1.0,
            Color::new(200, 200, 255, 255),
        );
    }
} 