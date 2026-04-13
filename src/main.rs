use clap::Parser;
use miette::IntoDiagnostic;
use std::io::{self, Write};
use std::path::Path;
use termux_stt::cli::{Cli, Commands};
use termux_stt::config::Config;
use termux_stt::{download_model, save_srt, save_text, stream_from_microphone, transcribe_file, SttEngine};

#[tokio::main]
async fn main() -> miette::Result<()> {
    // Setup tracing/logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let config = Config::load().into_diagnostic()?;

    // Model path priority: CLI argument > Config file > Default
    let model_path_str = if cli.model != "./vosk-model-small-en-us-0.15" {
        &cli.model
    } else {
        &config.model_path
    };

    let model_path = Path::new(model_path_str);
    if !model_path.exists() {
        println!("Vosk model not found at {}. Would you like to download it? (y/n)", model_path_str);
        print!("> ");
        io::stdout().flush().into_diagnostic()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).into_diagnostic()?;
        if input.trim().to_lowercase() == "y" {
            download_model().await.into_diagnostic()?;
        } else {
            return Err(miette::miette!("Model required to proceed. Please download it or specify a valid path."));
        }
    }

    let engine = SttEngine::new(model_path_str).into_diagnostic()?;
    let mut recognizer = engine.create_recognizer(config.sample_rate).into_diagnostic()?;

    match &cli.command {
        Some(Commands::Transcribe { file, output }) => {
            println!("Processing file: {}...", file);
            let results = transcribe_file(&mut recognizer, file).into_diagnostic()?;
            
            let base_name = output.as_deref().unwrap_or_else(|| {
                Path::new(file).file_stem().and_then(|s| s.to_str()).unwrap_or("output")
            });

            save_text(&results, &format!("{}.txt", base_name)).into_diagnostic()?;
            save_srt(&results, &format!("{}.srt", base_name)).into_diagnostic()?;
            println!("\nDone! Results saved to {}.txt and {}.srt", base_name, base_name);
        }
        Some(Commands::Record) | None => {
            stream_from_microphone(&mut recognizer).await.into_diagnostic()?;
        }
    }

    Ok(())
}
