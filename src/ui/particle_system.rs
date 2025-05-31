use super::particle::Particle;
use crate::cards::{Card, Color as CardColor};
use raylib::prelude::*;

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
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
            CardColor::Black => Color::new(30, 30, 30, 255), // Dark gray instead of pure black
        };

        let secondary_colors = [Color::WHITE, Color::YELLOW, Color::ORANGE, Color::LIGHTGRAY];

        // Generate multiple waves of particles for a more dramatic effect
        let total_particles = 35;

        for i in 0..total_particles {
            // Create varied explosion patterns
            let wave = i / 12; // Different waves have different behaviors

            let (base_speed, life_time, particle_size) = match wave {
                0 => (80.0, 1.0, 3.0),  // Fast, short-lived, small particles
                1 => (60.0, 1.2, 2.5),  // Medium speed, medium life
                2 => (40.0, 1.5, 2.0),  // Slow, long-lived, small particles
                _ => (100.0, 0.8, 4.0), // Very fast burst
            };

            let angle = (i as f32 / (total_particles / 4) as f32) * 2.0 * std::f32::consts::PI;
            let speed_variation = 0.5 + rand::random::<f32>() * 1.0;
            let final_speed = base_speed * speed_variation;

            let velocity = Vector2::new(
                angle.cos() * final_speed + (rand::random::<f32>() - 0.5) * 60.0,
                angle.sin() * final_speed + (rand::random::<f32>() - 0.5) * 60.0,
            );

            // Choose color based on particle index
            let color = if i % 4 == 0 {
                primary_color
            } else {
                secondary_colors[i % secondary_colors.len()]
            };

            let final_life_time = life_time + rand::random::<f32>() * 0.5;

            let particle_pos = Vector2::new(
                position.x + (rand::random::<f32>() - 0.5) * size * 0.3,
                position.y + (rand::random::<f32>() - 0.5) * size * 0.3,
            );

            let mut particle = Particle::new(particle_pos, velocity, color, final_life_time);
            particle.size = particle_size;
            self.particles.push(particle);
        }

        // Add some sparkle effects
        for _ in 0..8 {
            let sparkle_velocity = Vector2::new(
                (rand::random::<f32>() - 0.5) * 40.0,
                (rand::random::<f32>() - 0.5) * 40.0 - 30.0, // Slight upward bias
            );

            let sparkle_pos = Vector2::new(
                position.x + (rand::random::<f32>() - 0.5) * size * 0.5,
                position.y + (rand::random::<f32>() - 0.5) * size * 0.5,
            );

            let mut sparkle = Particle::new(sparkle_pos, sparkle_velocity, Color::YELLOW, 0.6);
            sparkle.size = 1.5;
            sparkle.acceleration = Vector2::new(0.0, 150.0); // Less gravity for sparkles
            self.particles.push(sparkle);
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update all particles and remove dead ones
        self.particles
            .retain_mut(|particle| particle.update(delta_time));
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for particle in &self.particles {
            particle.draw(d);
        }
    }
}
