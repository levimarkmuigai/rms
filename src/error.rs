use std::io;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("io: {0}")]
    Io(#[from] io::Error),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("database: {0}")]
    Database(#[from] postgres::Error),

    #[error("pool: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("password: {0}")]
    Password(#[from] crate::entities::user::PasswordError),

    #[error("internal: {0}")]
    Internal(String),
}
