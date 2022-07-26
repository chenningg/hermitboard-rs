use super::database_config::DatabaseConfig;
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::error;

#[async_trait]
pub trait DatabaseService {
    async fn new(db_config: &DatabaseConfig) -> Self;
    fn pool(&self) -> &PgPool;
}

// Implementation of the database service.
pub struct DatabaseServiceImpl {
    pool: PgPool,
}

#[async_trait]
impl DatabaseService for DatabaseServiceImpl {
    // Creates a new database service with a database pool of connections.
    async fn new(db_config: &DatabaseConfig) -> Self {
        match PgPoolOptions::new()
            .max_connections(db_config.max_connections)
            .connect(&db_config.url)
            .await
        {
            Ok(db_pool) => DatabaseServiceImpl { pool: db_pool },
            Err(err) => {
                error!("{}", err);
                panic!("{}", err);
            }
        }
    }

    // Getter function for database pool.
    fn pool(&self) -> &PgPool {
        &self.pool
    }
}
