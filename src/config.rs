use crate::error::{Result, SttError};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub model_path: String,
    pub sample_rate: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model_path: "./vosk-model-small-en-us-0.15".to_string(),
            sample_rate: 16000.0,
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
}
