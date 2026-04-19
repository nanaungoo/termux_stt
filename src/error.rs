use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum SttError {
    #[error("Vosk model error: {0}")]
    #[diagnostic(
        code(stt::model_error),
        help("Ensure the model directory exists and is valid.")
    )]
    ModelError(String),

    #[error("Vosk recognizer error")]
    #[diagnostic(
        code(stt::recognizer_error),
        help("Failed to initialize the Vosk recognizer.")
    )]
    RecognizerError,

    #[error("IO error: {0}")]
    #[diagnostic(code(stt::io_error))]
    Io(#[from] std::io::Error),

    #[error("Audio format error: {0}")]
    #[diagnostic(code(stt::audio_format))]
    AudioFormat(String),

    #[error("Configuration error: {0}")]
    #[diagnostic(code(stt::config_error))]
    Config(String),

    #[error("Processing error: {0}")]
    #[diagnostic(code(stt::processing_error))]
    Processing(String),

    #[error("External command error: {0}")]
    #[diagnostic(code(stt::external_command))]
    ExternalCommand(String),
}

pub type Result<T> = std::result::Result<T, SttError>;
