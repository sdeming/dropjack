use std::time::Instant;
use super::cards::Card;

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
}

// Game difficulty modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Hard,
}

impl Difficulty {
    pub fn to_string(&self) -> String {
        match self {
            Difficulty::Easy => "Easy".to_string(),
            Difficulty::Hard => "Hard".to_string(),
        }
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