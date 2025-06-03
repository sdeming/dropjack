use rusqlite::{params, Connection, Result};
use std::path::Path;

pub struct HighScore {
    pub id: Option<i64>,
    pub player_initials: String,
    pub score: i32,
    pub difficulty: String,
    pub date: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Create a high scores table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS high_scores (
                id INTEGER PRIMARY KEY,
                player_initials TEXT NOT NULL,
                score INTEGER NOT NULL,
                difficulty TEXT NOT NULL,
                date TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Database { conn })
    }

    pub fn add_high_score(&self, high_score: &HighScore) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO high_scores (player_initials, score, difficulty, date) VALUES (?1, ?2, ?3, ?4)",
            params![
                high_score.player_initials,
                high_score.score,
                high_score.difficulty,
                high_score.date
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_high_scores(&self, limit: usize) -> Result<Vec<HighScore>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, player_initials, score, difficulty, date FROM high_scores ORDER BY score DESC LIMIT ?1"
        )?;

        let high_scores = stmt.query_map(params![limit as i64], |row| {
            Ok(HighScore {
                id: Some(row.get(0)?),
                player_initials: row.get(1)?,
                score: row.get(2)?,
                difficulty: row.get(3)?,
                date: row.get(4)?,
            })
        })?;

        high_scores.collect()
    }
}
