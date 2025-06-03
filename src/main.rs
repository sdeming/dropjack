mod database;
mod game;
mod ui;
mod models;

use std::path::PathBuf;
use std::fs;

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

fn main() {
    // Get the proper application data directory
    let app_data_dir = get_app_data_dir()
        .expect("Failed to create application data directory");
    
    // Set the database path within the app data directory
    let db_path = app_data_dir.join("highscores.db");
    
    // Initialize the game with the proper database path
    let mut game = game::Game::new(&db_path);

    // Create and run the UI
    let mut game_ui = ui::GameUI::new();
    game_ui.run(&mut game);
}
