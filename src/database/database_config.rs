use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
pub struct DatabaseConfig {
    url: String, // URI pointing to the backing database.
}
