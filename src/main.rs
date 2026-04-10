use anyhow::{Context, Result};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use vosk::{Model, Recognizer};

#[derive(Debug, Clone)]
struct OwnedWord {
    pub start: f32,
    pub end: f32,
    pub word: String,
}

#[derive(Debug, Clone)]
struct OwnedResult {
    pub result: Vec<OwnedWord>,
    pub text: String,
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Setup Vosk
    let model_dir = "./vosk-model-small-en-us-0.15";
    let model_path = Path::new(model_dir);

    if !model_path.exists() {
        println!("Vosk model not found. Would you like to download it? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if input.trim().to_lowercase() == "y" {
            download_model()?;
        } else {
            return Err(anyhow::anyhow!("Model required to proceed."));
        }
    }

    println!("Loading Vosk model from {}...", model_dir);
    let model = Model::new(model_path.to_str().unwrap())
        .ok_or_else(|| anyhow::anyhow!("Failed to create Vosk model"))?;

    let mut recognizer = Recognizer::new(&model, 16000.0)
        .ok_or_else(|| anyhow::anyhow!("Failed to create Vosk recognizer"))?;
    recognizer.set_words(true);

    if args.len() > 1 {
        // File Processing Mode
        let audio_file_path = &args[1];
        println!("Processing file: {}...", audio_file_path);
        let results = transcribe_file(&mut recognizer, audio_file_path)?;
        
        let base_name = Path::new(audio_file_path).file_stem().and_then(|s| s.to_str()).unwrap_or("output");
        save_text(&results, &format!("{}.txt", base_name))?;
        save_srt(&results, &format!("{}.srt", base_name))?;
        println!("\nDone! Results saved to {}.txt and {}.srt", base_name, base_name);
    } else {
        // Real-time Streaming Mode
        if !is_termux_api_installed() {
            return Err(anyhow::anyhow!("termux-api not found. Please install it with 'pkg install termux-api'."));
        }
        println!("\n--- Real-time Transcription ---");
        println!("Press Enter to START/STOP recording.");
        let mut _l = String::new();
        io::stdin().read_line(&mut _l)?;

        stream_from_microphone(&mut recognizer)?;
    }

    Ok(())
}

fn stream_from_microphone(recognizer: &mut Recognizer) -> Result<()> {
    println!("Recording... (Press Enter in another terminal or Ctrl+C to stop)");
    
    // Start termux-microphone-record outputting to stdout
    let mut child = Command::new("termux-microphone-record")
        .arg("-f").arg("wav")
        .arg("-") // Stream to stdout
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to start termux-microphone-record")?;

    let mut stdout = child.stdout.take().unwrap();
    
    // Skip WAV header (44 bytes) for 16kHz mono 16-bit PCM
    let mut header = [0u8; 44];
    stdout.read_exact(&mut header)?;

    let mut buffer = [0u8; 3200]; // 0.1s of audio at 16kHz
    
    println!("Listening... (Speak now)\n");

    loop {
        let bytes_read = match stdout.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        // Convert u8 buffer to i16 samples
        let samples: Vec<i16> = buffer[..bytes_read]
            .chunks_exact(2)
            .map(|c| i16::from_le_bytes([c[0], c[1]]))
            .collect();

        match recognizer.accept_waveform(&samples) {
            Ok(vosk::DecodingState::Running) => {
                let partial = recognizer.partial_result().partial;
                if !partial.is_empty() {
                    print!("\rPartial: {}...", partial);
                    io::stdout().flush()?;
                }
            }
            Ok(vosk::DecodingState::Finalized) => {
                if let Some(res) = recognizer.result().single() {
                    println!("\rResult: {}", res.text);
                }
            }
            _ => {}
        }

        // Check if we should stop (check if process is still alive)
        if let Ok(Some(_)) = child.try_wait() {
            break;
        }
    }

    // Stop recording command
    let _ = Command::new("termux-microphone-record").arg("-q").status();
    let _ = child.kill();

    if let Some(res) = recognizer.final_result().single() {
        println!("\rFinal Result: {}", res.text);
    }

    Ok(())
}

fn transcribe_file(recognizer: &mut Recognizer, path: &str) -> Result<Vec<OwnedResult>> {
    let file = File::open(path).context("Failed to open audio file")?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if path.ends_with(".mp3") { hint.with_extension("mp3"); }
    if path.ends_with(".wav") { hint.with_extension("wav"); }

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .context("Unsupported audio format")?;

    let mut format = probed.format;
    let track = format.tracks().get(0).ok_or_else(|| anyhow::anyhow!("No audio track found"))?;
    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &DecoderOptions::default())?;

    let mut all_results = Vec::new();

    while let Ok(packet) = format.next_packet() {
        let decoded = decoder.decode(&packet)?;
        let spec = *decoded.spec();
        let duration = decoded.capacity() as u64;
        let mut sample_buffer = SampleBuffer::<i16>::new(duration, spec);
        sample_buffer.copy_interleaved_ref(decoded);

        if let Ok(vosk::DecodingState::Finalized) = recognizer.accept_waveform(sample_buffer.samples()) {
            if let Some(res) = recognizer.result().single() {
                all_results.push(OwnedResult {
                    text: res.text.to_string(),
                    result: res.result.iter().map(|w| OwnedWord {
                        start: w.start, end: w.end, word: w.word.to_string()
                    }).collect(),
                });
            }
        }
    }

    if let Some(res) = recognizer.final_result().single() {
        all_results.push(OwnedResult {
            text: res.text.to_string(),
            result: res.result.iter().map(|w| OwnedWord {
                start: w.start, end: w.end, word: w.word.to_string()
            }).collect(),
        });
    }

    Ok(all_results)
}

fn download_model() -> Result<()> {
    println!("Downloading Vosk model (English small)...");
    Command::new("wget")
        .arg("https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip")
        .status()?;
    Command::new("unzip")
        .arg("vosk-model-small-en-us-0.15.zip")
        .status()?;
    Ok(())
}

fn save_text(results: &[OwnedResult], path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    for res in results {
        if !res.text.is_empty() { writeln!(file, "{}", res.text)?; }
    }
    Ok(())
}

fn save_srt(results: &[OwnedResult], path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    let mut count = 1;
    for res in results {
        for chunk in res.result.chunks(8) {
            let start = chunk.first().unwrap().start;
            let end = chunk.last().unwrap().end;
            let text: Vec<&str> = chunk.iter().map(|w| w.word.as_str()).collect();
            writeln!(file, "{}", count)?;
            writeln!(file, "{} --> {}", format_timestamp(start as f64), format_timestamp(end as f64))?;
            writeln!(file, "{}\n", text.join(" "))?;
            count += 1;
        }
    }
    Ok(())
}

fn format_timestamp(seconds: f64) -> String {
    let h = (seconds / 3600.0).floor() as u32;
    let m = ((seconds % 3600.0) / 60.0).floor() as u32;
    let s = (seconds % 60.0).floor() as u32;
    let ms = ((seconds.fract() * 1000.0).round()) as u32;
    format!("{:02}:{:02}:{:02},{:03}", h, m, s, ms)
}

fn is_termux_api_installed() -> bool {
    Command::new("which").arg("termux-microphone-record")
        .stdout(Stdio::null()).status().map_or(false, |s| s.success())
}
