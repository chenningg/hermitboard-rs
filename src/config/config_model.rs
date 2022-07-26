use super::app_config::AppConfig;
use super::log_config::LogConfig;
use crate::database::database_config::DatabaseConfig;
use crate::graph::graph_config::GraphConfig;
use crate::server::server_config::ServerConfig;
use serde::{Deserialize, Serialize};
use std::fmt;
use tracing::warn;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub server: ServerConfig,
    pub graph: GraphConfig,
    pub log: LogConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ConfigFileType {
    Toml,
    Yaml,
    Json,
}

impl fmt::Display for ConfigFileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigFileType::Toml => write!(f, "Toml"),
            ConfigFileType::Yaml => write!(f, "Yaml"),
            ConfigFileType::Json => write!(f, "Json"),
        }
    }
}

impl From<String> for ConfigFileType {
    fn from(input: String) -> Self {
        let uppercase_str = input.to_ascii_uppercase();
        match uppercase_str.as_str() {
            "TOML" => ConfigFileType::Toml,
            "YAML" => ConfigFileType::Yaml,
            "JSON" => ConfigFileType::Json,
            _ => {
                warn!("Config file type supplied could not be parsed. Defaulting to config file type: Toml.");
                ConfigFileType::Toml
            }
        }
    }
}
