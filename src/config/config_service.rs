use super::config_model::Config;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use tracing::{error, warn};

// Defines a trait for the config service.
pub trait IConfigService {
    fn load_config(&self) -> Config;
}

// Implementation of the config service.
pub struct ConfigService;

impl IConfigService for ConfigService {
    fn load_config(&self) -> Config {
        // Load environment variable to get config file path.
        let config_file_path = std::env::var("HERMITBOARD_APP_CONFIG");

        match config_file_path {
            Ok(file_path) => Figment::new()
                .merge(Toml::file(&file_path))
                .merge(Env::prefixed("HERMITBOARD").split("_"))
                .extract()
                .unwrap_or_else(|err| {
                    error!("{}", err);
                    panic!("{}", err)
                }),
            Err(err) => {
                warn!("No configuration file path specified via environment variable 'HERMITBOARD_APP_CONFIG'. Reading configuration from environment variables only. More info: {}", err.to_string());
                Figment::new()
                    .merge(Env::prefixed("HERMITBOARD").split("_"))
                    .extract()
                    .unwrap_or_else(|err| {
                        error!("{}", err);
                        panic!("{}", err)
                    })
            }
        }
    }
}
