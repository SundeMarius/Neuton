use std::path::Path;

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{OxideResult, error::OxideError};

/// Initializes the global logger for the engine.
///
/// Should be called once at the start of your application.
pub fn init_logger(log_directory: Option<&Path>) -> OxideResult<()> {
    // Read filter from RUST_LOG environment variable or default to "info"
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_target(false)
        .with_line_number(false);

    let file_appender = tracing_appender::rolling::daily(
        log_directory
            .or(dirs::home_dir().map(|f| f.join(".oxide/logs")).as_deref())
            .ok_or(OxideError::LoggerError(
                "No available log directory".to_string(),
            ))?,
        "log",
    );
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_appender)
        .with_target(false);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .with(file_layer)
        .init();

    Ok(())
}

/// Macro for logging with different levels
/// This macro uses the `tracing` crate for structured logging.
/// It allows you to log messages at different levels (trace, debug, info, warn, error).
/// Example usage: `log!(info, "Your message here: {}", value);`
#[macro_export]
macro_rules! log {
    (trace, $($arg:tt)+) => {
        ::tracing::trace!($($arg)+)
    };
    (debug, $($arg:tt)+) => {
        ::tracing::debug!($($arg)+)
    };
    (info, $($arg:tt)+) => {
        ::tracing::info!($($arg)+)
    };
    (warn, $($arg:tt)+) => {
        ::tracing::warn!($($arg)+)
    };
    (error, $($arg:tt)+) => {
        ::tracing::error!($($arg)+)
    };
}
