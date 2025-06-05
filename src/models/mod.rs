// Models module - contains all data structures used throughout the application

pub mod cards;
pub mod database;
pub mod game;
pub mod ui;

// Re-export common models for easy access
pub use cards::{Card, CardColor, Deck, Suit, Value};
pub use database::HighScore;
pub use game::{
    DelayedDestruction, Difficulty, FallingCard, PlayingCard, Position, VisualPosition,
};
pub use ui::Particle;

// Export builder patterns for easy access - only export what we actually use
// (Removed unused wildcard imports and unused builder exports)
