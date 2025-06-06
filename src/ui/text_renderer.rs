use crate::ui::config::TextConfig;
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
            TextConfig::SHADOW_OFFSET_1,
            TextConfig::SHADOW_OFFSET_2,
            TextConfig::SHADOW_OFFSET_3,
        ];

        let colors = vec![
            TextConfig::SHADOW_COLOR_1,
            TextConfig::SHADOW_COLOR_2,
            TextConfig::SHADOW_COLOR_3,
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
            title_position: Vector2::new(600.0 - TextConfig::TITLE_X_OFFSET, TextConfig::TITLE_Y),
            title_size: TextConfig::TITLE_SIZE,
            subtitle_position: Vector2::new(
                600.0 - TextConfig::SUBTITLE_X_OFFSET,
                TextConfig::SUBTITLE_Y,
            ),
            subtitle_size: TextConfig::SUBTITLE_SIZE,
            shadow_config: ShadowConfig::new(),
            title_main_color: TextConfig::TITLE_MAIN_COLOR,
            title_highlight_color: TextConfig::TITLE_HIGHLIGHT_COLOR,
            subtitle_shadow_color: TextConfig::SUBTITLE_SHADOW_COLOR,
            subtitle_main_color: TextConfig::SUBTITLE_MAIN_COLOR,
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
            TextConfig::TITLE_SPACING,
            cache.title_main_color,
        );

        d.draw_text_ex(
            title_font,
            title,
            Vector2::new(cache.title_position.x, cache.title_position.y - 1.0),
            cache.title_size,
            TextConfig::TITLE_SPACING,
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
                cache.subtitle_position.x + TextConfig::SHADOW_OFFSET_SUBTITLE.x,
                cache.subtitle_position.y + TextConfig::SHADOW_OFFSET_SUBTITLE.y,
            ),
            cache.subtitle_size,
            TextConfig::SUBTITLE_SPACING,
            cache.subtitle_shadow_color,
        );

        // Main text
        d.draw_text_ex(
            font,
            subtitle,
            cache.subtitle_position,
            cache.subtitle_size,
            TextConfig::SUBTITLE_SPACING,
            cache.subtitle_main_color,
        );
    }
}
