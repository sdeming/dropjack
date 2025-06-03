use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

/// Audio system for the DropJack game using rodio
/// 
/// Rodio is a dedicated Rust audio library that provides:
/// - Simple API: just load_sound() and play()
/// - Cross-platform: works on Windows, macOS, Linux, and web
/// - Multiple formats: OGG, WAV, MP3, FLAC out of the box
/// - No lifetime issues: sounds are owned by the audio system
/// - High performance: designed specifically for games
pub struct AudioSystem {
    _stream: OutputStream,      // Keep alive for entire program duration
    stream_handle: OutputStreamHandle,
    click_sound_data: Option<Vec<u8>>,  // Store the raw audio data
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
                // We still create the system but audio won't work
                return AudioSystem {
                    _stream: OutputStream::try_default().unwrap_or_else(|_| {
                        panic!("Failed to create fallback audio stream")
                    }).0,
                    stream_handle: OutputStream::try_default().unwrap().1,
                    click_sound_data: None,
                };
            }
        };

        // Try to load the click sound
        let click_sound_data = Self::load_sound_file("assets/audio/click.ogg");
        
        if click_sound_data.is_none() {
            eprintln!("Warning: Could not load audio file assets/audio/click.ogg");
        }

        AudioSystem {
            _stream: stream,
            stream_handle,
            click_sound_data,
        }
    }
    
    /// Play the click sound effect
    pub fn play_click(&self, _rl: &mut raylib::prelude::RaylibHandle) {
        if let Some(ref sound_data) = self.click_sound_data {
            // Create a new decoder from the sound data for each play
            // This allows multiple simultaneous plays of the same sound
            let cursor = std::io::Cursor::new(sound_data.clone());
            
            match Decoder::new(cursor) {
                Ok(source) => {
                    // Play the sound directly - rodio handles mixing automatically
                    if let Err(e) = self.stream_handle.play_raw(source.convert_samples()) {
                        eprintln!("Failed to play click sound: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to decode click sound: {}", e);
                }
            }
        }
    }
    
    /// Set the master volume (0.0 = silent, 1.0 = full volume)
    /// Note: This affects future sounds. For more complex volume control,
    /// we could implement a Sink-based system.
    pub fn set_volume(&self, _rl: &mut raylib::prelude::RaylibHandle, volume: f32) {
        // For now, we'll just store this for future enhancement
        // Rodio doesn't have a global volume, but we can implement this
        // by using Sink and controlling volume per-sound
        println!("Volume setting not yet implemented (requested: {})", volume.clamp(0.0, 1.0));
    }
    
    /// Load a sound file into memory
    /// Returns the raw bytes that can be decoded multiple times
    fn load_sound_file(path: &str) -> Option<Vec<u8>> {
        match std::fs::read(path) {
            Ok(data) => {
                println!("Successfully loaded audio file: {}", path);
                Some(data)
            }
            Err(e) => {
                eprintln!("Failed to load audio file {}: {}", path, e);
                None
            }
        }
    }
    
    /// Check if audio is working
    pub fn is_enabled(&self) -> bool {
        self.click_sound_data.is_some()
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