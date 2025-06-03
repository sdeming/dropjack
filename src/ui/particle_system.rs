use crate::models::{Card, CardColor, Particle};
use raylib::prelude::*;

pub struct ParticleSystem {
    particles: Vec<Particle>,
    // Pre-allocated particle pool to reuse particles
    particle_pool: Vec<Particle>,
    // Pre-computed explosion patterns
    explosion_velocities: Vec<Vector2>,
    explosion_colors: [Color; 4],
    sparkle_velocities: Vec<Vector2>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        // Pre-compute explosion velocity patterns for reuse
        let total_particles = 35;
        let explosion_velocities: Vec<Vector2> = (0..total_particles)
            .map(|i| {
                let wave = i / 12;
                let base_speed = match wave {
                    0 => 80.0,
                    1 => 60.0,
                    2 => 40.0,
                    _ => 100.0,
                };

                let angle = (i as f32 / (total_particles / 4) as f32) * 2.0 * std::f32::consts::PI;
                let speed_variation = 0.5 + (i as f32 / total_particles as f32); // Deterministic instead of random
                let final_speed = base_speed * speed_variation;

                Vector2::new(angle.cos() * final_speed, angle.sin() * final_speed)
            })
            .collect();

        // Pre-compute sparkle velocities
        let sparkle_velocities: Vec<Vector2> = (0..8)
            .map(|i| {
                let angle = (i as f32 / 8.0) * 2.0 * std::f32::consts::PI;
                Vector2::new(
                    angle.cos() * 20.0,
                    angle.sin() * 20.0 - 30.0, // Upward bias
                )
            })
            .collect();

        // Pre-define secondary colors
        let explosion_colors = [Color::WHITE, Color::YELLOW, Color::ORANGE, Color::LIGHTGRAY];

        Self {
            particles: Vec::new(),
            particle_pool: Vec::with_capacity(100), // Pre-allocate space for reuse
            explosion_velocities,
            explosion_colors,
            sparkle_velocities,
        }
    }

    pub fn add_card_explosion(
        &mut self,
        card: Card,
        position: Vector2,
        size: f32,
        _atlas: &Option<Texture2D>,
    ) {
        // Create explosion effect based on card colors
        let primary_color = match card.suit.color() {
            CardColor::Red => Color::RED,
            CardColor::Black => Color::new(30, 30, 30, 255),
        };

        // Generate particles using pre-computed patterns
        let total_particles = self.explosion_velocities.len();

        for i in 0..total_particles {
            let wave = i / 12;
            let (life_time, particle_size) = match wave {
                0 => (1.0, 3.0),
                1 => (1.2, 2.5),
                2 => (1.5, 2.0),
                _ => (0.8, 4.0),
            };

            // Use pre-computed velocity with small variations
            let base_velocity = self.explosion_velocities[i];
            let velocity_variation = Vector2::new(
                (i % 7) as f32 * 8.6 - 30.0, // Deterministic variation
                (i % 5) as f32 * 12.0 - 30.0,
            );
            let velocity = Vector2::new(
                base_velocity.x + velocity_variation.x,
                base_velocity.y + velocity_variation.y,
            );

            // Choose color based on particle index
            let color = if i % 4 == 0 {
                primary_color
            } else {
                self.explosion_colors[i % self.explosion_colors.len()]
            };

            let final_life_time = life_time + (i % 10) as f32 * 0.05; // Deterministic variation

            let particle_pos = Vector2::new(
                position.x + ((i % 7) as f32 - 3.0) * size * 0.1, // Deterministic spread
                position.y + ((i % 5) as f32 - 2.0) * size * 0.1,
            );

            // Try to reuse particle from pool instead of allocating new one
            let particle = if let Some(mut reused_particle) = self.particle_pool.pop() {
                // Reset reused particle
                reused_particle.position = particle_pos;
                reused_particle.velocity = velocity;
                reused_particle.color = color;
                reused_particle.life_time = final_life_time;
                reused_particle.max_life_time = final_life_time;
                reused_particle.size = particle_size;
                reused_particle.acceleration = Vector2::new(0.0, 200.0);
                reused_particle.rotation = 0.0;
                reused_particle.angular_velocity = ((i % 7) as f32 - 3.0) * 3.0; // Deterministic rotation
                reused_particle
            } else {
                // Create new particle only if pool is empty
                let mut particle = Particle::new(particle_pos, velocity, color, final_life_time);
                particle.size = particle_size;
                particle.angular_velocity = ((i % 7) as f32 - 3.0) * 3.0;
                particle
            };

            self.particles.push(particle);
        }

        // Add sparkle effects using pre-computed velocities
        for i in 0..self.sparkle_velocities.len() {
            let sparkle_velocity = self.sparkle_velocities[i];
            let sparkle_pos = Vector2::new(
                position.x + ((i % 3) as f32 - 1.0) * size * 0.25, // Deterministic spread
                position.y + ((i % 3) as f32 - 1.0) * size * 0.25,
            );

            // Try to reuse sparkle particle from pool
            let sparkle = if let Some(mut reused_particle) = self.particle_pool.pop() {
                // Reset reused particle for sparkle
                reused_particle.position = sparkle_pos;
                reused_particle.velocity = sparkle_velocity;
                reused_particle.color = Color::YELLOW;
                reused_particle.life_time = 0.6;
                reused_particle.max_life_time = 0.6;
                reused_particle.size = 1.5;
                reused_particle.acceleration = Vector2::new(0.0, 150.0);
                reused_particle.rotation = 0.0;
                reused_particle.angular_velocity = i as f32 * 2.0 - 8.0;
                reused_particle
            } else {
                let mut sparkle = Particle::new(sparkle_pos, sparkle_velocity, Color::YELLOW, 0.6);
                sparkle.size = 1.5;
                sparkle.acceleration = Vector2::new(0.0, 150.0);
                sparkle.angular_velocity = i as f32 * 2.0 - 8.0;
                sparkle
            };

            self.particles.push(sparkle);
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update all particles and collect dead ones for reuse
        let mut i = 0;
        while i < self.particles.len() {
            if self.particles[i].update(delta_time) {
                i += 1;
            } else {
                // Move dead particle to pool for reuse instead of dropping it
                let dead_particle = self.particles.swap_remove(i);
                if self.particle_pool.len() < self.particle_pool.capacity() {
                    self.particle_pool.push(dead_particle);
                }
                // Don't increment i since we removed an element
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for particle in &self.particles {
            particle.draw(d);
        }
    }
}
