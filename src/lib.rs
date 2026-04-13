pub mod cli;
pub mod config;
pub mod core;
pub mod error;

pub use core::*;
pub use error::{Result, SttError};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.sample_rate, 16000.0);
        assert_eq!(config.model_path, "./vosk-model-small-en-us-0.15");
    }

    #[test]
    fn test_engine_init_fail() {
        // This should fail because the path doesn't exist (unless the user has it)
        let engine = SttEngine::new("non_existent_path");
        assert!(engine.is_err());
    }
}
