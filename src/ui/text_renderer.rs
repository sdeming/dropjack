use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::Vector2;
use raylib::prelude::Font;
use std::sync::LazyLock;

pub struct TextRenderer;

// Pre-computed shadow configurations
struct ShadowConfig {
    offsets: Vec<Vector2>,
    colors: Vec<Color>,
}

impl ShadowConfig {
    fn new() -> Self {
        let offsets = vec![
            Vector2::new(6.0, 6.0),
            Vector2::new(3.0, 3.0),
            Vector2::new(1.5, 1.5),
        ];

        let colors = vec![
            Color::new(0, 0, 0, 150),
            Color::new(0, 0, 0, 100),
            Color::new(0, 0, 0, 50),
        ];

        Self { offsets, colors }
    }
}

// Cache for text positioning calculations
struct TextCache {
    title_position: Vector2,
    title_size: f32,
    subtitle_position: Vector2,
    subtitle_size: f32,
    shadow_config: ShadowConfig,
    // Pre-computed title colors
    title_main_color: Color,
    title_highlight_color: Color,
    subtitle_shadow_color: Color,
    subtitle_main_color: Color,
}

impl TextCache {
    fn new() -> Self {
        Self {
            title_position: Vector2::new(600.0 - 160.0, 60.0),
            title_size: 120.0,
            subtitle_position: Vector2::new(600.0 - 140.0, 200.0),
            subtitle_size: 32.0,
            shadow_config: ShadowConfig::new(),
            title_main_color: Color::new(255, 215, 0, 255), // Gold
            title_highlight_color: Color::new(255, 255, 255, 200), // White highlight
            subtitle_shadow_color: Color::new(0, 0, 0, 80),
            subtitle_main_color: Color::new(200, 200, 255, 255),
        }
    }
}

// Thread-safe lazy static initialization
static TEXT_CACHE: LazyLock<TextCache> = LazyLock::new(TextCache::new);

impl TextRenderer {
    pub fn draw_title_with_shadow(d: &mut RaylibDrawHandle, title_font: &Font) {
        let cache = &*TEXT_CACHE;
        let title = "DropJack";

        // Draw shadow layers using pre-computed values
        for (offset, color) in cache
            .shadow_config
            .offsets
            .iter()
            .zip(cache.shadow_config.colors.iter())
        {
            d.draw_text_ex(
                title_font,
                title,
                Vector2::new(
                    cache.title_position.x + offset.x,
                    cache.title_position.y + offset.y,
                ),
                cache.title_size,
                2.0,
                *color,
            );
        }

        // Main title with gradient effect
        d.draw_text_ex(
            title_font,
            title,
            cache.title_position,
            cache.title_size,
            2.0,
            cache.title_main_color,
        );

        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(cache.title_position.x, cache.title_position.y - 1.0),
            cache.title_size,
            2.0,
            cache.title_highlight_color,
        );
    }

    pub fn draw_subtitle(d: &mut RaylibDrawHandle, font: &Font) {
        let cache = &*TEXT_CACHE;
        let subtitle = "A Strategic Card-Falling Puzzle";

        // Shadow
        d.draw_text_ex(
            font,
            subtitle,
            Vector2::new(
                cache.subtitle_position.x + 2.0,
                cache.subtitle_position.y + 2.0,
            ),
            cache.subtitle_size,
            1.0,
            cache.subtitle_shadow_color,
        );

        // Main text
        d.draw_text_ex(
            font,
            subtitle,
            cache.subtitle_position,
            cache.subtitle_size,
            1.0,
            cache.subtitle_main_color,
        );
    }
}
