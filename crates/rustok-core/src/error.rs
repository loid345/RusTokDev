use thiserror::Error;

pub type Result<T> = std::result::Result<T, RusToKError>;

#[derive(Debug, Error)]
pub enum RusToKError {
    #[error("Invalid ID format: {0}")]
    InvalidIdFormat(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
