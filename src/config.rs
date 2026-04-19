use crate::error::{Result, SttError};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    pub model_path: String,
    pub models_dir: String,
    pub sample_rate: f32,
    pub input_dir: String,
    pub output_dir: String,
    pub rpm: usize,
    pub tpm: usize,
    pub rpd: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model_path: "./models/vosk-model-en-us-0.22".to_string(),
            models_dir: "./models".to_string(),
            sample_rate: 16000.0,
            input_dir: "data/input".to_string(),
            output_dir: "data/output".to_string(),
            rpm: 15,
            tpm: 1_000_000,
            rpd: 1_500,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = "config.toml";
        if !Path::new(config_path).exists() {
            return Ok(Self::default());
        }

        let settings = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .map_err(|e| SttError::Config(e.to_string()))?;

        settings.try_deserialize().map_err(|e| SttError::Config(e.to_string()))
    }

    pub fn save(&self) -> Result<()> {
        let config_content = format!(
            "model_path = \"{}\"\n\
            models_dir = \"{}\"\n\
            sample_rate = {}\n\
            input_dir = \"{}\"\n\
            output_dir = \"{}\"\n\
            rpm = {}\n\
            tpm = {}\n\
            rpd = {}\n",
            self.model_path.replace('\\', "\\\\").replace('"', "\\\""),
            self.models_dir.replace('\\', "\\\\").replace('"', "\\\""),
            self.sample_rate,
            self.input_dir.replace('\\', "\\\\").replace('"', "\\\""),
            self.output_dir.replace('\\', "\\\\").replace('"', "\\\""),
            self.rpm,
            self.tpm,
            self.rpd
        );
        std::fs::write("config.toml", config_content).map_err(|e| SttError::Config(e.to_string()))?;
        Ok(())
    }
}
