use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::collections::HashMap;
use crate::game::AudioEvent;

/// Audio system for the DropJack game using rodio
/// 
/// Supports individual sound files for each of the 12 audio events.
/// Falls back to click.ogg if specific event sounds are missing.
pub struct AudioSystem {
    _stream: OutputStream,      // Keep alive for entire program duration
    stream_handle: OutputStreamHandle,
    sound_data: HashMap<AudioEvent, Vec<u8>>,  // Event-specific audio data
    fallback_sound: Option<Vec<u8>>,           // Fallback click.ogg for missing sounds
}

impl AudioSystem {
    /// Initialize the audio system using rodio
    pub fn new(_rl: &mut raylib::prelude::RaylibHandle, _thread: &raylib::prelude::RaylibThread) -> Self {
        // Initialize rodio output stream
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok((stream, handle)) => {
                println!("Audio system initialized successfully with rodio");
                (stream, handle)
            }
            Err(e) => {
                eprintln!("Warning: Could not initialize audio: {}", e);
                return AudioSystem {
                    _stream: OutputStream::try_default().unwrap_or_else(|_| {
                        panic!("Failed to create fallback audio stream")
                    }).0,
                    stream_handle: OutputStream::try_default().unwrap().1,
                    sound_data: HashMap::new(),
                    fallback_sound: None,
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
                println!("Using fallback sound for {:?} (missing: {})", event, file_path);
            }
        }

        AudioSystem {
            _stream: stream,
            stream_handle,
            sound_data,
            fallback_sound,
        }
    }
    
    /// Play sound for a specific audio event
    pub fn play_event(&self, event: AudioEvent, _rl: &mut raylib::prelude::RaylibHandle) {
        // Try to get event-specific sound, fall back to click.ogg if not found
        let sound_data = self.sound_data.get(&event)
            .or(self.fallback_sound.as_ref());
            
        if let Some(data) = sound_data {
            // Create a new decoder from the sound data for each play
            let cursor = std::io::Cursor::new(data.clone());
            
            match Decoder::new(cursor) {
                Ok(source) => {
                    // Play the sound directly - rodio handles mixing automatically
                    if let Err(e) = self.stream_handle.play_raw(source.convert_samples()) {
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
    
    /// Backwards compatibility: play click sound (now maps to a default event)
    pub fn play_click(&self, rl: &mut raylib::prelude::RaylibHandle) {
        self.play_event(AudioEvent::DropCard, rl);
    }
    
    /// Configuration mapping: AudioEvent -> file path
    /// 
    /// To customize audio, modify these file paths or add the corresponding
    /// audio files to your assets/audio/ directory.
    fn get_audio_config() -> HashMap<AudioEvent, String> {
        let mut config = HashMap::new();
        
        // UI and Menu Sounds
        config.insert(AudioEvent::DifficultyChange, "assets/audio/difficulty_change.ogg".to_string());
        config.insert(AudioEvent::StartGame, "assets/audio/start_game.ogg".to_string());
        config.insert(AudioEvent::PauseGame, "assets/audio/pause.ogg".to_string());
        config.insert(AudioEvent::ResumeGame, "assets/audio/resume.ogg".to_string());
        config.insert(AudioEvent::OpenQuitConfirmation, "assets/audio/open_quit.ogg".to_string());
        config.insert(AudioEvent::ReturnToGame, "assets/audio/return_to_game.ogg".to_string());
        config.insert(AudioEvent::QuitGame, "assets/audio/quit.ogg".to_string());
        
        // Gameplay Sounds
        config.insert(AudioEvent::DropCard, "assets/audio/drop_card.ogg".to_string());
        config.insert(AudioEvent::MakeMatch, "assets/audio/make_match.ogg".to_string());
        config.insert(AudioEvent::ExplodeCard, "assets/audio/explode_card.ogg".to_string());
        config.insert(AudioEvent::ForfeitGame, "assets/audio/forfeit.ogg".to_string());
        config.insert(AudioEvent::GameOver, "assets/audio/game_over.ogg".to_string());
        
        // Card Movement Sounds
        config.insert(AudioEvent::MoveLeft, "assets/audio/move_left.ogg".to_string());
        config.insert(AudioEvent::MoveRight, "assets/audio/move_right.ogg".to_string());
        config.insert(AudioEvent::SoftDrop, "assets/audio/soft_drop.ogg".to_string());
        
        config
    }
    
    /// Set the master volume (0.0 = silent, 1.0 = full volume)
    /// Note: For more complex volume control, we could implement a Sink-based system.
    pub fn set_volume(&self, _rl: &mut raylib::prelude::RaylibHandle, volume: f32) {
        // For now, we'll just store this for future enhancement
        println!("Volume setting not yet implemented (requested: {})", volume.clamp(0.0, 1.0));
    }
    
    /// Load a sound file into memory
    /// Returns the raw bytes that can be decoded multiple times
    fn load_sound_file(path: &str) -> Option<Vec<u8>> {
        match std::fs::read(path) {
            Ok(data) => {
                Some(data)
            }
            Err(_) => {
                // Don't spam errors for missing files - they're optional
                None
            }
        }
    }
    
    /// Check if audio is working (has at least fallback sound)
    pub fn is_enabled(&self) -> bool {
        self.fallback_sound.is_some() || !self.sound_data.is_empty()
    }
    
    /// Get statistics about loaded sounds
    pub fn get_audio_stats(&self) -> (usize, usize) {
        let specific_sounds = self.sound_data.len();
        let total_possible = Self::get_audio_config().len();
        (specific_sounds, total_possible)
    }
    
    /// List which sounds are loaded and which are using fallback
    pub fn print_audio_status(&self) {
        let config = Self::get_audio_config();
        println!("\n=== Audio System Status ===");
        
        if let Some(_) = &self.fallback_sound {
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
        println!("\nSummary: {}/{} event-specific sounds loaded", loaded, total);
        println!("========================\n");
    }
}

// Additional helper for future expansion
impl AudioSystem {
    /// Create a new sink for more complex audio control
    /// This could be used for background music, etc.
    pub fn create_sink(&self) -> Result<Sink, rodio::PlayError> {
        Sink::try_new(&self.stream_handle)
    }
}

impl Drop for AudioSystem {
    fn drop(&mut self) {
        // Nothing to clean up in the placeholder implementation
    }
} 