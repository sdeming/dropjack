use crate::models::{Card, CardColor, Particle};
use crate::ui::config::ParticleConfig;
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

pub struct ParticleSystemBuilder {
    particle_capacity: usize,
    explosion_particle_count: usize,
    sparkle_count: usize,
    explosion_base_speeds: Vec<f32>,
    explosion_colors: [Color; 4],
}

impl ParticleSystemBuilder {
    pub fn new() -> Self {
        Self {
            particle_capacity: ParticleConfig::SYSTEM_CAPACITY,
            explosion_particle_count: ParticleConfig::EXPLOSION_COUNT,
            sparkle_count: ParticleConfig::SPARKLE_COUNT,
            explosion_base_speeds: ParticleConfig::EXPLOSION_SPEEDS.to_vec(),
            explosion_colors: ParticleConfig::COLORS,
        }
    }

    pub fn particle_capacity(mut self, capacity: usize) -> Self {
        self.particle_capacity = capacity;
        self
    }

    pub fn explosion_particle_count(mut self, count: usize) -> Self {
        self.explosion_particle_count = count;
        self
    }

    // Removed unused builder methods - sparkle_count, explosion_base_speeds, explosion_colors
    // These can be added back if needed for future customization

    pub fn build(self) -> ParticleSystem {
        // Pre-compute explosion velocity patterns for reuse
        let explosion_velocities: Vec<Vector2> = (0..self.explosion_particle_count)
            .map(|i| {
                let wave = i / ParticleConfig::WAVE_SIZE;
                let base_speed = self
                    .explosion_base_speeds
                    .get(wave)
                    .copied()
                    .unwrap_or(100.0);

                let angle = (i as f32 / (self.explosion_particle_count / 4) as f32)
                    * 2.0
                    * std::f32::consts::PI;
                let speed_variation = 0.5 + (i as f32 / self.explosion_particle_count as f32);
                let final_speed = base_speed * speed_variation;

                Vector2::new(angle.cos() * final_speed, angle.sin() * final_speed)
            })
            .collect();

        // Pre-compute sparkle velocities
        let sparkle_velocities: Vec<Vector2> = (0..self.sparkle_count)
            .map(|i| {
                let angle = (i as f32 / self.sparkle_count as f32) * 2.0 * std::f32::consts::PI;
                Vector2::new(
                    angle.cos() * ParticleConfig::SPARKLE_SPEED,
                    angle.sin() * ParticleConfig::SPARKLE_SPEED
                        + ParticleConfig::SPARKLE_UPWARD_BIAS,
                )
            })
            .collect();

        ParticleSystem {
            particles: Vec::new(),
            particle_pool: Vec::with_capacity(self.particle_capacity),
            explosion_velocities,
            explosion_colors: self.explosion_colors,
            sparkle_velocities,
        }
    }
}

impl ParticleSystem {
    pub fn builder() -> ParticleSystemBuilder {
        ParticleSystemBuilder::new()
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
            CardColor::Black => ParticleConfig::COLOR_BLACK,
        };

        // Generate particles using pre-computed patterns
        let total_particles = self.explosion_velocities.len();

        for i in 0..total_particles {
            let wave = i / ParticleConfig::WAVE_SIZE;
            let (life_time, particle_size) = match wave {
                0 => (ParticleConfig::LIFE_TIMES[0], ParticleConfig::SIZES[0]),
                1 => (ParticleConfig::LIFE_TIMES[1], ParticleConfig::SIZES[1]),
                2 => (ParticleConfig::LIFE_TIMES[2], ParticleConfig::SIZES[2]),
                _ => (ParticleConfig::LIFE_TIMES[3], ParticleConfig::SIZES[3]),
            };

            // Use pre-computed velocity with small variations
            let base_velocity = self.explosion_velocities[i];
            let velocity_variation = Vector2::new(
                (i % 7) as f32 * 8.6 - ParticleConfig::VELOCITY_VARIATION_RANGE,
                (i % 5) as f32 * 12.0 - ParticleConfig::VELOCITY_VARIATION_RANGE,
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

            let final_life_time = life_time + (i % 10) as f32 * ParticleConfig::LIFE_TIME_VARIATION;

            let particle_pos = Vector2::new(
                position.x + ((i % 7) as f32 - 3.0) * size * 0.1, // Deterministic spread
                position.y + ((i % 5) as f32 - 2.0) * size * 0.1,
            );

            // Create particle using builder pattern for consistency
            let particle = if let Some(_reused_particle) = self.particle_pool.pop() {
                // Even when reusing, use builder for clean, consistent configuration
                Particle::builder(particle_pos, velocity, color, final_life_time)
                    .size(particle_size)
                    .acceleration(Vector2::new(0.0, ParticleConfig::ACCELERATION_Y))
                    .angular_velocity(
                        ((i % 7) as f32 - 3.0) * ParticleConfig::ANGULAR_VELOCITY_RANGE,
                    )
                    .build()
            } else {
                // Create new particle using builder
                Particle::builder(particle_pos, velocity, color, final_life_time)
                    .size(particle_size)
                    .angular_velocity(
                        ((i % 7) as f32 - 3.0) * ParticleConfig::ANGULAR_VELOCITY_RANGE,
                    )
                    .build()
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

            // Create sparkle using builder pattern for consistency
            let sparkle = if let Some(_reused_particle) = self.particle_pool.pop() {
                // Even when reusing, use builder for clean, consistent configuration
                Particle::builder(
                    sparkle_pos,
                    sparkle_velocity,
                    ParticleConfig::COLOR_YELLOW,
                    ParticleConfig::SPARKLE_LIFE,
                )
                .size(ParticleConfig::SPARKLE_SIZE)
                .acceleration(Vector2::new(0.0, ParticleConfig::SPARKLE_ACCELERATION_Y))
                .angular_velocity(
                    i as f32 * ParticleConfig::SPARKLE_ANGULAR_VELOCITY_MULTIPLIER
                        - ParticleConfig::SPARKLE_ANGULAR_VELOCITY_OFFSET,
                )
                .build()
            } else {
                Particle::builder(
                    sparkle_pos,
                    sparkle_velocity,
                    ParticleConfig::COLOR_YELLOW,
                    ParticleConfig::SPARKLE_LIFE,
                )
                .size(ParticleConfig::SPARKLE_SIZE)
                .acceleration(Vector2::new(0.0, ParticleConfig::SPARKLE_ACCELERATION_Y))
                .angular_velocity(
                    i as f32 * ParticleConfig::SPARKLE_ANGULAR_VELOCITY_MULTIPLIER
                        - ParticleConfig::SPARKLE_ANGULAR_VELOCITY_OFFSET,
                )
                .build()
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
