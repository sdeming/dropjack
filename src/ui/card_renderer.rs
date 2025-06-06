use crate::models::Card;
use crate::ui::atlas_card_renderer::AtlasCardRenderer;
use crate::ui::config::CardRendererConfig;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::prelude::Texture2D;

pub struct CardRenderer;

impl CardRenderer {
    /// Enhanced card rendering with decorative frame and atlas-based card display
    pub fn draw_card_inline(
        d: &mut RaylibDrawHandle,
        atlas: &Texture2D,
        card: Card,
        card_x: i32,
        card_y: i32,
        size: i32,
    ) {
        // Draw shadow layers for depth using configuration
        d.draw_rectangle(
            card_x + CardRendererConfig::SHADOW_OFFSET_1,
            card_y + CardRendererConfig::SHADOW_OFFSET_1,
            size,
            size,
            CardRendererConfig::SHADOW_LAYER_1_COLOR,
        );
        d.draw_rectangle(
            card_x + CardRendererConfig::SHADOW_OFFSET_2,
            card_y + CardRendererConfig::SHADOW_OFFSET_2,
            size,
            size,
            CardRendererConfig::SHADOW_LAYER_2_COLOR,
        );
        d.draw_rectangle(
            card_x + CardRendererConfig::SHADOW_OFFSET_3,
            card_y + CardRendererConfig::SHADOW_OFFSET_3,
            size,
            size,
            CardRendererConfig::SHADOW_LAYER_3_COLOR,
        );

        // Enhanced decorative frame system with configurable colors
        // Outer dark frame
        d.draw_rectangle(
            card_x - 3,
            card_y - 3,
            size + 6,
            size + 6,
            CardRendererConfig::FACE_DARK_COLOR,
        );
        // Middle frame with lighter brown
        d.draw_rectangle(
            card_x - 2,
            card_y - 2,
            size + 4,
            size + 4,
            CardRendererConfig::FACE_MEDIUM_COLOR,
        );
        // Inner highlight frame
        d.draw_rectangle(
            card_x - 1,
            card_y - 1,
            size + 2,
            size + 2,
            CardRendererConfig::FACE_LIGHT_COLOR,
        );

        // Use atlas card renderer for the actual card image
        AtlasCardRenderer::draw_card_from_card(d, atlas, card, card_x, card_y, size);

        // Enhanced lighting effects using configuration
        // Top highlight (simulating overhead light)
        d.draw_rectangle(
            card_x,
            card_y,
            size,
            CardRendererConfig::TOP_HIGHLIGHT_HEIGHT,
            CardRendererConfig::TOP_HIGHLIGHT_COLOR,
        );
        // Left edge highlight
        d.draw_rectangle(
            card_x,
            card_y,
            CardRendererConfig::LEFT_HIGHLIGHT_WIDTH,
            size,
            CardRendererConfig::LEFT_HIGHLIGHT_COLOR,
        );
        // Subtle inner glow
        d.draw_rectangle_lines(
            card_x + 1,
            card_y + 1,
            size - CardRendererConfig::BORDER_THICKNESS,
            size - CardRendererConfig::BORDER_THICKNESS,
            CardRendererConfig::BORDER_HIGHLIGHT_COLOR,
        );
    }
}
