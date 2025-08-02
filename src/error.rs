use thiserror::Error;

/// All errors that can occur in the engine.
#[derive(Debug, Error)]
pub enum OxideError {
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

impl From<std::io::Error> for OxideError {
    fn from(err: std::io::Error) -> Self {
        OxideError::Other(err.to_string())
    }
}

impl From<String> for OxideError {
    fn from(err: String) -> Self {
        OxideError::Other(err)
    }
}

/// Convenient result type for the engine.
pub type OxideResult<T> = std::result::Result<T, OxideError>;
