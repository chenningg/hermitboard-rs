mod config_error;

use self::config_error::ConfigError;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
enum AppEnv {
    Development,
    Testing,
    Staging,
    Production,
}

impl std::str::FromStr for AppEnv {
    type Err = ConfigError;

    fn from_str(input: &str) -> Result<AppEnv, ConfigError> {
        let uppercase_str = input.to_ascii_uppercase();
        match uppercase_str.as_str() {
            "DEVELOPMENT" => Ok(AppEnv::Development),
            "TESTING" => Ok(AppEnv::Testing),
            "STAGING" => Ok(AppEnv::Staging),
            "PRODUCTION" => Ok(AppEnv::Production),
            _ => Err(ConfigError::InvalidAppEnv {
                expected: "DEVELOPMENT|TESTING|STAGING|PRODUCTION".into(),
                key: "HERMITBOARD_APP_ENV".into(),
                actual: input.into(),
            }),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
struct Database {
    uri: String, // URI pointing to the backing database.
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct Config {
    config: String,  // File path to the config file.
    app_env: AppEnv, // Application environment (DEVELOPMENT|STAGING|TESTING|PRODUCTION)
    database: Database,
}

pub fn load_config() -> Config {
    // Load environment variable to get config file path. Panic if config file path is not supplied.
    let config_file_path = Env::var("HERMITBOARD_CONFIG")
        .expect("Could not find the environment variable value for the key 'HERMITBOARD_CONFIG', which is required for Hermitboard to load the application configuration. Please specify a valid file path to the configuration file.");

    // Initialize config struct and read config file. Panic if config values or config file cannot be found.
    let config = Figment::new()
        .join(Env::prefixed("HERMITBOARD_"))
        .merge(Toml::file(&config_file_path))
        .extract().unwrap_or_else(|err| panic!("Config file invalid or could not be found at the file path specified by environment variable 'HERMITBOARD_CONFIG' ({}). More info: {}", &config_file_path, &err.to_string()));

    config
}
