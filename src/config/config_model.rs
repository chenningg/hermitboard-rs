use super::app_config::AppConfig;
use super::log_config::LogConfig;
use crate::database::database_config::DatabaseConfig;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub log: LogConfig,
    pub database: DatabaseConfig,
}
