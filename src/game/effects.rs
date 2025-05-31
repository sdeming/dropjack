use std::time::Instant;

// Delayed destruction entry for cascading effects
#[derive(Debug, Clone)]
pub struct DelayedDestruction {
    pub destruction_time: Instant,
    pub chain_multiplier: i32,
    pub combination_index: usize,
}
