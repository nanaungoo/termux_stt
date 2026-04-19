pub mod config;
pub mod engine;
pub mod error;
pub mod ui;

// Re-export core types for convenience
pub use engine::audio::{convert_video_to_mp3, save_srt, save_text, transcribe_file};
pub use engine::vosk::{OwnedResult, OwnedWord, SttEngine, download_model, stream_from_microphone};
pub use error::{Result, SttError};
