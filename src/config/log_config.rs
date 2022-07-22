use serde::Deserialize;
use std::fmt;
use tracing::warn;

#[derive(Debug, PartialEq, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "Trace"),
            LogLevel::Debug => write!(f, "Debug"),
            LogLevel::Info => write!(f, "Info"),
            LogLevel::Warn => write!(f, "Warn"),
            LogLevel::Error => write!(f, "Error"),
            LogLevel::Fatal => write!(f, "Fatal"),
        }
    }
}

impl From<&str> for LogLevel {
    fn from(input: &str) -> Self {
        let uppercase_str = input.to_ascii_uppercase();
        match uppercase_str.as_str() {
            "TRACE" => LogLevel::Trace,
            "DEBUG" => LogLevel::Debug,
            "INFO" => LogLevel::Info,
            "WARN" => LogLevel::Warn,
            "ERROR" => LogLevel::Error,
            "FATAL" => LogLevel::Fatal,
            _ => {
                warn!("Log level supplied could not be parsed. Defaulting to log level: Warn.");
                LogLevel::Warn
            }
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct LogConfig {
    pub level: LogLevel, // Logging level.
}
