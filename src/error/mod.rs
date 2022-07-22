use async_graphql::ErrorExtensions;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("bad request")]
    BadRequest,
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("too many requests")]
    TooManyRequests,
    #[error(transparent)]
    InternalServerError(#[from] anyhow::Error),
}

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|err, e| match self {
            Error::BadRequest => e.set("code", 400),
            Error::Forbidden => e.set("code", 403),
            Error::NotFound => e.set("code", 404),
            Error::TooManyRequests => e.set("code", 429),
            Error::InternalServerError(err) => e.set("code", 500),
        })
    }
}
