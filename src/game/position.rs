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
