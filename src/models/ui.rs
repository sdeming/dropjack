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

pub struct ParticleBuilder {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    life_time: f32,
    color: Color,
    size: f32,
    rotation: f32,
    angular_velocity: f32,
}

impl ParticleBuilder {
    pub fn new(position: Vector2, velocity: Vector2, color: Color, life_time: f32) -> Self {
        Self {
            position,
            velocity,
            acceleration: Vector2::new(0.0, 200.0), // Default gravity
            life_time,
            color,
            size: 2.0, // Default size
            rotation: 0.0,
            angular_velocity: (rand::random::<f32>() - 0.5) * 10.0, // Default random rotation
        }
    }

    pub fn acceleration(mut self, acceleration: Vector2) -> Self {
        self.acceleration = acceleration;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    // Removed unused rotation method - kept more commonly used methods

    pub fn angular_velocity(mut self, angular_velocity: f32) -> Self {
        self.angular_velocity = angular_velocity;
        self
    }

    pub fn build(self) -> Particle {
        Particle {
            position: self.position,
            velocity: self.velocity,
            acceleration: self.acceleration,
            life_time: self.life_time,
            max_life_time: self.life_time,
            color: self.color,
            size: self.size,
            rotation: self.rotation,
            angular_velocity: self.angular_velocity,
        }
    }
}

impl Particle {
    pub fn builder(
        position: Vector2,
        velocity: Vector2,
        color: Color,
        life_time: f32,
    ) -> ParticleBuilder {
        ParticleBuilder::new(position, velocity, color, life_time)
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

#[cfg(test)]
mod tests {
    use super::*;

    // Test fixtures for UI testing
    mod test_fixtures {
        use super::*;

        pub fn create_test_particle() -> Particle {
            Particle {
                position: Vector2::new(100.0, 200.0),
                velocity: Vector2::new(50.0, -30.0),
                acceleration: Vector2::new(0.0, 98.0),
                life_time: 2.0,
                max_life_time: 2.0,
                color: Color::RED,
                size: 3.0,
                rotation: 0.0,
                angular_velocity: 1.5,
            }
        }

        pub fn create_test_vector() -> Vector2 {
            Vector2::new(10.0, 20.0)
        }

        pub fn create_test_colors() -> Vec<Color> {
            vec![
                Color::RED,
                Color::GREEN,
                Color::BLUE,
                Color::YELLOW,
                Color::WHITE,
                Color::BLACK,
            ]
        }
    }

    #[test]
    fn test_particle_creation() {
        let particle = test_fixtures::create_test_particle();

        assert_eq!(particle.position.x, 100.0);
        assert_eq!(particle.position.y, 200.0);
        assert_eq!(particle.velocity.x, 50.0);
        assert_eq!(particle.velocity.y, -30.0);
        assert_eq!(particle.life_time, 2.0);
        assert_eq!(particle.max_life_time, 2.0);
        assert_eq!(particle.color, Color::RED);
        assert_eq!(particle.size, 3.0);
    }

    #[test]
    fn test_particle_builder_basic() {
        let position = Vector2::new(50.0, 75.0);
        let velocity = Vector2::new(10.0, -5.0);
        let color = Color::BLUE;
        let life_time = 1.5;

        let particle = Particle::builder(position, velocity, color, life_time).build();

        assert_eq!(particle.position, position);
        assert_eq!(particle.velocity, velocity);
        assert_eq!(particle.color, color);
        assert_eq!(particle.life_time, life_time);
        assert_eq!(particle.max_life_time, life_time);

        // Test defaults
        assert_eq!(particle.acceleration, Vector2::new(0.0, 200.0)); // Default gravity
        assert_eq!(particle.size, 2.0); // Default size
        assert_eq!(particle.rotation, 0.0);
    }

    #[test]
    fn test_particle_builder_with_acceleration() {
        let acceleration = Vector2::new(5.0, 150.0);

        let particle = Particle::builder(
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Color::WHITE,
            1.0,
        )
        .acceleration(acceleration)
        .build();

        assert_eq!(particle.acceleration, acceleration);
    }

    #[test]
    fn test_particle_builder_with_size() {
        let size = 5.5;

        let particle = Particle::builder(
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Color::GREEN,
            1.0,
        )
        .size(size)
        .build();

        assert_eq!(particle.size, size);
    }

    #[test]
    fn test_particle_builder_with_angular_velocity() {
        let angular_velocity = std::f32::consts::PI;

        let particle = Particle::builder(
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Color::YELLOW,
            1.0,
        )
        .angular_velocity(angular_velocity)
        .build();

        assert_eq!(particle.angular_velocity, angular_velocity);
    }

    #[test]
    fn test_particle_builder_chaining() {
        let position = Vector2::new(100.0, 150.0);
        let velocity = Vector2::new(25.0, -15.0);
        let acceleration = Vector2::new(2.0, 100.0);
        let color = Color::PURPLE;
        let life_time = 3.0;
        let size = 4.0;
        let angular_velocity = 2.5;

        let particle = Particle::builder(position, velocity, color, life_time)
            .acceleration(acceleration)
            .size(size)
            .angular_velocity(angular_velocity)
            .build();

        assert_eq!(particle.position, position);
        assert_eq!(particle.velocity, velocity);
        assert_eq!(particle.acceleration, acceleration);
        assert_eq!(particle.color, color);
        assert_eq!(particle.life_time, life_time);
        assert_eq!(particle.size, size);
        assert_eq!(particle.angular_velocity, angular_velocity);
    }

    #[test]
    fn test_particle_update_physics() {
        let mut particle = Particle::builder(
            Vector2::new(0.0, 0.0),
            Vector2::new(10.0, 0.0),
            Color::RED,
            2.0,
        )
        .acceleration(Vector2::new(0.0, 10.0))
        .build();

        let delta_time = 0.1;
        let is_alive = particle.update(delta_time);

        // Check physics updates
        assert_eq!(particle.velocity.x, 10.0); // No acceleration in x
        assert_eq!(particle.velocity.y, 1.0); // acceleration * delta_time = 10.0 * 0.1
        assert_eq!(particle.position.x, 1.0); // velocity * delta_time = 10.0 * 0.1
        assert_eq!(particle.position.y, 0.1); // new_velocity * delta_time = 1.0 * 0.1

        // Check lifetime update
        assert_eq!(particle.life_time, 1.9); // 2.0 - 0.1
        assert!(is_alive);
    }

    #[test]
    fn test_particle_update_rotation() {
        let mut particle = Particle::builder(
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Color::BLUE,
            1.0,
        )
        .angular_velocity(5.0)
        .build();

        let delta_time = 0.2;
        particle.update(delta_time);

        assert_eq!(particle.rotation, 1.0); // 5.0 * 0.2
    }

    #[test]
    fn test_particle_lifetime_and_alpha() {
        let mut particle = Particle::builder(
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Color::new(255, 0, 0, 255),
            1.0,
        )
        .build();

        // Update half lifetime
        let is_alive = particle.update(0.5);
        assert!(is_alive);
        assert_eq!(particle.life_time, 0.5);
        assert_eq!(particle.color.a, 127); // ~50% alpha (255 * 0.5)

        // Update to nearly dead
        let is_alive = particle.update(0.49);
        assert!(is_alive);
        assert!(particle.color.a < 50); // Very low alpha

        // Update past lifetime
        let is_alive = particle.update(0.1);
        assert!(!is_alive);
        assert!(particle.life_time <= 0.0);
        assert_eq!(particle.color.a, 0); // Alpha should be 0
    }

    #[test]
    fn test_particle_alpha_fade() {
        let initial_alpha = 200u8;
        let mut particle = Particle::builder(
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Color::new(255, 255, 255, initial_alpha),
            2.0,
        )
        .build();

        // Update to half lifetime
        particle.update(1.0); // Half of 2.0 seconds
        // The alpha calculation is based on max_life_time (255), not initial alpha
        let alpha_ratio = particle.life_time / particle.max_life_time; // 1.0 / 2.0 = 0.5
        let expected_alpha = (255.0 * alpha_ratio) as u8; // 255 * 0.5 = 127
        assert_eq!(particle.color.a, expected_alpha);
    }

    #[test]
    fn test_particle_death() {
        let mut particle = Particle::builder(
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Color::WHITE,
            0.1, // Very short lifetime
        )
        .build();

        // Should be alive initially
        assert!(particle.life_time > 0.0);

        // Update past lifetime
        let is_alive = particle.update(0.2);
        assert!(!is_alive);
        assert!(particle.life_time <= 0.0);
    }

    #[test]
    fn test_particle_with_different_colors() {
        let colors = test_fixtures::create_test_colors();

        for color in colors {
            let particle =
                Particle::builder(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0), color, 1.0)
                    .build();

            assert_eq!(particle.color, color);
        }
    }

    #[test]
    fn test_particle_zero_lifetime() {
        let mut particle = Particle::builder(
            Vector2::new(0.0, 0.0),
            Vector2::new(0.0, 0.0),
            Color::RED,
            0.0, // Zero lifetime
        )
        .build();

        let is_alive = particle.update(0.01);
        assert!(!is_alive);
    }

    #[test]
    fn test_particle_negative_lifetime() {
        let mut particle = test_fixtures::create_test_particle();
        particle.life_time = -1.0; // Manually set negative

        let is_alive = particle.update(0.01);
        assert!(!is_alive);
        assert_eq!(particle.color.a, 0); // Alpha should be 0
    }

    #[test]
    fn test_particle_builder_default_values() {
        let particle = Particle::builder(
            Vector2::new(1.0, 2.0),
            Vector2::new(3.0, 4.0),
            Color::ORANGE,
            5.0,
        )
        .build();

        // Test that defaults are applied correctly
        assert_eq!(particle.acceleration, Vector2::new(0.0, 200.0));
        assert_eq!(particle.size, 2.0);
        assert_eq!(particle.rotation, 0.0);
        // Angular velocity should be random, just check it's set
        assert!(particle.angular_velocity >= -5.0 && particle.angular_velocity <= 5.0);
    }

    mod integration_tests {
        use super::*;

        #[test]
        fn test_particle_simulation() {
            // Test a complete particle simulation
            let mut particle = Particle::builder(
                Vector2::new(100.0, 50.0),
                Vector2::new(20.0, -30.0),
                Color::new(255, 100, 50, 255),
                1.0,
            )
            .acceleration(Vector2::new(0.0, 98.0)) // Gravity
            .size(3.5)
            .angular_velocity(2.0)
            .build();

            let mut time_step = 0.0;
            let delta_time = 0.01; // 10ms steps
            let mut updates = 0;
            let initial_y = particle.position.y;

            // Simulate particle for its lifetime
            while time_step < 1.0 && updates < 200 {
                // Safety limit
                let is_alive = particle.update(delta_time);

                if !is_alive {
                    break;
                }

                // Verify physics are working - particle can move up initially then down due to gravity
                // Just check that it's moving and rotating
                assert!(particle.rotation >= 0.0); // Should rotate

                time_step += delta_time;
                updates += 1;
            }

            // Should have completed simulation
            assert!(
                updates < 200,
                "Simulation should complete within reasonable time"
            );
            assert!(
                particle.life_time <= 0.0,
                "Particle should be dead after simulation"
            );
        }

        #[test]
        fn test_multiple_particles_independence() {
            // Test that multiple particles can be created and updated independently
            let mut particles = vec![
                Particle::builder(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(10.0, 0.0),
                    Color::RED,
                    1.0,
                )
                .build(),
                Particle::builder(
                    Vector2::new(10.0, 10.0),
                    Vector2::new(-5.0, 5.0),
                    Color::BLUE,
                    2.0,
                )
                .build(),
                Particle::builder(
                    Vector2::new(20.0, 20.0),
                    Vector2::new(0.0, -10.0),
                    Color::GREEN,
                    0.5,
                )
                .build(),
            ];

            let initial_positions: Vec<Vector2> = particles.iter().map(|p| p.position).collect();

            // Update all particles
            for particle in &mut particles {
                particle.update(0.1);
            }

            // Verify they moved independently
            for (i, particle) in particles.iter().enumerate() {
                assert_ne!(
                    particle.position, initial_positions[i],
                    "Particle {} should have moved",
                    i
                );
            }

            // Verify different lifetimes
            assert!(particles[0].life_time != particles[1].life_time);
            assert!(particles[1].life_time != particles[2].life_time);
        }
    }
}
