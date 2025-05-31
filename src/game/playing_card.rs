use super::position::{Position, VisualPosition};
use crate::cards::Card;

// A card in play with its position and animation state
#[derive(Debug, Clone)]
pub struct PlayingCard {
    pub card: Card,
    pub position: Position,              // Logical grid position
    pub visual_position: VisualPosition, // Visual position for smooth movement
    pub target: Position,                // Target position for animation
    pub is_falling: bool,                // Whether the card is currently falling
}
