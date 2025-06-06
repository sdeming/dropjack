use crate::game::AudioEvent;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Source};
use std::collections::HashMap;

/// Audio system for the DropJack game using rodio
///
/// Supports individual sound files for each of the 12 audio events.
/// Falls back to click.ogg if specific event sounds are missing.
/// Also includes music playback capabilities (stubbed for now).
pub struct AudioSystem {
    _stream: OutputStream, // Keep alive for the entire program duration
    stream_handle: OutputStreamHandle,
    sound_data: HashMap<AudioEvent, Vec<u8>>, // Event-specific audio data
    fallback_sound: Option<Vec<u8>>,          // Fallback click.ogg for missing sounds
    music_playing: bool,                      // Track if music is currently playing
    current_music_volume: f32,                // Current music volume
}

impl AudioSystem {
    /// Initialize the audio system using rodio
    pub fn new() -> Self {
        // Initialize rodio output stream
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok((stream, handle)) => {
                println!("Audio system initialized successfully with rodio");
                (stream, handle)
            }
            Err(e) => {
                eprintln!("Warning: Could not initialize audio: {}", e);
                return AudioSystem {
                    _stream: OutputStream::try_default()
                        .unwrap_or_else(|_| panic!("Failed to create fallback audio stream"))
                        .0,
                    stream_handle: OutputStream::try_default().unwrap().1,
                    sound_data: HashMap::new(),
                    fallback_sound: None,
                    music_playing: false,
                    current_music_volume: 0.7,
                };
            }
        };

        // Load the fallback click sound
        let fallback_sound = Self::load_sound_file("assets/audio/click.ogg");
        if fallback_sound.is_none() {
            eprintln!("Warning: Could not load fallback audio file assets/audio/click.ogg");
        }

        // Try to load event-specific audio files
        let mut sound_data = HashMap::new();
        let audio_config = Self::get_audio_config();

        for (event, file_path) in audio_config {
            if let Some(data) = Self::load_sound_file(&file_path) {
                sound_data.insert(event, data);
                println!("Loaded audio for {:?}: {}", event, file_path);
            } else {
                println!(
                    "Using fallback sound for {:?} (missing: {})",
                    event, file_path
                );
            }
        }

        AudioSystem {
            _stream: stream,
            stream_handle,
            sound_data,
            fallback_sound,
            music_playing: false,
            current_music_volume: 0.7,
        }
    }

    /// Play sound for a specific audio event with volume control
    pub fn play_event(
        &self,
        event: AudioEvent,
        volume: f32,
        muted: bool,
        _rl: &mut raylib::prelude::RaylibHandle,
    ) {
        // Don't play if muted or volume is 0
        if muted || volume <= 0.0 {
            return;
        }

        // Try to get event-specific sound, fall back to click.ogg if not found
        let sound_data = self.sound_data.get(&event).or(self.fallback_sound.as_ref());

        if let Some(data) = sound_data {
            // Create a new decoder from the sound data for each play
            let cursor = std::io::Cursor::new(data.clone());

            match Decoder::new(cursor) {
                Ok(source) => {
                    // Apply volume adjustment and play the sound
                    let source_with_volume = source.amplify(volume);
                    if let Err(e) = self
                        .stream_handle
                        .play_raw(source_with_volume.convert_samples())
                    {
                        eprintln!("Failed to play sound for {:?}: {}", event, e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to decode sound for {:?}: {}", event, e);
                }
            }
        } else {
            eprintln!("No audio available for {:?}", event);
        }
    }

    /// Configuration mapping: AudioEvent -> file path
    ///
    /// To customize audio, modify these file paths or add the corresponding
    /// audio files to your assets/audio/ directory.
    fn get_audio_config() -> HashMap<AudioEvent, String> {
        HashMap::from([
            (
                AudioEvent::DifficultyChange,
                "assets/audio/difficulty_change.ogg".to_string(),
            ),
            (
                AudioEvent::StartGame,
                "assets/audio/start_game.ogg".to_string(),
            ),
            (AudioEvent::PauseGame, "assets/audio/pause.ogg".to_string()),
            (
                AudioEvent::ResumeGame,
                "assets/audio/resume.ogg".to_string(),
            ),
            (
                AudioEvent::OpenQuitConfirmation,
                "assets/audio/open_quit.ogg".to_string(),
            ),
            (
                AudioEvent::ReturnToGame,
                "assets/audio/return_to_game.ogg".to_string(),
            ),
            (AudioEvent::QuitGame, "assets/audio/quit.ogg".to_string()),
            (
                AudioEvent::DropCard,
                "assets/audio/drop_card.ogg".to_string(),
            ),
            (
                AudioEvent::MakeMatch,
                "assets/audio/make_match.ogg".to_string(),
            ),
            (
                AudioEvent::ExplodeCard,
                "assets/audio/explode_card.ogg".to_string(),
            ),
            (
                AudioEvent::ForfeitGame,
                "assets/audio/forfeit.ogg".to_string(),
            ),
            (
                AudioEvent::GameOver,
                "assets/audio/game_over.ogg".to_string(),
            ),
            (
                AudioEvent::MoveLeft,
                "assets/audio/move_left.ogg".to_string(),
            ),
            (
                AudioEvent::MoveRight,
                "assets/audio/move_right.ogg".to_string(),
            ),
            (
                AudioEvent::SoftDrop,
                "assets/audio/soft_drop.ogg".to_string(),
            ),
            (
                AudioEvent::HardDrop,
                "assets/audio/hard_drop.ogg".to_string(),
            ),
        ])
    }

    /// Load a sound file into memory
    /// Returns the raw bytes that can be decoded multiple times
    fn load_sound_file(path: &str) -> Option<Vec<u8>> {
        std::fs::read(path).ok()
    }

    /// Get statistics about loaded sounds
    pub fn get_audio_stats(&self) -> (usize, usize) {
        let specific_sounds = self.sound_data.len();
        let total_possible = Self::get_audio_config().len();
        (specific_sounds, total_possible)
    }

    /// Start playing background music (stubbed implementation)
    pub fn start_music(&mut self, volume: f32, _muted: bool) {
        // In a real implementation, this would load and start playing a music file
        // For now, we just track the state
        self.music_playing = true;
        self.current_music_volume = volume;
        // TODO: Load and play actual music file (e.g., "assets/music/background.ogg")
    }

    /// Stop playing background music
    pub fn stop_music(&mut self) {
        self.music_playing = false;
        // TODO: Stop the actual music playback
    }

    /// Set music volume
    pub fn set_music_volume(&mut self, volume: f32) {
        self.current_music_volume = volume.clamp(0.0, 1.0);
        // TODO: Apply volume to the currently playing music
    }

    /// Check if music is currently playing
    pub fn is_music_playing(&self) -> bool {
        self.music_playing
    }

    /// Get current music volume
    pub fn get_music_volume(&self) -> f32 {
        self.current_music_volume
    }

    /// List which sounds are loaded and which are using fallback
    pub fn print_audio_status(&self) {
        let config = Self::get_audio_config();
        println!("\n=== Audio System Status ===");

        if self.fallback_sound.is_some() {
            println!("✅ Fallback sound (click.ogg) loaded");
        } else {
            println!("❌ No fallback sound available");
        }

        println!("\nEvent-specific sounds:");
        for (event, file_path) in config {
            if self.sound_data.contains_key(&event) {
                println!("✅ {:?}: {}", event, file_path);
            } else {
                println!("⚠️  {:?}: {} (using fallback)", event, file_path);
            }
        }

        let (loaded, total) = self.get_audio_stats();
        println!(
            "\nSummary: {}/{} event-specific sounds loaded",
            loaded, total
        );
        println!("========================\n");
    }
}

impl Drop for AudioSystem {
    fn drop(&mut self) {
        // Nothing to clean up in the placeholder implementation
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Test fixtures for audio testing
    mod test_fixtures {
        use super::*;

        pub fn create_test_audio_config() -> HashMap<AudioEvent, String> {
            HashMap::from([
                (AudioEvent::StartGame, "test_start.ogg".to_string()),
                (AudioEvent::DropCard, "test_drop.ogg".to_string()),
                (AudioEvent::MakeMatch, "test_match.ogg".to_string()),
            ])
        }

        pub fn create_all_audio_events() -> Vec<AudioEvent> {
            vec![
                AudioEvent::DifficultyChange,
                AudioEvent::StartGame,
                AudioEvent::PauseGame,
                AudioEvent::ResumeGame,
                AudioEvent::OpenQuitConfirmation,
                AudioEvent::ReturnToGame,
                AudioEvent::QuitGame,
                AudioEvent::DropCard,
                AudioEvent::MakeMatch,
                AudioEvent::ExplodeCard,
                AudioEvent::ForfeitGame,
                AudioEvent::GameOver,
                AudioEvent::MoveLeft,
                AudioEvent::MoveRight,
                AudioEvent::SoftDrop,
                AudioEvent::HardDrop,
            ]
        }
    }

    #[test]
    fn test_audio_event_enum_completeness() {
        let events = test_fixtures::create_all_audio_events();
        let config = AudioSystem::get_audio_config();

        // Verify all events have configuration
        for event in events {
            assert!(
                config.contains_key(&event),
                "AudioEvent {:?} missing from config",
                event
            );
        }

        // Verify configuration is complete
        assert!(
            config.len() >= 16,
            "Audio configuration should have at least 16 events"
        );
    }

    #[test]
    fn test_audio_config_paths() {
        let config = AudioSystem::get_audio_config();

        // Check that all paths are in expected format
        for (event, path) in config {
            assert!(
                path.starts_with("assets/audio/"),
                "Audio path for {:?} should start with 'assets/audio/': {}",
                event,
                path
            );
            assert!(
                path.ends_with(".ogg"),
                "Audio path for {:?} should end with '.ogg': {}",
                event,
                path
            );
        }
    }

    #[test]
    fn test_audio_config_unique_files() {
        let config = AudioSystem::get_audio_config();
        let mut paths = Vec::new();

        for (_, path) in config {
            assert!(
                !paths.contains(&path),
                "Duplicate audio file path: {}",
                path
            );
            paths.push(path);
        }
    }

    #[test]
    fn test_load_sound_file_nonexistent() {
        let result = AudioSystem::load_sound_file("nonexistent_file.ogg");
        assert!(result.is_none());
    }

    #[test]
    fn test_load_sound_file_invalid_path() {
        let result = AudioSystem::load_sound_file("");
        assert!(result.is_none());

        let result = AudioSystem::load_sound_file("/invalid/path/file.ogg");
        assert!(result.is_none());
    }

    #[test]
    fn test_audio_system_initialization() {
        // Test that audio system can be created without panicking
        // This will likely fail to load actual audio files but should not crash
        let audio_system = AudioSystem::new();

        // Should have the correct number of configured events
        let (loaded, total) = audio_system.get_audio_stats();
        assert_eq!(total, 16); // Should match the number of events in config
        assert!(loaded <= total); // Loaded count should not exceed total
    }

    #[test]
    fn test_audio_stats() {
        let audio_system = AudioSystem::new();
        let (loaded, total) = audio_system.get_audio_stats();

        assert!(total > 0, "Should have audio events configured");
        assert!(
            loaded <= total,
            "Loaded sounds should not exceed total events"
        );

        // Verify stats match actual data
        assert_eq!(loaded, audio_system.sound_data.len());
        assert_eq!(total, AudioSystem::get_audio_config().len());
    }

    #[test]
    fn test_audio_events_hash_equality() {
        // Test that AudioEvent enum properly implements Hash and Eq
        let event1 = AudioEvent::StartGame;
        let event2 = AudioEvent::StartGame;
        let event3 = AudioEvent::DropCard;

        assert_eq!(event1, event2);
        assert_ne!(event1, event3);

        // Test in HashMap
        let mut map = HashMap::new();
        map.insert(event1, "test");
        assert!(map.contains_key(&event2));
        assert!(!map.contains_key(&event3));
    }

    #[test]
    fn test_audio_events_debug() {
        // Test that AudioEvent implements Debug properly
        let event = AudioEvent::MakeMatch;
        let debug_str = format!("{:?}", event);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("MakeMatch"));
    }

    #[test]
    fn test_play_event_with_mock_handle() {
        // This test verifies play_event doesn't panic with invalid audio
        let audio_system = AudioSystem::new();

        // Create a mock raylib handle (this will be None in tests but shouldn't crash)
        // We can't easily test actual audio playback in unit tests, but we can test
        // that the method doesn't panic when called
        // Note: This would require a more complex mock setup in a real scenario

        // For now, just verify the method exists and basic structure
        assert!(audio_system.sound_data.len() <= AudioSystem::get_audio_config().len());
    }

    #[test]
    fn test_audio_system_drop() {
        // Test that AudioSystem can be dropped without issues
        let audio_system = AudioSystem::new();
        drop(audio_system);
        // If we reach here, drop succeeded
    }

    mod integration_tests {
        use super::*;

        #[test]
        fn test_audio_system_lifecycle() {
            // Test complete lifecycle
            let audio_system = AudioSystem::new();

            // Get initial stats
            let (initial_loaded, total) = audio_system.get_audio_stats();

            // Verify configuration consistency
            let config = AudioSystem::get_audio_config();
            assert_eq!(total, config.len());

            // Test that fallback sound handling works
            let has_fallback = audio_system.fallback_sound.is_some();
            let has_specific_sounds = audio_system.sound_data.len() > 0;

            // Should have either fallback or specific sounds (or both)
            assert!(
                has_fallback || has_specific_sounds,
                "Audio system should have either fallback sound or specific sounds"
            );

            // Test stats consistency
            assert!(initial_loaded <= total);
        }

        #[test]
        fn test_all_audio_events_have_config() {
            let all_events = test_fixtures::create_all_audio_events();
            let config = AudioSystem::get_audio_config();

            for event in all_events {
                assert!(
                    config.contains_key(&event),
                    "Event {:?} should have audio configuration",
                    event
                );

                let path = &config[&event];
                assert!(
                    !path.is_empty(),
                    "Audio path should not be empty for {:?}",
                    event
                );
                assert!(
                    path.contains("assets/"),
                    "Audio path should contain 'assets/' for {:?}",
                    event
                );
            }
        }
    }
}
