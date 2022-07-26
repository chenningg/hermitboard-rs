use super::config_model::{Config, ConfigFileType};
use figment::{
    providers::{Env, Format, Json, Toml, Yaml},
    Figment,
};
use tracing::{error, warn};

pub trait ConfigService {
    fn new() -> Self;
    fn load_config(self) -> Self;
    fn config(&self) -> &Config;
}

#[derive(Debug)]
pub struct ConfigServiceImpl {
    config: Config,
}

impl ConfigService for ConfigServiceImpl {
    // Initializes a new ConfigService, holding a Config struct loaded from environment variables.
    fn new() -> Self {
        // Load config from environment variables
        let loaded_config = Figment::new()
            .merge(Env::prefixed("HERMITBOARD").split("_"))
            .extract()
            .unwrap_or_else(|err| {
                // Failed to read config, panic.
                error!("{}", err);
                panic!("{}", err);
            });

        ConfigServiceImpl {
            config: loaded_config,
        }
    }

    // Loads configuration from file or environment variables.
    fn load_config(mut self) -> Self {
        // Check if we need to load in a config file, else just load from environment variables.
        if let Some(config_file_path) = Env::var("HERMITBOARD_APP_CONFIG") {
            // Check if config file type is specified.
            let config_file_type = Env::var("HERMITBOARD_APP_CONFIG_TYPE").unwrap_or_else(|| {
                warn!("No config file type specified at environment key 'HERMITBOARD_APP_CONFIG_TYPE'. Defaulting to config file type: Toml.");
                String::from("Toml")
            }).into();

            // Match based on config file type and load in config file.
            let loaded_config = match config_file_type {
                ConfigFileType::Toml => {
                    Figment::new()
                        .merge(Toml::file(&config_file_path))
                        .merge(Env::prefixed("HERMITBOARD").split("_"))
                        .extract()
                        .unwrap_or_else(|err| {
                            // Failed to read config, panic.
                            error!("{}", err);
                            panic!("{}", err)
                        })
                }
                ConfigFileType::Yaml => {
                    Figment::new()
                        .merge(Yaml::file(&config_file_path))
                        .merge(Env::prefixed("HERMITBOARD").split("_"))
                        .extract()
                        .unwrap_or_else(|err| {
                            // Failed to read config, panic.
                            error!("{}", err);
                            panic!("{}", err)
                        })
                }
                ConfigFileType::Json => {
                    Figment::new()
                        .merge(Json::file(&config_file_path))
                        .merge(Env::prefixed("HERMITBOARD").split("_"))
                        .extract()
                        .unwrap_or_else(|err| {
                            // Failed to read config, panic.
                            error!("{}", err);
                            panic!("{}", err)
                        })
                }
            };

            self.config = loaded_config;
        } else {
            warn!("No config file path specified at the environment key 'HERMITBOARD_APP_CONFIG'. Only environment variables will be loaded.");
        }

        self
    }

    // Read only getter of the loaded configuration.
    fn config(&self) -> &Config {
        &self.config
    }
}
