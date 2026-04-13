use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "termux_stt")]
#[command(version, about = "Speech-to-Text tool for Termux and Ubuntu", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Path to the Vosk model
    #[arg(short, long, default_value = "./vosk-model-small-en-us-0.15")]
    pub model: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Transcribe an audio file (mp3, wav)
    Transcribe {
        /// Path to the audio file
        file: String,
        /// Output base name (default: same as input)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Real-time transcription from microphone (Termux only)
    Record,
}
