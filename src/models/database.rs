// Database-related models

pub struct HighScore {
    pub id: Option<i64>,
    pub player_initials: String,
    pub score: i32,
    pub difficulty: String,
    pub date: String,
}
