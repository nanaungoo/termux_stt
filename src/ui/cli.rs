use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "termux_stt")]
#[command(version, about = "Speech-to-Text tool for Termux and Ubuntu", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Path to the Vosk model
    #[arg(short, long, default_value = "./vosk-model-en-us-0.22")]
    pub model: String,

    /// Directory to search for input files if a partial path is given
    #[arg(long)]
    pub input_dir: Option<String>,

    /// Directory to save output files
    #[arg(long)]
    pub output_dir: Option<String>,
}

#[derive(Subcommand, Clone)]
pub enum Commands {
    /// Transcribe an audio file (mp3, wav)
    Transcribe {
        /// Path to the audio file
        file: String,
        /// Output base name (default: same as input)
        #[arg(short, long)]
        output: Option<String>,
        /// Custom output directory for this task
        #[arg(long)]
        out_dir: Option<String>,
    },
    /// Real-time transcription from microphone (Termux only)
    Record,
}
