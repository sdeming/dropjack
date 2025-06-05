use crate::models::{Card, Suit, Value};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Rectangle, Vector2};
use raylib::prelude::Texture2D;

/// Empty struct that provides static methods for rendering cards from atlas
pub struct AtlasCardRenderer;

// Atlas constants
const ATLAS_CARD_SIZE: i32 = 48;

/// Configuration for rendering a card from the atlas
#[derive(Debug, Clone, Copy)]
pub struct CardRenderOptions {
    pub x: i32,
    pub y: i32,
    pub size: i32,
    pub rotation: f32,
    pub tint: Color,
}

impl CardRenderOptions {
    pub fn new(x: i32, y: i32, size: i32) -> Self {
        Self {
            x,
            y,
            size,
            rotation: 0.0,
            tint: Color::WHITE,
        }
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_tint(mut self, tint: Color) -> Self {
        self.tint = tint;
        self
    }
}

impl AtlasCardRenderer {
    /// Draw a card from the atlas with basic parameters (no rotation, white tint)
    pub fn draw_card(
        d: &mut RaylibDrawHandle,
        atlas: &Texture2D,
        card: Card,
        x: i32,
        y: i32,
        size: i32,
    ) {
        let options = CardRenderOptions::new(x, y, size);
        Self::draw_card_with_options(d, atlas, card, options);
    }

    /// Draw a card from the atlas with full customization options
    pub fn draw_card_with_options(
        d: &mut RaylibDrawHandle,
        atlas: &Texture2D,
        card: Card,
        options: CardRenderOptions,
    ) {
        let (atlas_row, atlas_col) = Self::get_atlas_position(card);

        let source_rect = Rectangle::new(
            (atlas_col * ATLAS_CARD_SIZE) as f32,
            (atlas_row * ATLAS_CARD_SIZE) as f32,
            ATLAS_CARD_SIZE as f32,
            ATLAS_CARD_SIZE as f32,
        );

        let dest_rect = Rectangle::new(
            options.x as f32,
            options.y as f32,
            options.size as f32,
            options.size as f32,
        );

        d.draw_texture_pro(
            atlas,
            source_rect,
            dest_rect,
            Vector2::zero(),
            options.rotation,
            options.tint,
        );
    }

    /// Get atlas position for a card (row, column)
    pub fn get_atlas_position(card: Card) -> (i32, i32) {
        let atlas_row = match card.suit {
            Suit::Spades => 0,
            Suit::Hearts => 1,
            Suit::Diamonds => 2,
            Suit::Clubs => 3,
        };

        let atlas_col = match card.value {
            Value::Ace => 0,
            Value::Two => 1,
            Value::Three => 2,
            Value::Four => 3,
            Value::Five => 4,
            Value::Six => 5,
            Value::Seven => 6,
            Value::Eight => 7,
            Value::Nine => 8,
            Value::Ten => 9,
            Value::Jack => 10,
            Value::Queen => 11,
            Value::King => 12,
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
        Self::draw_card(d, atlas, card, x, y, size);
    }
}
