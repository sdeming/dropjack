use raylib::prelude::*;

// Particle system for card explosion effects
#[derive(Debug, Clone)]
pub struct Particle {
    pub position: Vector2,
    pub velocity: Vector2,
    pub acceleration: Vector2,
    pub life_time: f32,
    pub max_life_time: f32,
    pub color: Color,
    pub size: f32,
    pub rotation: f32,
    pub angular_velocity: f32,
}

impl Particle {
    pub fn new(position: Vector2, velocity: Vector2, color: Color, life_time: f32) -> Self {
        Self {
            position,
            velocity,
            acceleration: Vector2::new(0.0, 200.0), // Gravity
            life_time,
            max_life_time: life_time,
            color,
            size: 2.0,
            rotation: 0.0,
            angular_velocity: (rand::random::<f32>() - 0.5) * 10.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> bool {
        // Update physics
        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;
        self.rotation += self.angular_velocity * delta_time;

        // Update lifetime
        self.life_time -= delta_time;

        // Fade out over time
        let alpha_ratio = self.life_time / self.max_life_time;
        self.color.a = (255.0 * alpha_ratio.max(0.0)) as u8;

        // Return true if particle is still alive
        self.life_time > 0.0
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        if self.life_time > 0.0 {
            // Draw a subtle glow effect for larger particles
            if self.size > 2.5 {
                let glow_color = Color::new(
                    self.color.r,
                    self.color.g,
                    self.color.b,
                    (self.color.a as f32 * 0.3) as u8,
                );
                d.draw_circle_v(self.position, self.size + 1.0, glow_color);
            }

            // Draw the main particle
            d.draw_circle_v(self.position, self.size, self.color);

            // Add a bright center for sparkle effect
            if self.color == Color::YELLOW && self.size < 2.0 {
                d.draw_circle_v(self.position, self.size * 0.5, Color::WHITE);
            }
        }
    }
}
