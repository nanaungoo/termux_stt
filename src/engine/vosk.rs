use crate::error::{Result, SttError};
use std::process::Command;
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
        let model =
            Model::new(model_path).ok_or_else(|| SttError::ModelError(model_path.to_string()))?;
        Ok(Self { model })
    }

    pub fn create_recognizer(&self, sample_rate: f32) -> Result<Recognizer> {
        let mut recognizer =
            Recognizer::new(&self.model, sample_rate).ok_or_else(|| SttError::RecognizerError)?;
        recognizer.set_words(true);
        Ok(recognizer)
    }
}

#[cfg(target_os = "android")]
pub async fn stream_from_microphone(recognizer: &mut Recognizer) -> Result<()> {
    use std::process::Stdio;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::process::Command;

    let mut child = Command::new("termux-microphone-record")
        .arg("-f")
        .arg("wav")
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| {
            SttError::ExternalCommand(format!("Failed to start termux-microphone-record: {}", e))
        })?;

    let mut stdout = child
        .stdout
        .take()
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

    let _ = Command::new("termux-microphone-record")
        .arg("-q")
        .status()
        .await;
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
    Err(SttError::ExternalCommand(
        "Real-time transcription is only implemented for Termux.".to_string(),
    ))
}

pub async fn download_model() -> Result<()> {
    println!("Downloading Vosk model (English Large - 0.22)...");
    println!("Note: This is a large file (~1.8GB), please ensure you have enough space.");
    let status = Command::new("wget")
        .arg("-c")
        .arg("https://alphacephei.com/vosk/models/vosk-model-en-us-0.22.zip")
        .status()?;
    if !status.success() {
        return Err(SttError::ExternalCommand(
            "Failed to download model".to_string(),
        ));
    }

    println!("Extracting model...");
    let status = Command::new("unzip")
        .arg("-o")
        .arg("vosk-model-en-us-0.22.zip")
        .status()?;
    if !status.success() {
        return Err(SttError::ExternalCommand(
            "Failed to unzip model".to_string(),
        ));
    }
    // Optional: rm vosk-model-en-us-0.22.zip after extraction if desired
    Ok(())
}
