use thiserror::Error;

/// All errors that can occur in the engine.
#[derive(Debug, Error)]
pub enum NeutonError {
    #[error("SDL2 error: {0}")]
    Sdl2Error(String),

    #[error("Window creation failed: {0}")]
    WindowError(String),

    #[error("Canvas creation failed: {0}")]
    CanvasError(String),

    #[error("Failed to initialize logger: {0}")]
    LoggerError(String),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<std::io::Error> for NeutonError {
    fn from(err: std::io::Error) -> Self {
        NeutonError::Other(err.to_string())
    }
}

impl From<String> for NeutonError {
    fn from(err: String) -> Self {
        NeutonError::Other(err)
    }
}

/// Convenient result type for the engine.
pub type NeutonResult<T> = std::result::Result<T, NeutonError>;
