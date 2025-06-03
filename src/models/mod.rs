// Models module - contains all data structures used throughout the application

pub mod cards;
pub mod game;
pub mod database;
pub mod ui;

// Re-export common models for easy access
#[allow(unused_imports)]
pub use cards::{Card, Deck, Suit, Value, CardColor};
#[allow(unused_imports)]
pub use game::{Position, VisualPosition, PlayingCard, Difficulty, DelayedDestruction, FallingCard};
pub use database::HighScore;
#[allow(unused_imports)]
pub use ui::Particle; 