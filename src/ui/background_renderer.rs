use crate::ui::drawing::{SCREEN_HEIGHT, SCREEN_WIDTH, BOARD_OFFSET_X, BOARD_OFFSET_Y};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};

pub struct BackgroundRenderer;

impl BackgroundRenderer {
    pub fn draw_gradient_background(d: &mut RaylibDrawHandle) {
        // Create a sophisticated atmospheric background with subtle variations
        let gradient_steps = 40;
        let step_height = SCREEN_HEIGHT / gradient_steps;

        for i in 0..gradient_steps {
            let ratio = i as f32 / gradient_steps as f32;
            // More sophisticated color transitions with subtle color shifts
            let r = (8.0 + ratio * 12.0 + (ratio * std::f32::consts::PI).sin() * 2.0) as u8;
            let g = (15.0 + ratio * 15.0 + (ratio * 2.1).sin() * 3.0) as u8;
            let b = (25.0 + ratio * 20.0 + (ratio * 1.7).sin() * 4.0) as u8;

            let color = Color::new(r, g, b, 255);
            d.draw_rectangle(0, i * step_height, SCREEN_WIDTH, step_height + 1, color);
        }

        // Add subtle atmospheric particles for ambiance
        for i in 0..25 {
            let x = (i * 127) % SCREEN_WIDTH;
            let y = (i * 211) % SCREEN_HEIGHT;
            let alpha = ((i * 17) % 35 + 10) as u8;
            let size = 0.3 + ((i * 13) % 7) as f32 * 0.1;
            d.draw_circle(x, y, size, Color::new(255, 255, 255, alpha));
        }
    }

    pub fn draw_game_board_background(d: &mut RaylibDrawHandle, board_width: i32, board_height: i32, cell_size: i32) {
        let board_pixel_width = board_width * cell_size;
        let board_pixel_height = board_height * cell_size;
        let center_x = BOARD_OFFSET_X + board_pixel_width / 2;
        let center_y = BOARD_OFFSET_Y + board_pixel_height / 2;
        
        // Enhanced decorative frame system with more depth
        // Outermost shadow
        d.draw_rectangle(
            BOARD_OFFSET_X - 12,
            BOARD_OFFSET_Y - 12,
            board_pixel_width + 24,
            board_pixel_height + 24,
            Color::new(0, 0, 0, 100),
        );
        
        // Outer dark wood frame
        d.draw_rectangle(
            BOARD_OFFSET_X - 10,
            BOARD_OFFSET_Y - 10,
            board_pixel_width + 20,
            board_pixel_height + 20,
            Color::new(80, 40, 20, 255),
        );
        
        // Middle wood frame with grain effect
        d.draw_rectangle(
            BOARD_OFFSET_X - 8,
            BOARD_OFFSET_Y - 8,
            board_pixel_width + 16,
            board_pixel_height + 16,
            Color::new(139, 69, 19, 255),
        );
        
        // Add wood grain lines for realism
        for i in 0..8 {
            let grain_offset = i * 2;
            d.draw_line(
                BOARD_OFFSET_X - 8 + grain_offset,
                BOARD_OFFSET_Y - 8,
                BOARD_OFFSET_X - 8 + grain_offset,
                BOARD_OFFSET_Y + board_pixel_height + 8,
                Color::new(110, 55, 15, 100),
            );
        }
        
        // Inner bevel frame
        d.draw_rectangle(
            BOARD_OFFSET_X - 6,
            BOARD_OFFSET_Y - 6,
            board_pixel_width + 12,
            board_pixel_height + 12,
            Color::new(160, 82, 45, 255),
        );
        
        // Innermost highlight frame
        d.draw_rectangle(
            BOARD_OFFSET_X - 4,
            BOARD_OFFSET_Y - 4,
            board_pixel_width + 8,
            board_pixel_height + 8,
            Color::new(210, 180, 140, 255),
        );

        // Create realistic radial lighting on green felt (like casino table lighting) - OPTIMIZED
        let max_radius = ((board_pixel_width * board_pixel_width + board_pixel_height * board_pixel_height) as f32).sqrt() / 2.0;
        
        // Use efficient overlapping rectangles for smooth gradient - NO GAPS
        let gradient_steps = 25; // Reduced for performance but still smooth
        let step_width = (board_pixel_width as f32 / gradient_steps as f32).ceil() as i32;
        let step_height = (board_pixel_height as f32 / gradient_steps as f32).ceil() as i32;
        
        for y in 0..gradient_steps {
            for x in 0..gradient_steps {
                let rect_x = BOARD_OFFSET_X + x * step_width;
                let rect_y = BOARD_OFFSET_Y + y * step_height;
                
                // Make rectangles overlap slightly to eliminate gaps
                let rect_width = if x == gradient_steps - 1 { 
                    board_pixel_width - x * step_width + 2 
                } else { 
                    step_width + 2 
                };
                let rect_height = if y == gradient_steps - 1 { 
                    board_pixel_height - y * step_height + 2 
                } else { 
                    step_height + 2 
                };
                
                // Calculate center of this rectangle for distance calculation
                let center_x_offset = (rect_x + rect_width / 2) - center_x;
                let center_y_offset = (rect_y + rect_height / 2) - center_y;
                let distance = ((center_x_offset * center_x_offset + center_y_offset * center_y_offset) as f32).sqrt();
                let distance_ratio = (distance / max_radius).min(1.0);
                let light_factor = 1.0 - (distance_ratio * distance_ratio * 0.6);
                
                // Rich green felt with subtle variations
                let x_ratio = x as f32 / gradient_steps as f32;
                let y_ratio = y as f32 / gradient_steps as f32;
                let base_r = 20.0 + y_ratio * 15.0;
                let base_g = 80.0 + x_ratio * 30.0;
                let base_b = 30.0 + (x_ratio + y_ratio) * 10.0;
                
                let r = (base_r * light_factor) as u8;
                let g = (base_g * light_factor + 10.0) as u8;
                let b = (base_b * light_factor) as u8;
                
                let color = Color::new(r, g, b, 255);
                d.draw_rectangle(rect_x, rect_y, rect_width, rect_height, color);
            }
        }
        
        // Add realistic felt texture with more sophisticated pattern
        for i in 0..120 {
            let x = BOARD_OFFSET_X + (i * 47) % board_pixel_width;
            let y = BOARD_OFFSET_Y + (i * 83) % board_pixel_height;
            
            // Distance from center affects texture visibility
            let dx = x - center_x;
            let dy = y - center_y;
            let distance_from_center = ((dx * dx + dy * dy) as f32).sqrt();
            let max_distance = (board_pixel_width / 2) as f32;
            let distance_ratio = (distance_from_center / max_distance).min(1.0);
            
            // Texture is more visible in lit areas, less in shadows
            let base_alpha = 25.0 * (1.0 - distance_ratio * 0.7);
            let alpha = ((i * 19) % 15 + base_alpha as i32) as u8;
            
            let size = 0.2 + ((i * 11) % 5) as f32 * 0.1;
            d.draw_circle(x, y, size, Color::new(255, 255, 255, alpha));
        }
        
        // Add subtle fabric weave pattern
        for i in 0..15 {
            let spacing = board_pixel_width / 15;
            let x = BOARD_OFFSET_X + i * spacing;
            for j in 0..3 {
                d.draw_line(
                    x + j,
                    BOARD_OFFSET_Y,
                    x + j,
                    BOARD_OFFSET_Y + board_pixel_height,
                    Color::new(0, 0, 0, (8 + j * 3) as u8),
                );
            }
        }
        
        for i in 0..12 {
            let spacing = board_pixel_height / 12;
            let y = BOARD_OFFSET_Y + i * spacing;
            for j in 0..3 {
                d.draw_line(
                    BOARD_OFFSET_X,
                    y + j,
                    BOARD_OFFSET_X + board_pixel_width,
                    y + j,
                    Color::new(0, 0, 0, (8 + j * 3) as u8),
                );
            }
        }
        
        // Enhanced grid lines with depth and lighting awareness
        for x in 0..=board_width {
            let line_x = BOARD_OFFSET_X + x * cell_size;
            let distance_from_center = (line_x - center_x).abs() as f32;
            let max_distance = (board_pixel_width / 2) as f32;
            let distance_ratio = distance_from_center / max_distance;
            
            // Grid lines are more visible in the center (lit area)
            let alpha = (50.0 * (1.0 - distance_ratio * 0.6)) as u8;
            
            d.draw_line(
                line_x,
                BOARD_OFFSET_Y,
                line_x,
                BOARD_OFFSET_Y + board_pixel_height,
                Color::new(0, 0, 0, alpha),
            );
        }
        
        for y in 0..=board_height {
            let line_y = BOARD_OFFSET_Y + y * cell_size;
            let distance_from_center = (line_y - center_y).abs() as f32;
            let max_distance = (board_pixel_height / 2) as f32;
            let distance_ratio = distance_from_center / max_distance;
            
            // Grid lines are more visible in the center (lit area)
            let alpha = (50.0 * (1.0 - distance_ratio * 0.6)) as u8;
            
            d.draw_line(
                BOARD_OFFSET_X,
                line_y,
                BOARD_OFFSET_X + board_pixel_width,
                line_y,
                Color::new(0, 0, 0, alpha),
            );
        }
        
        // Add subtle corner accent lighting
        let corner_glow_size = 30;
        for corner in 0..4 {
            let (corner_x, corner_y) = match corner {
                0 => (BOARD_OFFSET_X, BOARD_OFFSET_Y), // Top-left
                1 => (BOARD_OFFSET_X + board_pixel_width, BOARD_OFFSET_Y), // Top-right
                2 => (BOARD_OFFSET_X, BOARD_OFFSET_Y + board_pixel_height), // Bottom-left
                _ => (BOARD_OFFSET_X + board_pixel_width, BOARD_OFFSET_Y + board_pixel_height), // Bottom-right
            };
            
            for i in 0..corner_glow_size {
                let alpha = 15 - i / 2;
                if alpha > 0 {
                    d.draw_circle(corner_x, corner_y, i as f32, Color::new(255, 255, 200, alpha as u8));
                }
            }
        }
    }
} 