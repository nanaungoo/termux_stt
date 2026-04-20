use crate::engine::vosk::{OwnedResult, OwnedWord, SttEngine};
use crate::error::{Result, SttError};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::Write;
use symphonia::core::audio::SampleBuffer;
use symphonia::core::codecs::DecoderOptions;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use std::process::Command;

pub fn ensure_wav_format(input_path: &str, output_path: &str) -> Result<()> {
    println!("🎬 Converting {} to optimized WAV (16kHz, Mono, PCM)...", input_path);
    
    // Optimized ffmpeg flags:
    // -ar 16000: Set sample rate to 16kHz (Vosk standard)
    // -ac 1: Convert to Mono
    // -c:a pcm_s16le: Force 16-bit Little Endian PCM (Required for most STT)
    // -map_metadata -1: Strip metadata to prevent demuxer "junk" errors
    // -fflags +bitexact: Ensure a clean, standard header
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-ar")
        .arg("16000")
        .arg("-ac")
        .arg("1")
        .arg("-c:a")
        .arg("pcm_s16le")
        .arg("-map_metadata")
        .arg("-1")
        .arg("-fflags")
        .arg("+bitexact")
        .arg("-y") // Overwrite
        .arg(output_path)
        .status()
        .map_err(|e| SttError::ExternalCommand(format!("Failed to run ffmpeg: {}", e)))?;

    if !status.success() {
        return Err(SttError::ExternalCommand(
            format!("ffmpeg failed to convert media: {}", input_path),
        ));
    }
    println!("✅ Optimized WAV created: {}", output_path);
    Ok(())
}

pub fn transcribe_file(engine: &SttEngine, path: &str) -> Result<Vec<OwnedResult>> {
    let file = File::open(path)?;
    let metadata = file.metadata().map_err(|e| SttError::Io(e))?;
    let file_size = metadata.len();

    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    let lower_path = path.to_lowercase();
    if lower_path.ends_with(".mp3") {
        hint.with_extension("mp3");
    } else if lower_path.ends_with(".wav") {
        hint.with_extension("wav");
    }

    let probed = symphonia::default::get_probe()
        .format(
            &hint,
            mss,
            &FormatOptions::default(),
            &MetadataOptions::default(),
        )
        .map_err(|e| SttError::AudioFormat(e.to_string()))?;

    let mut format = probed.format;
    let track = format
        .tracks()
        .get(0)
        .ok_or_else(|| SttError::AudioFormat("No audio track found".to_string()))?;

    let sample_rate = track.codec_params.sample_rate.unwrap_or(16000) as f32;
    let channels = track.codec_params.channels.map(|c| c.count()).unwrap_or(1);

    println!(
        "Audio detected: {} Hz, {} channel(s)",
        sample_rate, channels
    );

    let mut recognizer = engine.create_recognizer(sample_rate)?;

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
        pb.inc(packet.data.len() as u64);

        let decoded = decoder
            .decode(&packet)
            .map_err(|e| SttError::AudioFormat(e.to_string()))?;

        let spec = *decoded.spec();
        let duration = decoded.capacity() as u64;
        let mut sample_buffer = SampleBuffer::<i16>::new(duration, spec);
        sample_buffer.copy_interleaved_ref(decoded);

        let samples = sample_buffer.samples();
        let mono_samples: Vec<i16> = if channels > 1 {
            samples
                .chunks_exact(channels)
                .map(|chunk| {
                    let sum: i32 = chunk.iter().map(|&s| s as i32).sum();
                    (sum / channels as i32) as i16
                })
                .collect()
        } else {
            samples.to_vec()
        };

        if let Ok(vosk::DecodingState::Finalized) = recognizer.accept_waveform(&mono_samples) {
            if let Some(res) = recognizer.result().single() {
                all_results.push(OwnedResult {
                    text: res.text.to_string(),
                    result: res
                        .result
                        .iter()
                        .map(|w| OwnedWord {
                            start: w.start,
                            end: w.end,
                            word: w.word.to_string(),
                        })
                        .collect(),
                });
            }
        }
    }

    if let Some(res) = recognizer.final_result().single() {
        all_results.push(OwnedResult {
            text: res.text.to_string(),
            result: res
                .result
                .iter()
                .map(|w| OwnedWord {
                    start: w.start,
                    end: w.end,
                    word: w.word.to_string(),
                })
                .collect(),
        });
    }

    pb.finish_with_message("Transcription complete");
    Ok(all_results)
}

pub fn save_text(results: &[OwnedResult], path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    for res in results {
        if !res.text.is_empty() {
            writeln!(file, "{}", res.text)?;
        }
    }
    Ok(())
}

pub fn save_srt(results: &[OwnedResult], path: &str) -> Result<()> {
    let mut file = File::create(path)?;
    let mut count = 1;
    for res in results {
        for chunk in res.result.chunks(8) {
            let start = chunk
                .first()
                .ok_or_else(|| {
                    SttError::Processing("Empty chunk in subtitle generation".to_string())
                })?
                .start;
            let end = chunk
                .last()
                .ok_or_else(|| {
                    SttError::Processing("Empty chunk in subtitle generation".to_string())
                })?
                .end;
            let text: Vec<&str> = chunk.iter().map(|w| w.word.as_str()).collect();
            writeln!(file, "{}", count)?;
            writeln!(
                file,
                "{} --> {}",
                format_timestamp(start as f64),
                format_timestamp(end as f64)
            )?;
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
            OwnedResult {
                text: "Hello world".to_string(),
                result: vec![],
            },
            OwnedResult {
                text: "Testing STT".to_string(),
                result: vec![],
            },
        ];
        let path = "test_output.txt";
        save_text(&results, path)?;
        let content = fs::read_to_string(path).map_err(SttError::Io)?;
        assert_eq!(content, "Hello world\nTesting STT\n");
        fs::remove_file(path).map_err(SttError::Io)?;
        Ok(())
    }
}
