use serde::{Deserialize, Serialize};
use std::fmt;
use tracing::warn;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Env {
    Development,
    Testing,
    Staging,
    Production,
}

impl fmt::Display for Env {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Env::Development => write!(f, "Development"),
            Env::Testing => write!(f, "Testing"),
            Env::Staging => write!(f, "Staging"),
            Env::Production => write!(f, "Production"),
        }
    }
}

impl From<String> for Env {
    fn from(input: String) -> Self {
        let uppercase_str = input.to_ascii_uppercase();
        match uppercase_str.as_str() {
            "DEVELOPMENT" => Env::Development,
            "TESTING" => Env::Testing,
            "STAGING" => Env::Staging,
            "PRODUCTION" => Env::Production,

            _ => {
                warn!(
                    "Application environment supplied could not be parsed. Defaulting to environment: Development."
                );
                Env::Development
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub config: String, // Filepath to the config file.
    pub env: Env,       // Application environment (DEVELOPMENT|STAGING|TESTING|PRODUCTION).
}
