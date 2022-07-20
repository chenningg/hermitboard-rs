#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error(
        "invalid application environment (expected {expected:?} for key {key:?}, found {actual:?})"
    )]
    InvalidAppEnv {
        expected: String,
        key: String,
        actual: String,
    },
}
