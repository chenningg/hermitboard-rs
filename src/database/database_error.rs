#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("database initialization failed")]
    InitializationFailed(#[source] sqlx::Error),
}
