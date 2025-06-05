use crate::models::{Card, Deck, Suit, Value};
use crate::ui::atlas_card_renderer::AtlasCardRenderer;
use crate::ui::atlas_card_renderer::CardRenderOptions;
use crate::ui::config::AnimationConfig;
use crate::ui::config::ScreenConfig;
use raylib::prelude::*;

#[derive(Clone)]
pub struct AnimatedCard {
    pub position: Vector2,
    pub velocity: Vector2,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub card: Card,
    pub size: f32,
    pub alpha: u8,
}

impl AnimatedCard {
    pub fn new(card: Card) -> Self {
        // Create random card with 10% larger size
        let size = AnimationConfig::CARD_SIZE;

        // Random position across the screen
        let x = rand::random::<f32>() * ScreenConfig::WIDTH as f32;
        let y = rand::random::<f32>() * ScreenConfig::HEIGHT as f32;

        // Random velocity - not too fast as specified
        let velocity_x = (rand::random::<f32>() - 0.5) * AnimationConfig::MAX_SPEED;
        let velocity_y = (rand::random::<f32>() - 0.5) * AnimationConfig::MAX_SPEED;

        // Random rotation and angular velocity - slight rotation
        let angular_velocity =
            (rand::random::<f32>() - 0.5) * AnimationConfig::ANGULAR_VELOCITY_RANGE;

        Self {
            position: Vector2::new(x, y),
            velocity: Vector2::new(velocity_x, velocity_y),
            rotation: rand::random::<f32>() * AnimationConfig::ROTATION_MAX,
            angular_velocity,
            card,
            size,
            alpha: AnimationConfig::ALPHA,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        // Update position
        self.position.x += self.velocity.x * delta_time;
        self.position.y += self.velocity.y * delta_time;

        // Update rotation
        self.rotation += self.angular_velocity * delta_time;
        if self.rotation > AnimationConfig::ROTATION_MAX {
            self.rotation -= AnimationConfig::ROTATION_MAX;
        } else if self.rotation < 0.0 {
            self.rotation += AnimationConfig::ROTATION_MAX;
        }

        // Bounce off walls
        let half_size = self.size / 2.0;

        // Left and right boundaries
        if self.position.x - half_size <= 0.0 {
            self.position.x = half_size;
            self.velocity.x = -self.velocity.x;
        } else if self.position.x + half_size >= ScreenConfig::WIDTH as f32 {
            self.position.x = ScreenConfig::WIDTH as f32 - half_size;
            self.velocity.x = -self.velocity.x;
        }

        // Top and bottom boundaries
        if self.position.y - half_size <= 0.0 {
            self.position.y = half_size;
            self.velocity.y = -self.velocity.y;
        } else if self.position.y + half_size >= ScreenConfig::HEIGHT as f32 {
            self.position.y = ScreenConfig::HEIGHT as f32 - half_size;
            self.velocity.y = -self.velocity.y;
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, atlas: &Texture2D) {
        let tint = Color::new(255, 255, 255, self.alpha);
        let options = CardRenderOptions::new(
            (self.position.x - self.size / 2.0) as i32,
            (self.position.y - self.size / 2.0) as i32,
            self.size as i32,
        )
        .with_rotation(self.rotation)
        .with_tint(tint);

        AtlasCardRenderer::draw_card_with_options(d, atlas, self.card, options);
    }
}

pub struct AnimatedBackground {
    cards: Vec<AnimatedCard>,
}

impl AnimatedBackground {
    pub fn new() -> Self {
        let mut deck = Deck::new();
        deck.shuffle();

        // Create evenly distributed cards across the screen
        let cols = AnimationConfig::GRID_COLS;
        let rows = AnimationConfig::GRID_ROWS;
        let total_cards = cols * rows;

        let cards = (0..total_cards)
            .map(|i| {
                let col = i % cols;
                let row = i / cols;

                // Get a card from the deck, reshuffle if needed
                let card = if let Some(card) = deck.draw() {
                    card
                } else {
                    // If deck is empty, create a new shuffled deck
                    deck = Deck::new();
                    deck.shuffle();
                    deck.draw().unwrap_or(Card::new(Suit::Spades, Value::Ace))
                };

                // Base position in grid
                let grid_x = (col as f32 + 0.5) * (ScreenConfig::WIDTH as f32 / cols as f32);
                let grid_y = (row as f32 + 0.5) * (ScreenConfig::HEIGHT as f32 / rows as f32);

                // Add some randomness to avoid perfect grid
                let x = grid_x + (rand::random::<f32>() - 0.5) * AnimationConfig::RANDOMNESS;
                let y = grid_y + (rand::random::<f32>() - 0.5) * AnimationConfig::RANDOMNESS;

                let mut animated_card = AnimatedCard::new(card);
                animated_card.position = Vector2::new(
                    x.max(animated_card.size / 2.0)
                        .min(ScreenConfig::WIDTH as f32 - animated_card.size / 2.0),
                    y.max(animated_card.size / 2.0)
                        .min(ScreenConfig::HEIGHT as f32 - animated_card.size / 2.0),
                );

                animated_card
            })
            .collect();

        Self { cards }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.cards
            .iter_mut()
            .for_each(|card| card.update(delta_time));
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, atlas: &Texture2D) {
        self.cards.iter().for_each(|card| card.draw(d, atlas));
    }
}
