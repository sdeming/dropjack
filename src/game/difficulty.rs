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
