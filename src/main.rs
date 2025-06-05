mod audio;
mod database;
mod game;
mod models;
mod ui;

use std::fs;
use std::path::PathBuf;
// Removed unused Duration import

fn get_app_data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let app_name = "DropJack";

    // Get the appropriate data directory for the current platform
    let data_dir = dirs::data_dir()
        .ok_or("Could not determine data directory")?
        .join(app_name);

    // Create the directory if it doesn't exist
    fs::create_dir_all(&data_dir)?;

    Ok(data_dir)
}

// Removed unused create_game_with_difficulty function
// This function demonstrated builder usage but wasn't called in the current codebase

fn main() {
    // Get the proper application data directory
    let app_data_dir = get_app_data_dir().expect("Failed to create application data directory");

    // Set the database path within the app data directory
    let db_path = app_data_dir.join("highscores.db");

    // Initialize the game with default configuration using builder pattern
    // This demonstrates how the builder makes it easy to create different game configurations
    let mut game = game::Game::builder()
        .database_path(&db_path)
        .build()
        .expect("Failed to initialize game");

    // The builder pattern makes it easy to create custom configurations if needed:
    // let mut game = game::Game::builder()
    //     .board_size(12, 18)
    //     .difficulty(models::Difficulty::Hard)
    //     .fall_speed(Duration::from_millis(800))
    //     .database_path(&db_path)
    //     .build()
    //     .expect("Failed to initialize custom game");

    // Create and run the UI
    let mut game_ui = ui::GameUI::new();
    game_ui.run(&mut game);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_app_data_dir() {
        let result = get_app_data_dir();
        assert!(result.is_ok());
        
        let app_data_dir = result.unwrap();
        assert!(app_data_dir.ends_with("DropJack"));
        assert!(app_data_dir.exists()); // Should be created by the function
    }

    #[test]
    fn test_get_app_data_dir_creates_directory() {
        // This test verifies that the function creates the directory
        // Even if it already exists, it should still work
        let result = get_app_data_dir();
        assert!(result.is_ok());
        
        let app_data_dir = result.unwrap();
        assert!(app_data_dir.is_dir());
    }

    #[test]
    fn test_get_app_data_dir_consistency() {
        // Multiple calls should return the same path
        let dir1 = get_app_data_dir().unwrap();
        let dir2 = get_app_data_dir().unwrap();
        assert_eq!(dir1, dir2);
    }

    // Integration test for the main application setup
    #[test]
    fn test_application_initialization() {
        // Test that we can initialize the core components without panicking
        let app_data_dir = get_app_data_dir().expect("Failed to get app data directory");
        let db_path = app_data_dir.join("test_highscores.db");

        // Test that we can create a game instance
        let game_result = game::Game::builder()
            .database_path(&db_path)
            .build();

        assert!(game_result.is_ok());

        // Clean up test database
        if db_path.exists() {
            let _ = std::fs::remove_file(&db_path);
        }
    }
}
