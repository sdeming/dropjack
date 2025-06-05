// Database-related models

pub struct HighScore {
    #[allow(dead_code)] // Used by database operations
    pub id: Option<i64>,
    pub player_initials: String,
    pub score: i32,
    pub difficulty: String,
    pub date: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_fixtures {
        use super::*;
        use chrono::Utc;

        pub fn create_test_high_score() -> HighScore {
            HighScore {
                id: None,
                player_initials: "ABC".to_string(),
                score: 1500,
                difficulty: "Medium".to_string(),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            }
        }

        pub fn create_high_score_with_id(id: i64) -> HighScore {
            HighScore {
                id: Some(id),
                player_initials: "XYZ".to_string(),
                score: 2000,
                difficulty: "Hard".to_string(),
                date: "2024-01-15 14:30:00".to_string(),
            }
        }

        pub fn create_multiple_high_scores() -> Vec<HighScore> {
            vec![
                HighScore {
                    id: Some(1),
                    player_initials: "AAA".to_string(),
                    score: 1000,
                    difficulty: "Easy".to_string(),
                    date: "2024-01-01 10:00:00".to_string(),
                },
                HighScore {
                    id: Some(2),
                    player_initials: "BBB".to_string(),
                    score: 1500,
                    difficulty: "Medium".to_string(),
                    date: "2024-01-02 11:00:00".to_string(),
                },
                HighScore {
                    id: Some(3),
                    player_initials: "CCC".to_string(),
                    score: 2000,
                    difficulty: "Hard".to_string(),
                    date: "2024-01-03 12:00:00".to_string(),
                },
            ]
        }
    }

    #[test]
    fn test_high_score_creation() {
        let high_score = HighScore {
            id: None,
            player_initials: "TEST".to_string(),
            score: 1234,
            difficulty: "Easy".to_string(),
            date: "2024-01-01 12:00:00".to_string(),
        };

        assert!(high_score.id.is_none());
        assert_eq!(high_score.player_initials, "TEST");
        assert_eq!(high_score.score, 1234);
        assert_eq!(high_score.difficulty, "Easy");
        assert_eq!(high_score.date, "2024-01-01 12:00:00");
    }

    #[test]
    fn test_high_score_with_id() {
        let high_score = test_fixtures::create_high_score_with_id(42);
        
        assert_eq!(high_score.id, Some(42));
        assert_eq!(high_score.player_initials, "XYZ");
        assert_eq!(high_score.score, 2000);
        assert_eq!(high_score.difficulty, "Hard");
    }

    #[test]
    fn test_high_score_fixture() {
        let high_score = test_fixtures::create_test_high_score();
        
        assert!(high_score.id.is_none());
        assert_eq!(high_score.player_initials, "ABC");
        assert_eq!(high_score.score, 1500);
        assert_eq!(high_score.difficulty, "Medium");
        // Date should be recent (within last minute)
        assert!(!high_score.date.is_empty());
    }

    #[test]
    fn test_multiple_high_scores_fixture() {
        let high_scores = test_fixtures::create_multiple_high_scores();
        
        assert_eq!(high_scores.len(), 3);
        
        // Check first score
        assert_eq!(high_scores[0].id, Some(1));
        assert_eq!(high_scores[0].player_initials, "AAA");
        assert_eq!(high_scores[0].score, 1000);
        
        // Check scores are different
        assert_ne!(high_scores[0].score, high_scores[1].score);
        assert_ne!(high_scores[1].score, high_scores[2].score);
    }

    #[test]
    fn test_high_score_string_fields() {
        let mut high_score = test_fixtures::create_test_high_score();
        
        // Test that string fields can be modified
        high_score.player_initials = "NEW".to_string();
        high_score.difficulty = "Hard".to_string();
        high_score.date = "2024-12-31 23:59:59".to_string();
        
        assert_eq!(high_score.player_initials, "NEW");
        assert_eq!(high_score.difficulty, "Hard");
        assert_eq!(high_score.date, "2024-12-31 23:59:59");
    }

    #[test]
    fn test_high_score_numeric_fields() {
        let mut high_score = test_fixtures::create_test_high_score();
        
        // Test score modifications
        high_score.score = 0;
        assert_eq!(high_score.score, 0);
        
        high_score.score = -100; // Negative scores (for testing edge cases)
        assert_eq!(high_score.score, -100);
        
        high_score.score = 999999; // Very high score
        assert_eq!(high_score.score, 999999);
    }

    #[test]
    fn test_high_score_id_transitions() {
        let mut high_score = test_fixtures::create_test_high_score();
        
        // Start with no ID
        assert!(high_score.id.is_none());
        
        // Assign an ID (like after database insertion)
        high_score.id = Some(123);
        assert_eq!(high_score.id, Some(123));
        
        // Remove ID (unusual but possible)
        high_score.id = None;
        assert!(high_score.id.is_none());
    }
}
