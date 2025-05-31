use crate::cards::Card;
use crate::ui::drawing::AtlasCardRenderer;
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::Texture2D;

// Atlas constants
const ATLAS_CARD_SIZE: i32 = 48;

impl AtlasCardRenderer {
    /// Draw a card from the atlas with basic parameters (no rotation, white tint)
    pub fn draw_card(
        d: &mut RaylibDrawHandle,
        atlas: &Texture2D,
        atlas_row: i32,
        atlas_col: i32,
        x: i32,
        y: i32,
        size: i32,
    ) {
        Self::draw_card_with_options(
            d,
            atlas,
            atlas_row,
            atlas_col,
            x,
            y,
            size,
            0.0,
            Color::WHITE,
        );
    }

    /// Draw a card from the atlas with full customization options
    pub fn draw_card_with_options(
        d: &mut RaylibDrawHandle,
        atlas: &Texture2D,
        atlas_row: i32,
        atlas_col: i32,
        x: i32,
        y: i32,
        size: i32,
        rotation: f32,
        tint: Color,
    ) {
        let source_rect = Rectangle::new(
            (atlas_col * ATLAS_CARD_SIZE) as f32,
            (atlas_row * ATLAS_CARD_SIZE) as f32,
            ATLAS_CARD_SIZE as f32,
            ATLAS_CARD_SIZE as f32,
        );

        let dest_rect = Rectangle::new(x as f32, y as f32, size as f32, size as f32);

        d.draw_texture_pro(
            atlas,
            source_rect,
            dest_rect,
            Vector2::zero(),
            rotation,
            tint,
        );
    }

    /// Get atlas position for a card (row, column)
    pub fn get_atlas_position(card: Card) -> (i32, i32) {
        let atlas_row = match card.suit {
            crate::cards::Suit::Spades => 0,
            crate::cards::Suit::Hearts => 1,
            crate::cards::Suit::Diamonds => 2,
            crate::cards::Suit::Clubs => 3,
        };

        let atlas_col = match card.value {
            crate::cards::Value::Ace => 0,
            crate::cards::Value::Two => 1,
            crate::cards::Value::Three => 2,
            crate::cards::Value::Four => 3,
            crate::cards::Value::Five => 4,
            crate::cards::Value::Six => 5,
            crate::cards::Value::Seven => 6,
            crate::cards::Value::Eight => 7,
            crate::cards::Value::Nine => 8,
            crate::cards::Value::Ten => 9,
            crate::cards::Value::Jack => 10,
            crate::cards::Value::Queen => 11,
            crate::cards::Value::King => 12,
        };

        (atlas_row, atlas_col)
    }

    /// Draw a specific card from the atlas
    pub fn draw_card_from_card(
        d: &mut RaylibDrawHandle,
        atlas: &Texture2D,
        card: Card,
        x: i32,
        y: i32,
        size: i32,
    ) {
        let (atlas_row, atlas_col) = Self::get_atlas_position(card);
        Self::draw_card(d, atlas, atlas_row, atlas_col, x, y, size);
    }
}
