use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("Repository error: {0}")]
    Repository(String),

    #[error("Node not found: {0}")]
    NodeNotFound(Uuid),

    #[error("Validation error: {0}")]
    Validation(String),
}

pub type ContentResult<T> = Result<T, ContentError>;
