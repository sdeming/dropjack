use crate::ui::config::ScreenConfig;
use crate::ui::config::{BackgroundConfig, BoardConfig};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use std::sync::LazyLock;

pub struct BackgroundRenderer;

// Pre-computed values for gradient backgrounds
struct GradientCache {
    gradient_steps: i32,
    step_height: i32,
    gradient_colors: Vec<Color>,
    particle_positions: Vec<(i32, i32)>,
    particle_alphas: Vec<u8>,
    particle_sizes: Vec<f32>,
}

impl GradientCache {
    fn new() -> Self {
        let gradient_steps = BackgroundConfig::GRADIENT_STEPS;
        let step_height = ScreenConfig::HEIGHT / gradient_steps;

        // Pre-compute all ratios and colors
        let step_ratios: Vec<f32> = (0..gradient_steps)
            .map(|i| i as f32 / gradient_steps as f32)
            .collect();

        let gradient_colors: Vec<Color> = step_ratios
            .iter()
            .map(|&ratio| {
                let r = (BackgroundConfig::GRADIENT_R_BASE
                    + ratio * BackgroundConfig::GRADIENT_R_RANGE
                    + (ratio * std::f32::consts::PI).sin()
                        * BackgroundConfig::GRADIENT_R_SIN_MULTIPLIER)
                    as u8;
                let g = (BackgroundConfig::GRADIENT_G_BASE
                    + ratio * BackgroundConfig::GRADIENT_G_RANGE
                    + (ratio * BackgroundConfig::GRADIENT_G_SIN_FREQUENCY).sin()
                        * BackgroundConfig::GRADIENT_G_SIN_MULTIPLIER)
                    as u8;
                let b = (BackgroundConfig::GRADIENT_B_BASE
                    + ratio * BackgroundConfig::GRADIENT_B_RANGE
                    + (ratio * BackgroundConfig::GRADIENT_B_SIN_FREQUENCY).sin()
                        * BackgroundConfig::GRADIENT_B_SIN_MULTIPLIER)
                    as u8;
                Color::new(r, g, b, 255)
            })
            .collect();

        // Pre-compute particle positions and properties
        let particle_positions: Vec<(i32, i32)> = (0..BackgroundConfig::PARTICLE_COUNT)
            .map(|i| {
                let x = (i * 127) % ScreenConfig::WIDTH;
                let y = (i * 211) % ScreenConfig::HEIGHT;
                (x, y)
            })
            .collect();

        let particle_alphas: Vec<u8> = (0..BackgroundConfig::PARTICLE_COUNT)
            .map(|i| {
                ((i * 17) % BackgroundConfig::PARTICLE_ALPHA_RANGE
                    + BackgroundConfig::PARTICLE_ALPHA_BASE) as u8
            })
            .collect();

        let particle_sizes: Vec<f32> = (0..BackgroundConfig::PARTICLE_COUNT)
            .map(|i| {
                BackgroundConfig::PARTICLE_SIZE_BASE
                    + ((i * 13) % BackgroundConfig::PARTICLE_SIZE_MULTIPLIER) as f32
                        * BackgroundConfig::PARTICLE_SIZE_RANGE
            })
            .collect();

        Self {
            gradient_steps,
            step_height,
            gradient_colors,
            particle_positions,
            particle_alphas,
            particle_sizes,
        }
    }
}

// Cache for board background calculations
struct BoardCache {
    gradient_steps: i32,
    x_ratios: Vec<f32>,
    y_ratios: Vec<f32>,
    texture_coords: Vec<(i32, i32)>,
    texture_alphas: Vec<u8>,
    texture_sizes: Vec<f32>,
}

impl BoardCache {
    fn new() -> Self {
        let gradient_steps = BoardConfig::GRADIENT_STEPS;

        // Pre-compute ratios for x and y
        let x_ratios: Vec<f32> = (0..gradient_steps)
            .map(|x| x as f32 / gradient_steps as f32)
            .collect();
        let y_ratios: Vec<f32> = (0..gradient_steps)
            .map(|y| y as f32 / gradient_steps as f32)
            .collect();

        // Pre-compute texture coordinates and properties
        let texture_coords: Vec<(i32, i32)> = (0..BoardConfig::TEXTURE_COUNT)
            .map(|i| (i * 47, i * 83))
            .collect();

        let texture_alphas: Vec<u8> = (0..BoardConfig::TEXTURE_COUNT)
            .map(|i| ((i * 19) % 15 + 25) as u8)
            .collect();

        let texture_sizes: Vec<f32> = (0..BoardConfig::TEXTURE_COUNT)
            .map(|i| 0.2 + ((i * 11) % 5) as f32 * 0.1)
            .collect();

        Self {
            gradient_steps,
            x_ratios,
            y_ratios,
            texture_coords,
            texture_alphas,
            texture_sizes,
        }
    }
}

// Thread-safe lazy static initialization
static GRADIENT_CACHE: LazyLock<GradientCache> = LazyLock::new(GradientCache::new);
static BOARD_CACHE: LazyLock<BoardCache> = LazyLock::new(BoardCache::new);

impl BackgroundRenderer {
    pub fn draw_gradient_background(d: &mut RaylibDrawHandle) {
        let cache = &*GRADIENT_CACHE;

        // Use pre-computed colors and ratios
        for i in 0..cache.gradient_steps {
            let color = cache.gradient_colors[i as usize];
            d.draw_rectangle(
                0,
                i * cache.step_height,
                ScreenConfig::WIDTH,
                cache.step_height + 1,
                color,
            );
        }

        // Use pre-computed particle properties
        for i in 0..BackgroundConfig::PARTICLE_COUNT as usize {
            let (x, y) = cache.particle_positions[i];
            let alpha = cache.particle_alphas[i];
            let size = cache.particle_sizes[i];
            d.draw_circle(x, y, size, Color::new(255, 255, 255, alpha));
        }
    }

    pub fn draw_game_board_background(
        d: &mut RaylibDrawHandle,
        board_width: i32,
        board_height: i32,
        cell_size: i32,
    ) {
        let board_pixel_width = board_width * cell_size;
        let board_pixel_height = board_height * cell_size;
        let center_x = BoardConfig::OFFSET_X + board_pixel_width / 2;
        let center_y = BoardConfig::OFFSET_Y + board_pixel_height / 2;

        // Enhanced decorative frame system with more depth
        // Outermost shadow
        d.draw_rectangle(
            BoardConfig::OFFSET_X - BoardConfig::SHADOW_SIZE / 2,
            BoardConfig::OFFSET_Y - BoardConfig::SHADOW_SIZE / 2,
            board_pixel_width + BoardConfig::SHADOW_SIZE,
            board_pixel_height + BoardConfig::SHADOW_SIZE,
            BoardConfig::SHADOW_COLOR,
        );

        // Outer dark wood frame
        d.draw_rectangle(
            BoardConfig::OFFSET_X - BoardConfig::OUTER_FRAME_OFFSET,
            BoardConfig::OFFSET_Y - BoardConfig::OUTER_FRAME_OFFSET,
            board_pixel_width + BoardConfig::OUTER_FRAME_SIZE,
            board_pixel_height + BoardConfig::OUTER_FRAME_SIZE,
            BoardConfig::OUTER_FRAME_COLOR,
        );

        // Middle wood frame with grain effect
        d.draw_rectangle(
            BoardConfig::OFFSET_X - BoardConfig::MIDDLE_FRAME_OFFSET,
            BoardConfig::OFFSET_Y - BoardConfig::MIDDLE_FRAME_OFFSET,
            board_pixel_width + BoardConfig::MIDDLE_FRAME_SIZE,
            board_pixel_height + BoardConfig::MIDDLE_FRAME_SIZE,
            BoardConfig::MIDDLE_FRAME_COLOR,
        );

        // Add wood grain lines for realism
        for i in 0..BoardConfig::GRAIN_LINES {
            let grain_offset = i * BoardConfig::GRAIN_SPACING;
            d.draw_line(
                BoardConfig::OFFSET_X - BoardConfig::MIDDLE_FRAME_OFFSET + grain_offset,
                BoardConfig::OFFSET_Y - BoardConfig::MIDDLE_FRAME_OFFSET,
                BoardConfig::OFFSET_X - BoardConfig::MIDDLE_FRAME_OFFSET + grain_offset,
                BoardConfig::OFFSET_Y + board_pixel_height + BoardConfig::MIDDLE_FRAME_OFFSET,
                BoardConfig::GRAIN_COLOR,
            );
        }

        // Inner bevel frame
        d.draw_rectangle(
            BoardConfig::OFFSET_X - BoardConfig::INNER_FRAME_OFFSET,
            BoardConfig::OFFSET_Y - BoardConfig::INNER_FRAME_OFFSET,
            board_pixel_width + BoardConfig::INNER_FRAME_SIZE,
            board_pixel_height + BoardConfig::INNER_FRAME_SIZE,
            BoardConfig::INNER_FRAME_COLOR,
        );

        // Innermost highlight frame
        d.draw_rectangle(
            BoardConfig::OFFSET_X - BoardConfig::HIGHLIGHT_FRAME_OFFSET,
            BoardConfig::OFFSET_Y - BoardConfig::HIGHLIGHT_FRAME_OFFSET,
            board_pixel_width + BoardConfig::HIGHLIGHT_FRAME_SIZE,
            board_pixel_height + BoardConfig::HIGHLIGHT_FRAME_SIZE,
            BoardConfig::HIGHLIGHT_FRAME_COLOR,
        );

        let cache = &*BOARD_CACHE;

        // Create realistic radial lighting on green felt (like casino table lighting) - OPTIMIZED
        let max_radius = ((board_pixel_width * board_pixel_width
            + board_pixel_height * board_pixel_height) as f32)
            .sqrt()
            / 2.0;
        let max_radius_squared = max_radius * max_radius;

        // Use efficient overlapping rectangles for smooth gradient - NO GAPS
        let step_width = (board_pixel_width as f32 / cache.gradient_steps as f32).ceil() as i32;
        let step_height = (board_pixel_height as f32 / cache.gradient_steps as f32).ceil() as i32;

        // Pre-compute base colors for each position
        let mut base_colors =
            Vec::with_capacity((cache.gradient_steps * cache.gradient_steps) as usize);

        for y in 0..cache.gradient_steps {
            for x in 0..cache.gradient_steps {
                let x_ratio = cache.x_ratios[x as usize];
                let y_ratio = cache.y_ratios[y as usize];

                let base_r = 20.0 + y_ratio * 15.0;
                let base_g = 80.0 + x_ratio * 30.0;
                let base_b = 30.0 + (x_ratio + y_ratio) * 10.0;

                base_colors.push((base_r, base_g, base_b));
            }
        }

        // Now render with pre-computed values
        for y in 0..cache.gradient_steps {
            for x in 0..cache.gradient_steps {
                let rect_x = BoardConfig::OFFSET_X + x * step_width;
                let rect_y = BoardConfig::OFFSET_Y + y * step_height;

                // Make rectangles overlap slightly to eliminate gaps
                let rect_width = if x == cache.gradient_steps - 1 {
                    board_pixel_width - x * step_width + 2
                } else {
                    step_width + 2
                };
                let rect_height = if y == cache.gradient_steps - 1 {
                    board_pixel_height - y * step_height + 2
                } else {
                    step_height + 2
                };

                // Calculate the center of this rectangle for distance calculation
                let center_x_offset = (rect_x + rect_width / 2) - center_x;
                let center_y_offset = (rect_y + rect_height / 2) - center_y;
                let distance_squared =
                    (center_x_offset * center_x_offset + center_y_offset * center_y_offset) as f32;
                let distance_ratio = (distance_squared / max_radius_squared).min(1.0);
                let light_factor = 1.0 - (distance_ratio * 0.6);

                // Use pre-computed base color
                let color_index = (y * cache.gradient_steps + x) as usize;
                let (base_r, base_g, base_b) = base_colors[color_index];

                let r = (base_r * light_factor) as u8;
                let g = (base_g * light_factor + 10.0) as u8;
                let b = (base_b * light_factor) as u8;

                let color = Color::new(r, g, b, 255);
                d.draw_rectangle(rect_x, rect_y, rect_width, rect_height, color);
            }
        }

        // Add realistic felt texture with a more sophisticated pattern-OPTIMIZED
        let max_distance = (board_pixel_width / 2) as f32;
        let max_distance_squared = max_distance * max_distance;

        for i in 0..BoardConfig::TEXTURE_COUNT as usize {
            let (x_offset, y_offset) = cache.texture_coords[i];
            let x = BoardConfig::OFFSET_X + x_offset % board_pixel_width;
            let y = BoardConfig::OFFSET_Y + y_offset % board_pixel_height;

            // Distance from the center affects texture visibility - optimized calculation
            let dx = x - center_x;
            let dy = y - center_y;
            let distance_squared = (dx * dx + dy * dy) as f32;
            let distance_ratio = (distance_squared / max_distance_squared).min(1.0);

            // Texture is more visible in lit areas, less in shadows
            let base_alpha = 25.0 * (1.0 - distance_ratio * 0.7);
            let alpha = (cache.texture_alphas[i] as f32 + base_alpha) as u8;

            let size = cache.texture_sizes[i];
            d.draw_circle(x, y, size, Color::new(255, 255, 255, alpha));
        }

        // Add a subtle fabric weave pattern
        for i in 0..BackgroundConfig::VERTICAL_WEAVE_LINES {
            let spacing = board_pixel_width / BackgroundConfig::VERTICAL_WEAVE_LINES;
            let x = BoardConfig::OFFSET_X + i * spacing;
            for j in 0..BackgroundConfig::WEAVE_LINE_VARIATIONS {
                d.draw_line(
                    x + j,
                    BoardConfig::OFFSET_Y,
                    x + j,
                    BoardConfig::OFFSET_Y + board_pixel_height,
                    Color::new(
                        0,
                        0,
                        0,
                        (BackgroundConfig::WEAVE_BASE_ALPHA
                            + j * BackgroundConfig::WEAVE_ALPHA_STEP) as u8,
                    ),
                );
            }
        }

        for i in 0..BackgroundConfig::HORIZONTAL_WEAVE_LINES {
            let spacing = board_pixel_height / BackgroundConfig::HORIZONTAL_WEAVE_LINES;
            let y = BoardConfig::OFFSET_Y + i * spacing;
            for j in 0..BackgroundConfig::WEAVE_LINE_VARIATIONS {
                d.draw_line(
                    BoardConfig::OFFSET_X,
                    y + j,
                    BoardConfig::OFFSET_X + board_pixel_width,
                    y + j,
                    Color::new(
                        0,
                        0,
                        0,
                        (BackgroundConfig::WEAVE_BASE_ALPHA
                            + j * BackgroundConfig::WEAVE_ALPHA_STEP) as u8,
                    ),
                );
            }
        }

        // Enhanced grid lines with depth and lighting awareness - OPTIMIZED
        let max_width_distance = (board_pixel_width / 2) as f32;
        let max_height_distance = (board_pixel_height / 2) as f32;

        for x in 0..=board_width {
            let line_x = BoardConfig::OFFSET_X + x * cell_size;
            let distance_from_center = (line_x - center_x).abs() as f32;
            let distance_ratio = distance_from_center / max_width_distance;

            // Grid lines are more visible in the center (lit area)
            let alpha = (50.0 * (1.0 - distance_ratio * 0.6)) as u8;

            d.draw_line(
                line_x,
                BoardConfig::OFFSET_Y,
                line_x,
                BoardConfig::OFFSET_Y + board_pixel_height,
                Color::new(0, 0, 0, alpha),
            );
        }

        for y in 0..=board_height {
            let line_y = BoardConfig::OFFSET_Y + y * cell_size;
            let distance_from_center = (line_y - center_y).abs() as f32;
            let distance_ratio = distance_from_center / max_height_distance;

            // Grid lines are more visible in the center (lit area)
            let alpha = (50.0 * (1.0 - distance_ratio * 0.6)) as u8;

            d.draw_line(
                BoardConfig::OFFSET_X,
                line_y,
                BoardConfig::OFFSET_X + board_pixel_width,
                line_y,
                Color::new(0, 0, 0, alpha),
            );
        }

        // Add subtle corner accent lighting
        let corner_glow_size = 30;
        for corner in 0..4 {
            let (corner_x, corner_y) = match corner {
                0 => (BoardConfig::OFFSET_X, BoardConfig::OFFSET_Y), // Top-left
                1 => (
                    BoardConfig::OFFSET_X + board_pixel_width,
                    BoardConfig::OFFSET_Y,
                ), // Top-right
                2 => (
                    BoardConfig::OFFSET_X,
                    BoardConfig::OFFSET_Y + board_pixel_height,
                ), // Bottom-left
                _ => (
                    BoardConfig::OFFSET_X + board_pixel_width,
                    BoardConfig::OFFSET_Y + board_pixel_height,
                ), // Bottom-right
            };

            for i in 0..corner_glow_size {
                let alpha = 15 - i / 2;
                if alpha > 0 {
                    d.draw_circle(
                        corner_x,
                        corner_y,
                        i as f32,
                        Color::new(255, 255, 200, alpha as u8),
                    );
                }
            }
        }
    }
}
