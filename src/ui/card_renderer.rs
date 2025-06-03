use crate::models::Card;
use crate::ui::drawing::AtlasCardRenderer;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::prelude::Texture2D;

pub struct CardRenderer;

impl CardRenderer {
    pub fn draw_card_inline(
        d: &mut RaylibDrawHandle,
        atlas: &Texture2D,
        card: Card,
        card_x: i32,
        card_y: i32,
        size: i32,
    ) {
        // Enhanced multi-layer shadow system for dramatic depth
        // Outer shadow (most diffuse)
        d.draw_rectangle(card_x + 6, card_y + 6, size, size, Color::new(0, 0, 0, 40));
        // Middle shadow
        d.draw_rectangle(card_x + 4, card_y + 4, size, size, Color::new(0, 0, 0, 60));
        // Inner shadow (sharpest)
        d.draw_rectangle(card_x + 2, card_y + 2, size, size, Color::new(0, 0, 0, 80));
        
        // Enhanced border system with beveled edges
        // Outer dark frame
        d.draw_rectangle(card_x - 3, card_y - 3, size + 6, size + 6, Color::new(101, 50, 14, 255));
        // Middle frame with lighter brown
        d.draw_rectangle(card_x - 2, card_y - 2, size + 4, size + 4, Color::new(139, 69, 19, 255));
        // Inner highlight frame
        d.draw_rectangle(card_x - 1, card_y - 1, size + 2, size + 2, Color::new(222, 184, 135, 255));

        // Use atlas card renderer
        AtlasCardRenderer::draw_card_from_card(d, atlas, card, card_x, card_y, size);
        
        // Enhanced lighting effects
        // Top highlight (simulating overhead light)
        d.draw_rectangle(card_x, card_y, size, 3, Color::new(255, 255, 255, 80));
        // Left edge highlight 
        d.draw_rectangle(card_x, card_y, 2, size, Color::new(255, 255, 255, 50));
        // Subtle inner glow
        d.draw_rectangle_lines(card_x + 1, card_y + 1, size - 2, size - 2, Color::new(255, 255, 255, 30));
    }
} 