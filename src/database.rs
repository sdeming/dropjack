use crate::models::HighScore;
use rusqlite::{Connection, Result, params};
use std::path::Path;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::TempDir;

    // Test fixtures for creating temporary databases and test data
    mod test_fixtures {
        use super::*;
        use chrono::Utc;

        pub fn create_temp_database() -> (Database, TempDir) {
            let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
            let db_path = temp_dir.path().join("test.db");
            let db = Database::new(&db_path).expect("Failed to create test database");
            (db, temp_dir)
        }

        pub fn create_sample_high_score(initials: &str, score: i32, difficulty: &str) -> HighScore {
            HighScore {
                id: None,
                player_initials: initials.to_string(),
                score,
                difficulty: difficulty.to_string(),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            }
        }

        pub fn create_multiple_high_scores() -> Vec<HighScore> {
            vec![
                create_sample_high_score("AAA", 1000, "Easy"),
                create_sample_high_score("BBB", 1500, "Medium"),
                create_sample_high_score("CCC", 2000, "Hard"),
                create_sample_high_score("DDD", 500, "Easy"),
                create_sample_high_score("EEE", 1200, "Medium"),
            ]
        }
    }

    #[test]
    fn test_database_creation() {
        let (db, _temp_dir) = test_fixtures::create_temp_database();

        // Test that we can query the database and it has the expected table
        let mut stmt = db
            .conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='high_scores'")
            .expect("Failed to prepare statement");

        let result: rusqlite::Result<Vec<String>> = stmt
            .query_map([], |row| Ok(row.get::<_, String>(0)?))
            .and_then(|mapped_rows| mapped_rows.collect());

        assert!(result.is_ok());
        let table_names = result.unwrap();
        assert!(!table_names.is_empty());
        assert_eq!(table_names[0], "high_scores");
    }

    #[test]
    fn test_add_high_score() {
        let (db, _temp_dir) = test_fixtures::create_temp_database();
        let high_score = test_fixtures::create_sample_high_score("TST", 1000, "Easy");

        let result = db.add_high_score(&high_score);
        assert!(result.is_ok());
        assert!(result.unwrap() > 0); // Should return a valid row ID
    }

    #[test]
    fn test_add_multiple_high_scores() {
        let (db, _temp_dir) = test_fixtures::create_temp_database();
        let high_scores = test_fixtures::create_multiple_high_scores();

        for high_score in &high_scores {
            let result = db.add_high_score(high_score);
            assert!(result.is_ok());
        }

        // Verify we can retrieve the scores
        let retrieved_scores = db.get_high_scores(10).expect("Failed to retrieve scores");
        assert_eq!(retrieved_scores.len(), high_scores.len());
    }

    #[test]
    fn test_get_high_scores_empty_database() {
        let (db, _temp_dir) = test_fixtures::create_temp_database();

        let scores = db
            .get_high_scores(10)
            .expect("Failed to get high scores from empty database");
        assert!(scores.is_empty());
    }

    #[test]
    fn test_get_high_scores_ordering() {
        let (db, _temp_dir) = test_fixtures::create_temp_database();

        // Add scores in random order
        let high_scores = vec![
            test_fixtures::create_sample_high_score("LOW", 100, "Easy"),
            test_fixtures::create_sample_high_score("HIGH", 2000, "Hard"),
            test_fixtures::create_sample_high_score("MID", 1000, "Medium"),
        ];

        for high_score in &high_scores {
            db.add_high_score(high_score)
                .expect("Failed to add high score");
        }

        // Retrieve scores - should be ordered by score DESC
        let retrieved_scores = db.get_high_scores(10).expect("Failed to retrieve scores");
        assert_eq!(retrieved_scores.len(), 3);
        assert_eq!(retrieved_scores[0].score, 2000); // Highest first
        assert_eq!(retrieved_scores[1].score, 1000); // Middle second
        assert_eq!(retrieved_scores[2].score, 100); // Lowest last
    }

    #[test]
    fn test_get_high_scores_limit() {
        let (db, _temp_dir) = test_fixtures::create_temp_database();
        let high_scores = test_fixtures::create_multiple_high_scores();

        for high_score in &high_scores {
            db.add_high_score(high_score)
                .expect("Failed to add high score");
        }

        // Test limit functionality
        let limited_scores = db
            .get_high_scores(3)
            .expect("Failed to retrieve limited scores");
        assert_eq!(limited_scores.len(), 3);

        // Should get the top 3 scores
        let all_scores = db
            .get_high_scores(10)
            .expect("Failed to retrieve all scores");
        for i in 0..3 {
            assert_eq!(limited_scores[i].score, all_scores[i].score);
        }
    }

    #[test]
    fn test_high_score_data_integrity() {
        let (db, _temp_dir) = test_fixtures::create_temp_database();
        let original_score = test_fixtures::create_sample_high_score("XYZ", 1500, "Medium");

        let row_id = db
            .add_high_score(&original_score)
            .expect("Failed to add high score");
        assert!(row_id > 0);

        let retrieved_scores = db.get_high_scores(1).expect("Failed to retrieve scores");
        assert_eq!(retrieved_scores.len(), 1);

        let retrieved_score = &retrieved_scores[0];
        assert_eq!(
            retrieved_score.player_initials,
            original_score.player_initials
        );
        assert_eq!(retrieved_score.score, original_score.score);
        assert_eq!(retrieved_score.difficulty, original_score.difficulty);
        assert_eq!(retrieved_score.date, original_score.date);
        assert!(retrieved_score.id.is_some());
        assert_eq!(retrieved_score.id.unwrap(), row_id);
    }

    #[test]
    fn test_database_persistence() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let db_path = temp_dir.path().join("persistent_test.db");

        // Create database and add a score
        {
            let db = Database::new(&db_path).expect("Failed to create database");
            let high_score = test_fixtures::create_sample_high_score("PER", 999, "Test");
            db.add_high_score(&high_score)
                .expect("Failed to add high score");
        }

        // Reopen database and verify data persists
        {
            let db = Database::new(&db_path).expect("Failed to reopen database");
            let scores = db.get_high_scores(10).expect("Failed to retrieve scores");
            assert_eq!(scores.len(), 1);
            assert_eq!(scores[0].player_initials, "PER");
            assert_eq!(scores[0].score, 999);
        }
    }

    #[test]
    fn test_database_error_handling() {
        // Test with invalid path (should fail gracefully)
        let invalid_path = Path::new("/invalid/path/that/does/not/exist/test.db");
        let result = Database::new(invalid_path);
        assert!(result.is_err());
    }
}
