use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,          // URL string pointing to the backing database.
    pub max_connections: u32, // Set the maximum number of connections that the database should maintain.
}
