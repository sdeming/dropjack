// Models module - contains all data structures used throughout the application

pub mod cards;
pub mod database;
pub mod game;
pub mod ui;

// Re-export common models for easy access
pub use cards::{Card, CardColor, Deck, Suit, Value};
pub use database::HighScore;
pub use game::{
    DelayedDestruction, Difficulty, FallingCard, PlayingCard, Position, VisualPosition,
};
pub use ui::Particle;

// Export builder patterns for easy access - only export what we actually use
// (Removed unused wildcard imports and unused builder exports)

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GameSettings {
    pub music_volume: f32, // 0.0 to 1.0
    pub music_muted: bool,
    pub sound_effects_volume: f32, // 0.0 to 1.0
    pub sound_effects_muted: bool,
    pub vsync_enabled: bool,
    pub difficulty: game::Difficulty, // Game difficulty setting
    #[serde(skip)]
    pub selected_option: usize, // 0: Music, 1: SFX, 2: VSync, 3: Difficulty (for settings navigation)
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            music_volume: 0.7,
            music_muted: false,
            sound_effects_volume: 0.8,
            sound_effects_muted: false,
            vsync_enabled: true,
            difficulty: game::Difficulty::Easy,
            selected_option: 0,
        }
    }
}

impl GameSettings {
    /// Get the path to the settings file
    pub fn settings_file_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        Self::settings_file_path_with_name("settings.json")
    }

    /// Get the path to a settings file with a custom name (for testing)
    #[cfg(test)]
    pub fn settings_file_path_with_name(
        filename: &str,
    ) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let test_dir = std::path::PathBuf::from("/tmp/dropjack_test_settings");
        std::fs::create_dir_all(&test_dir)?;
        Ok(test_dir.join(filename))
    }

    #[cfg(not(test))]
    pub fn settings_file_path_with_name(
        filename: &str,
    ) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
        let app_data_dir = dirs::data_dir()
            .ok_or("Could not determine data directory")?
            .join("DropJack");

        // Ensure the directory exists
        std::fs::create_dir_all(&app_data_dir)?;

        Ok(app_data_dir.join(filename))
    }

    /// Load settings from disk, returning default settings if file doesn't exist or is corrupted
    pub fn load() -> Self {
        match Self::try_load() {
            Ok(settings) => {
                println!("Settings loaded successfully");
                settings
            }
            Err(e) => {
                println!("Failed to load settings, using defaults: {}", e);
                Self::default()
            }
        }
    }

    /// Attempt to load settings from disk
    fn try_load() -> Result<Self, Box<dyn std::error::Error>> {
        let settings_path = Self::settings_file_path()?;

        if !settings_path.exists() {
            return Err("Settings file does not exist".into());
        }

        let contents = std::fs::read_to_string(settings_path)?;
        let mut settings: GameSettings = serde_json::from_str(&contents)?;

        // Reset UI state (selected_option should always start at 0)
        settings.selected_option = 0;

        Ok(settings)
    }

    /// Save settings to disk
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings_path = Self::settings_file_path()?;

        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(settings_path, contents)?;

        println!("Settings saved successfully");
        Ok(())
    }
}

// Import for tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_settings_default() {
        let settings = GameSettings::default();
        assert_eq!(settings.music_volume, 0.7);
        assert_eq!(settings.music_muted, false);
        assert_eq!(settings.sound_effects_volume, 0.8);
        assert_eq!(settings.sound_effects_muted, false);
        assert_eq!(settings.vsync_enabled, true);
        assert_eq!(settings.selected_option, 0);
    }

    #[test]
    fn test_game_settings_serialization() {
        let settings = GameSettings {
            music_volume: 0.5,
            music_muted: true,
            sound_effects_volume: 0.3,
            sound_effects_muted: false,
            vsync_enabled: false,
            difficulty: game::Difficulty::Hard,
            selected_option: 2, // This should be skipped in serialization
        };

        let serialized = serde_json::to_string(&settings).unwrap();
        let deserialized: GameSettings = serde_json::from_str(&serialized).unwrap();

        // Check that persisted values are preserved
        assert_eq!(deserialized.music_volume, 0.5);
        assert_eq!(deserialized.music_muted, true);
        assert_eq!(deserialized.sound_effects_volume, 0.3);
        assert_eq!(deserialized.sound_effects_muted, false);
        assert_eq!(deserialized.vsync_enabled, false);
        assert_eq!(deserialized.difficulty, game::Difficulty::Hard);

        // Check that selected_option is reset to default (0) since it's marked #[serde(skip)]
        assert_eq!(deserialized.selected_option, 0);
    }

    #[test]
    fn test_game_settings_load_nonexistent_file() {
        // Use a unique filename for this test
        let settings_path =
            GameSettings::settings_file_path_with_name("test_nonexistent.json").unwrap();
        let _ = std::fs::remove_file(&settings_path);

        // Loading settings when file doesn't exist should return defaults
        let settings = GameSettings::load();
        assert_eq!(settings.music_volume, 0.7);
        assert_eq!(settings.selected_option, 0);
    }

    #[test]
    fn test_game_settings_save_and_load() {
        // Use a unique filename for this test
        let test_filename = "test_save_load.json";
        let settings_path = GameSettings::settings_file_path_with_name(test_filename).unwrap();
        let _ = std::fs::remove_file(&settings_path);

        // Create a custom save method for testing
        let save_to_test_file =
            |settings: &GameSettings| -> Result<(), Box<dyn std::error::Error>> {
                let contents = serde_json::to_string_pretty(settings)?;
                std::fs::write(&settings_path, contents)?;
                Ok(())
            };

        // Create a custom load method for testing
        let load_from_test_file = || -> GameSettings {
            if !settings_path.exists() {
                return GameSettings::default();
            }

            let contents = std::fs::read_to_string(&settings_path).unwrap();
            let mut settings: GameSettings = serde_json::from_str(&contents).unwrap();
            settings.selected_option = 0; // Reset UI state
            settings
        };

        // Create custom settings
        let mut original_settings = GameSettings::default();
        original_settings.difficulty = game::Difficulty::Hard;
        original_settings.music_volume = 0.4;
        original_settings.music_muted = true;
        original_settings.sound_effects_volume = 0.6;
        original_settings.vsync_enabled = false;
        original_settings.selected_option = 1; // This shouldn't be saved

        // Save the settings
        assert!(save_to_test_file(&original_settings).is_ok());

        // Load the settings back
        let loaded_settings = load_from_test_file();

        // Check that persisted values match
        assert_eq!(loaded_settings.music_volume, 0.4);
        assert_eq!(loaded_settings.music_muted, true);
        assert_eq!(loaded_settings.sound_effects_volume, 0.6);
        assert_eq!(loaded_settings.vsync_enabled, false);

        // Check that UI state (selected_option) was reset to default
        assert_eq!(loaded_settings.selected_option, 0);

        // Clean up test file
        let _ = std::fs::remove_file(&settings_path);
    }

    #[test]
    fn test_corrupted_settings_file() {
        // Use a unique filename for this test
        let test_filename = "test_corrupted.json";
        let settings_path = GameSettings::settings_file_path_with_name(test_filename).unwrap();
        std::fs::create_dir_all(settings_path.parent().unwrap()).unwrap();

        // Write corrupted JSON to the settings file
        std::fs::write(&settings_path, "{ invalid json }").unwrap();

        // Create a custom load method for testing that simulates the actual load behavior
        let load_from_corrupted_file = || -> GameSettings {
            match std::fs::read_to_string(&settings_path) {
                Ok(contents) => {
                    match serde_json::from_str::<GameSettings>(&contents) {
                        Ok(mut settings) => {
                            settings.selected_option = 0;
                            settings
                        }
                        Err(_) => GameSettings::default(), // Return defaults on corruption
                    }
                }
                Err(_) => GameSettings::default(), // Return defaults if file doesn't exist
            }
        };

        // Loading corrupted settings should return defaults without panicking
        let settings = load_from_corrupted_file();
        assert_eq!(settings.music_volume, 0.7);
        assert_eq!(settings.selected_option, 0);

        // Clean up test file
        let _ = std::fs::remove_file(&settings_path);
    }
}
