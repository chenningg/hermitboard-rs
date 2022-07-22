use super::database_error::DatabaseError;
use anyhow::Context;
use async_trait::async_trait;
use sqlx::{pool::Pool, postgres::PgPoolOptions, Postgres};

use super::database_config::DatabaseConfig;

// Defines a trait for the database service.
#[async_trait]
pub trait IDatabaseService {
    async fn init_db(&self) -> Pool<Postgres>;
}

// Implementation of the database service.
pub struct DatabaseService {
    pub config: DatabaseConfig,
}

#[async_trait]
impl IDatabaseService for DatabaseService {
    async fn init_db(&self) -> Pool<Postgres> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&*self.config.url)
            .await
            .map_err(|err| DatabaseError::InitializationFailed(err))
            .with_context(|| "failed to initialize connection to database pool");
        pool.unwrap()
    }
}
