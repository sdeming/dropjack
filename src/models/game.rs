use super::cards::Card;
use std::fmt::Display;
use std::time::Instant;

// Position of a card on the board
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

// Visual position for smooth animations (in pixels)
#[derive(Debug, Clone, Copy)]
pub struct VisualPosition {
    pub x: f32,
    pub y: f32,
}

// A card in play with its position and animation state
#[derive(Debug, Clone)]
pub struct PlayingCard {
    pub card: Card,
    pub position: Position,              // Logical grid position
    pub visual_position: VisualPosition, // Visual position for smooth movement
    pub target: Position,                // Target position for animation
    pub is_falling: bool,                // Whether the card is currently falling
    pub is_hard_dropping: bool,          // Whether the card is hard dropping (faster fall)
}

pub struct PlayingCardBuilder {
    card: Card,
    position: Position,
    visual_position: Option<VisualPosition>,
    target: Option<Position>,
    is_falling: bool,
    is_hard_dropping: bool,
    cell_size: i32,
}

impl PlayingCardBuilder {
    pub fn new(card: Card, position: Position) -> Self {
        Self {
            card,
            position,
            visual_position: None,
            target: None,
            is_falling: false,
            is_hard_dropping: false,
            cell_size: 48, // Default cell size
        }
    }

    pub fn cell_size(mut self, cell_size: i32) -> Self {
        self.cell_size = cell_size;
        self
    }

    pub fn visual_position(mut self, visual_position: VisualPosition) -> Self {
        self.visual_position = Some(visual_position);
        self
    }

    pub fn target(mut self, target: Position) -> Self {
        self.target = Some(target);
        self
    }

    pub fn falling(mut self, is_falling: bool) -> Self {
        self.is_falling = is_falling;
        self
    }

    pub fn hard_dropping(mut self, is_hard_dropping: bool) -> Self {
        self.is_hard_dropping = is_hard_dropping;
        self
    }

    pub fn build(self) -> PlayingCard {
        let visual_position = self.visual_position.unwrap_or_else(|| VisualPosition {
            x: (self.position.x * self.cell_size) as f32,
            y: (self.position.y * self.cell_size) as f32,
        });

        let target = self.target.unwrap_or(self.position);

        PlayingCard {
            card: self.card,
            position: self.position,
            visual_position,
            target,
            is_falling: self.is_falling,
            is_hard_dropping: self.is_hard_dropping,
        }
    }
}

impl PlayingCard {
    pub fn builder(card: Card, position: Position) -> PlayingCardBuilder {
        PlayingCardBuilder::new(card, position)
    }
}

// Game difficulty modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Difficulty {
    Easy,
    Hard,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Difficulty::Easy => "Easy",
            Difficulty::Hard => "Hard",
        };
        write!(f, "{}", s)
    }
}

// Delayed destruction entry for cascading effects
#[derive(Debug, Clone)]
pub struct DelayedDestruction {
    pub destruction_time: Instant,
    pub chain_multiplier: i32,
    pub combination_index: usize,
}

// A card falling due to gravity
#[derive(Debug, Clone)]
pub struct FallingCard {
    pub card: Card,
    pub to_y: i32,
    pub x: i32,
    pub visual_y: f32,
    pub is_animating: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Card, Suit, Value};
    use std::time::{Duration, Instant};

    // Test fixtures for creating test data
    mod test_fixtures {
        use super::*;

        pub fn create_test_card() -> Card {
            Card::new(Suit::Hearts, Value::Ace)
        }

        pub fn create_test_position() -> Position {
            Position { x: 2, y: 3 }
        }

        pub fn create_test_visual_position() -> VisualPosition {
            VisualPosition { x: 100.0, y: 150.0 }
        }

        #[allow(dead_code)]
        pub fn create_test_playing_card() -> PlayingCard {
            PlayingCard::builder(create_test_card(), create_test_position()).build()
        }

        pub fn create_falling_card() -> FallingCard {
            FallingCard {
                card: create_test_card(),
                to_y: 7,
                x: 2,
                visual_y: 100.0,
                is_animating: true,
            }
        }
    }

    #[test]
    fn test_position() {
        let pos = Position { x: 5, y: 10 };
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 10);
    }

    #[test]
    fn test_visual_position() {
        let vis_pos = VisualPosition {
            x: 100.5,
            y: 200.75,
        };
        assert_eq!(vis_pos.x, 100.5);
        assert_eq!(vis_pos.y, 200.75);
    }

    #[test]
    fn test_difficulty_display() {
        assert_eq!(format!("{}", Difficulty::Easy), "Easy");
        assert_eq!(format!("{}", Difficulty::Hard), "Hard");
    }

    #[test]
    fn test_difficulty_equality() {
        assert_eq!(Difficulty::Easy, Difficulty::Easy);
        assert_eq!(Difficulty::Hard, Difficulty::Hard);
        assert_ne!(Difficulty::Easy, Difficulty::Hard);
    }

    #[test]
    fn test_playing_card_builder_basic() {
        let card = test_fixtures::create_test_card();
        let position = test_fixtures::create_test_position();

        let playing_card = PlayingCard::builder(card, position).build();

        assert_eq!(playing_card.card, card);
        assert_eq!(playing_card.position.x, position.x);
        assert_eq!(playing_card.position.y, position.y);
        assert_eq!(playing_card.target.x, position.x);
        assert_eq!(playing_card.target.y, position.y);
        assert!(!playing_card.is_falling);
        assert!(!playing_card.is_hard_dropping);
    }

    #[test]
    fn test_playing_card_builder_with_cell_size() {
        let card = test_fixtures::create_test_card();
        let position = Position { x: 2, y: 3 };
        let cell_size = 60;

        let playing_card = PlayingCard::builder(card, position)
            .cell_size(cell_size)
            .build();

        // Visual position should be calculated based on cell size
        assert_eq!(playing_card.visual_position.x, (2 * cell_size) as f32);
        assert_eq!(playing_card.visual_position.y, (3 * cell_size) as f32);
    }

    #[test]
    fn test_playing_card_builder_with_custom_visual_position() {
        let card = test_fixtures::create_test_card();
        let position = test_fixtures::create_test_position();
        let visual_pos = test_fixtures::create_test_visual_position();

        let playing_card = PlayingCard::builder(card, position)
            .visual_position(visual_pos)
            .build();

        assert_eq!(playing_card.visual_position.x, visual_pos.x);
        assert_eq!(playing_card.visual_position.y, visual_pos.y);
    }

    #[test]
    fn test_playing_card_builder_with_target() {
        let card = test_fixtures::create_test_card();
        let position = Position { x: 1, y: 2 };
        let target = Position { x: 3, y: 4 };

        let playing_card = PlayingCard::builder(card, position).target(target).build();

        assert_eq!(playing_card.position.x, 1);
        assert_eq!(playing_card.position.y, 2);
        assert_eq!(playing_card.target.x, 3);
        assert_eq!(playing_card.target.y, 4);
    }

    #[test]
    fn test_playing_card_builder_with_falling_state() {
        let card = test_fixtures::create_test_card();
        let position = test_fixtures::create_test_position();

        let playing_card = PlayingCard::builder(card, position)
            .falling(true)
            .hard_dropping(true)
            .build();

        assert!(playing_card.is_falling);
        assert!(playing_card.is_hard_dropping);
    }

    #[test]
    fn test_playing_card_builder_chain_methods() {
        let card = test_fixtures::create_test_card();
        let position = Position { x: 0, y: 1 };
        let target = Position { x: 2, y: 3 };
        let visual_pos = VisualPosition { x: 50.0, y: 75.0 };

        let playing_card = PlayingCard::builder(card, position)
            .cell_size(25)
            .visual_position(visual_pos)
            .target(target)
            .falling(true)
            .hard_dropping(false)
            .build();

        assert_eq!(playing_card.card, card);
        assert_eq!(playing_card.position.x, 0);
        assert_eq!(playing_card.position.y, 1);
        assert_eq!(playing_card.target.x, 2);
        assert_eq!(playing_card.target.y, 3);
        assert_eq!(playing_card.visual_position.x, 50.0);
        assert_eq!(playing_card.visual_position.y, 75.0);
        assert!(playing_card.is_falling);
        assert!(!playing_card.is_hard_dropping);
    }

    #[test]
    fn test_delayed_destruction() {
        let now = Instant::now();
        let destruction = DelayedDestruction {
            destruction_time: now + Duration::from_millis(500),
            chain_multiplier: 2,
            combination_index: 1,
        };

        assert!(destruction.destruction_time > now);
        assert_eq!(destruction.chain_multiplier, 2);
        assert_eq!(destruction.combination_index, 1);
    }

    #[test]
    fn test_falling_card() {
        let card = test_fixtures::create_test_card();
        let falling_card = FallingCard {
            card,
            to_y: 5,
            x: 3,
            visual_y: 120.0,
            is_animating: true,
        };

        assert_eq!(falling_card.card, card);
        assert_eq!(falling_card.to_y, 5);
        assert_eq!(falling_card.x, 3);
        assert_eq!(falling_card.visual_y, 120.0);
        assert!(falling_card.is_animating);
    }

    #[test]
    fn test_falling_card_creation_from_fixture() {
        let falling_card = test_fixtures::create_falling_card();

        assert_eq!(falling_card.to_y, 7);
        assert_eq!(falling_card.x, 2);
        assert_eq!(falling_card.visual_y, 100.0);
        assert!(falling_card.is_animating);
    }

    mod builder_edge_cases {
        use super::*;

        #[test]
        fn test_builder_with_zero_cell_size() {
            let card = test_fixtures::create_test_card();
            let position = Position { x: 5, y: 3 };

            let playing_card = PlayingCard::builder(card, position).cell_size(0).build();

            assert_eq!(playing_card.visual_position.x, 0.0);
            assert_eq!(playing_card.visual_position.y, 0.0);
        }

        #[test]
        fn test_builder_with_negative_position() {
            let card = test_fixtures::create_test_card();
            let position = Position { x: -1, y: -2 };

            let playing_card = PlayingCard::builder(card, position).cell_size(50).build();

            assert_eq!(playing_card.position.x, -1);
            assert_eq!(playing_card.position.y, -2);
            assert_eq!(playing_card.visual_position.x, -50.0);
            assert_eq!(playing_card.visual_position.y, -100.0);
        }

        #[test]
        fn test_builder_overriding_defaults() {
            let card = test_fixtures::create_test_card();
            let position = Position { x: 1, y: 1 };

            // First set falling to true, then override to false
            let playing_card = PlayingCard::builder(card, position)
                .falling(true)
                .falling(false)
                .build();

            assert!(!playing_card.is_falling);
        }
    }

    mod integration_tests {
        use super::*;

        #[test]
        fn test_complete_card_lifecycle() {
            let card = Card::new(Suit::Spades, Value::King);
            let initial_pos = Position { x: 2, y: 1 };
            let target_pos = Position { x: 2, y: 7 };

            // Create a playing card that starts falling
            let mut playing_card = PlayingCard::builder(card, initial_pos)
                .target(target_pos)
                .falling(true)
                .cell_size(48)
                .build();

            // Verify initial state
            assert_eq!(playing_card.card.suit, Suit::Spades);
            assert_eq!(playing_card.card.value, Value::King);
            assert_eq!(playing_card.position.x, 2);
            assert_eq!(playing_card.position.y, 1);
            assert_eq!(playing_card.target.x, 2);
            assert_eq!(playing_card.target.y, 7);
            assert!(playing_card.is_falling);

            // Simulate animation progress
            playing_card.visual_position.y += 10.0; // Move down
            assert!(playing_card.visual_position.y > (initial_pos.y * 48) as f32);

            // Simulate reaching target
            playing_card.position = playing_card.target;
            playing_card.is_falling = false;
            playing_card.visual_position.y = (target_pos.y * 48) as f32;

            assert_eq!(playing_card.position.y, 7);
            assert!(!playing_card.is_falling);
        }
    }
}
