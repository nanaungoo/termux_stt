use crate::error::{Result, SttError};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Write;
#[cfg(target_os = "android")]
use std::io::{self, Read};
use std::process::Command;
#[cfg(target_os = "android")]
use std::process::Stdio;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use vosk::{Model, Recognizer};

#[derive(Debug, Clone)]
pub struct OwnedWord {
    pub start: f32,
    pub end: f32,
    pub word: String,
}

#[derive(Debug, Clone)]
pub struct OwnedResult {
    pub result: Vec<OwnedWord>,
    pub text: String,
}

pub struct SttEngine {
    pub model: Model,
}

impl SttEngine {
    pub fn new(model_path: &str) -> Result<Self> {
        let model = Model::new(model_path)
            .ok_or_else(|| SttError::ModelError(model_path.to_string()))?;
        Ok(Self { model })
    }

    pub fn create_recognizer(&self, sample_rate: f32) -> Result<Recognizer> {
        let mut recognizer = Recognizer::new(&self.model, sample_rate)
            .ok_or_else(|| SttError::RecognizerError)?;
        recognizer.set_words(true);
        Ok(recognizer)
    }
}

pub fn transcribe_file(recognizer: &mut Recognizer, path: &str) -> Result<Vec<OwnedResult>> {
    let file = File::open(path)?;
    let metadata = file.metadata().map_err(|e| SttError::Io(e))?;
    let file_size = metadata.len();
    
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if path.ends_with(".mp3") { hint.with_extension("mp3"); }
    if path.ends_with(".wav") { hint.with_extension("wav"); }

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())
        .map_err(|e| SttError::AudioFormat(e.to_string()))?;

    let mut format = probed.format;
    let track = format.tracks().get(0)
        .ok_or_else(|| SttError::AudioFormat("No audio track found".to_string()))?;
    
    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &DecoderOptions::default())
        .map_err(|e| SttError::AudioFormat(e.to_string()))?;

    let mut all_results = Vec::new();
    
    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .map_err(|e| SttError::Config(format!("Invalid progress bar style: {}", e)))?
        .progress_chars("#>-"));

    while let Ok(packet) = format.next_packet() {
        // Update progress bar position by packet size.
        pb.inc(packet.data.len() as u64);
        
        let decoded = decoder.decode(&packet)
            .map_err(|e| SttError::AudioFormat(e.to_string()))?;
        
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

    pb.finish_with_message("Transcription complete");
    Ok(all_results)
}

#[cfg(target_os = "android")]
pub async fn stream_from_microphone(recognizer: &mut Recognizer) -> Result<()> {
    use tokio::process::Command;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use std::process::Stdio;

    let mut child = Command::new("termux-microphone-record")
        .arg("-f").arg("wav")
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| SttError::ExternalCommand(format!("Failed to start termux-microphone-record: {}", e)))?;

    let mut stdout = child.stdout.take()
        .ok_or_else(|| SttError::ExternalCommand("Failed to capture stdout".to_string()))?;
    
    let mut header = [0u8; 44];
    stdout.read_exact(&mut header).await?;

    let mut buffer = [0u8; 3200];
    let mut stdout_handle = tokio::io::stdout();
    
    loop {
        let bytes_read = match stdout.read(&mut buffer).await {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => break,
        };

        let samples: Vec<i16> = buffer[..bytes_read]
            .chunks_exact(2)
            .map(|c| i16::from_le_bytes([c[0], c[1]]))
            .collect();

        match recognizer.accept_waveform(&samples) {
            Ok(vosk::DecodingState::Running) => {
                let partial = recognizer.partial_result().partial;
                if !partial.is_empty() {
                    let msg = format!("\rPartial: {}...", partial);
                    stdout_handle.write_all(msg.as_bytes()).await?;
                    stdout_handle.flush().await?;
                }
            }
            Ok(vosk::DecodingState::Finalized) => {
                if let Some(res) = recognizer.result().single() {
                    let msg = format!("\rResult: {}\n", res.text);
                    stdout_handle.write_all(msg.as_bytes()).await?;
                    stdout_handle.flush().await?;
                }
            }
            _ => {}
        }

        if let Ok(Some(_)) = child.try_wait() {
            break;
        }
    }

    let _ = Command::new("termux-microphone-record").arg("-q").status().await;
    let _ = child.kill().await;

    if let Some(res) = recognizer.final_result().single() {
        let msg = format!("\rFinal Result: {}\n", res.text);
        stdout_handle.write_all(msg.as_bytes()).await?;
        stdout_handle.flush().await?;
    }

    Ok(())
}

#[cfg(not(target_os = "android"))]
pub async fn stream_from_microphone(_recognizer: &mut Recognizer) -> Result<()> {
    Err(SttError::ExternalCommand("Real-time transcription is only implemented for Termux.".to_string()))
}

pub fn save_text(results: &[OwnedResult], path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    for res in results {
        if !res.text.is_empty() { writeln!(file, "{}", res.text)?; }
    }
    Ok(())
}

pub fn save_srt(results: &[OwnedResult], path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    let mut count = 1;
    for res in results {
        for chunk in res.result.chunks(8) {
            let start = chunk.first().ok_or_else(|| SttError::Processing("Empty chunk in subtitle generation".to_string()))?.start;
            let end = chunk.last().ok_or_else(|| SttError::Processing("Empty chunk in subtitle generation".to_string()))?.end;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_format_timestamp() {
        assert_eq!(format_timestamp(0.0), "00:00:00,000");
        assert_eq!(format_timestamp(1.5), "00:00:01,500");
        assert_eq!(format_timestamp(3661.123), "01:01:01,123");
    }

    #[test]
    fn test_save_text() -> Result<()> {
        let results = vec![
            OwnedResult { text: "Hello world".to_string(), result: vec![] },
            OwnedResult { text: "Testing STT".to_string(), result: vec![] },
        ];
        let path = "test_output.txt";
        save_text(&results, path)?;
        
        let content = fs::read_to_string(path).map_err(SttError::Io)?;
        assert_eq!(content, "Hello world\nTesting STT\n");
        
        fs::remove_file(path).map_err(SttError::Io)?;
        Ok(())
    }

    #[test]
    fn test_save_srt() -> Result<()> {
        let results = vec![
            OwnedResult {
                text: "Hello".to_string(),
                result: vec![
                    OwnedWord { start: 0.0, end: 1.0, word: "Hello".to_string() }
                ],
            },
        ];
        let path = "test_output.srt";
        save_srt(&results, path)?;
        
        let content = fs::read_to_string(path).map_err(SttError::Io)?;
        assert!(content.contains("1"));
        assert!(content.contains("00:00:00,000 --> 00:00:01,000"));
        assert!(content.contains("Hello"));
        
        fs::remove_file(path).map_err(SttError::Io)?;
        Ok(())
    }
}

pub async fn download_model() -> Result<()> {
    println!("Downloading Vosk model (English small)...");
    let status = Command::new("wget")
        .arg("-c")
        .arg("https://alphacephei.com/vosk/models/vosk-model-small-en-us-0.15.zip")
        .status()?;
    if !status.success() {
        return Err(SttError::ExternalCommand("Failed to download model".to_string()));
    }

    let status = Command::new("unzip")
        .arg("-o")
        .arg("vosk-model-small-en-us-0.15.zip")
        .status()?;
    if !status.success() {
        return Err(SttError::ExternalCommand("Failed to unzip model".to_string()));
    }
    Ok(())
}
