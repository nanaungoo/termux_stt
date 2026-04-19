use clap::Parser;
use miette::IntoDiagnostic;
use std::path::Path;
use termux_stt::ui::{Cli, Commands};
use termux_stt::config::Config;
use termux_stt::{
    SttEngine, convert_video_to_mp3, download_model, save_srt, save_text, stream_from_microphone,
    transcribe_file,
};
use dialoguer::{theme::ColorfulTheme, Select};

#[tokio::main]
async fn main() -> miette::Result<()> {
    // Setup tracing/logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let mut config = Config::load().into_diagnostic()?;

    // Directory resolution: Use owned strings to avoid borrowing issues
    let input_dir = cli.input_dir.clone().unwrap_or_else(|| config.input_dir.clone());
    let output_dir = cli.output_dir.clone().unwrap_or_else(|| config.output_dir.clone());

    // Create directories if they don't exist
    std::fs::create_dir_all(&input_dir).into_diagnostic()?;
    std::fs::create_dir_all(&output_dir).into_diagnostic()?;

    let mut model_path_str = if cli.model != "./vosk-model-en-us-0.22" {
        cli.model.clone()
    } else {
        config.model_path.clone()
    };

    let command = if let Some(cmd) = &cli.command {
        cmd.clone()
    } else {
        loop {
            println!("🎤 Welcome to Termux STT Clean Edition!");
            let selections = &["Transcribe Audio File", "Record Real-time (Termux only)", "Configure Settings", "Exit"];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What would you like to do?")
                .default(0)
                .items(&selections[..])
                .interact()
                .into_diagnostic()?;

            match selection {
                0 => {
                    // 1. Language Model Selection
                    let model_entries = std::fs::read_dir(&config.models_dir).into_diagnostic()?;
                    let mut available_models: Vec<String> = model_entries
                        .filter_map(|entry| entry.ok())
                        .filter(|entry| entry.path().is_dir())
                        .map(|entry| entry.file_name().to_string_lossy().to_string())
                        .collect();
                    
                    if available_models.is_empty() {
                        println!("⚠️ No Vosk models found in {}. Downloading default English model...", config.models_dir);
                        download_model().await.into_diagnostic()?;
                        available_models.push("vosk-model-small-en-us-0.15".to_string());
                    }

                    available_models.sort();

                    let model_idx = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Choose a Language Model")
                        .default(0)
                        .items(&available_models[..])
                        .interact()
                        .into_diagnostic()?;
                    
                    let selected_model_name = &available_models[model_idx];
                    let full_model_path = Path::new(&config.models_dir).join(selected_model_name);
                    model_path_str = full_model_path.to_string_lossy().to_string();
                    
                    // Update and save config
                    config.model_path = model_path_str.clone();
                    let _ = config.save();

                    // 2. File Selection
                    let paths = std::fs::read_dir(&input_dir).into_diagnostic()?;
                    let mut files: Vec<String> = paths
                        .filter_map(|entry| entry.ok())
                        .map(|entry| entry.file_name().to_string_lossy().to_string())
                        .filter(|f| {
                            let f = f.to_lowercase();
                            f.ends_with(".mp3") || f.ends_with(".wav") || 
                            f.ends_with(".mp4") || f.ends_with(".mkv") || 
                            f.ends_with(".avi") || f.ends_with(".mov") ||
                            f.ends_with(".webm")
                        })
                        .collect();

                    if files.is_empty() {
                        println!("⚠️ No supported audio/video files found in {:?}.", input_dir);
                        continue;
                    }

                    files.push("Back".to_string());
                    let file_selection = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Choose a file to transcribe")
                        .default(0)
                        .items(&files[..])
                        .interact()
                        .into_diagnostic()?;

                    if file_selection == files.len() - 1 { continue; }
                    break Commands::Transcribe { file: files[file_selection].clone(), output: None };
                }
                1 => break Commands::Record,
                2 => {
                    let settings_modes = &["Select Default Input/Output Directories", "Back"];
                    let s_idx = Select::with_theme(&ColorfulTheme::default())
                        .with_prompt("Configure Settings")
                        .items(&settings_modes[..])
                        .interact()
                        .into_diagnostic()?;

                    if s_idx == 0 {
                        use dialoguer::Input;
                        config.input_dir = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter Input Directory")
                            .default(config.input_dir.clone())
                            .interact_text()
                            .into_diagnostic()?;
                        config.output_dir = Input::with_theme(&ColorfulTheme::default())
                            .with_prompt("Enter Output Directory")
                            .default(config.output_dir.clone())
                            .interact_text()
                            .into_diagnostic()?;
                        config.save().into_diagnostic()?;
                        println!("✅ Settings saved. (Restart recommended for changes to take effect)");
                    }
                    continue;
                }
                _ => return Ok(()),
            }
        }
    };

    let model_path = Path::new(&model_path_str);
    if !model_path.exists() {
        return Err(miette::miette!("Selected model not found at {}. Please check your models folder.", model_path_str));
    }

    let engine = SttEngine::new(&model_path_str).into_diagnostic()?;

    match command {
        Commands::Transcribe { file, output } => {
            let input_path = if Path::new(&file).exists() {
                Path::new(&file).to_path_buf()
            } else {
                Path::new(&input_dir).join(&file)
            };

            if !input_path.exists() {
                return Err(miette::miette!("Input file not found: {:?}", input_path));
            }

            let base_name = output.as_deref().unwrap_or_else(|| {
                input_path.file_stem().and_then(|s| s.to_str()).unwrap_or("output")
            });

            // Automatic Video to MP3 conversion
            let mut final_input_path = input_path.clone();
            let ext = input_path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
            if ext != "mp3" && ext != "wav" {
                let mp3_path = input_path.with_extension("mp3");
                convert_video_to_mp3(input_path.to_str().unwrap(), mp3_path.to_str().unwrap()).into_diagnostic()?;
                final_input_path = mp3_path;
            }

            let txt_path = Path::new(&output_dir).join(format!("{}.txt", base_name));
            let srt_path = Path::new(&output_dir).join(format!("{}.srt", base_name));

            println!("🚀 Transcribing file: {:?}...", final_input_path);
            let results = transcribe_file(&engine, final_input_path.to_str().unwrap()).into_diagnostic()?;
            
            save_text(&results, txt_path.to_str().unwrap()).into_diagnostic()?;
            save_srt(&results, srt_path.to_str().unwrap()).into_diagnostic()?;
            println!("\n✅ Done! Results saved to {:?} and {:?}", txt_path, srt_path);
        }
        Commands::Record => {
            let mut recognizer = engine.create_recognizer(config.sample_rate).into_diagnostic()?;
            stream_from_microphone(&mut recognizer).await.into_diagnostic()?;
        }
    }

    Ok(())
}
