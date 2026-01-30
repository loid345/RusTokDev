use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Entity not found: {entity_type} with id {id}")]
    NotFound { entity_type: String, id: String },

    #[error("Index error: {0}")]
    Index(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type IndexResult<T> = Result<T, IndexError>;

impl From<IndexError> for rustok_core::Error {
    fn from(error: IndexError) -> Self {
        match error {
            IndexError::Database(error) => Self::Database(error),
            IndexError::NotFound { entity_type, id } => {
                Self::NotFound(format!("{entity_type} with id {id}"))
            }
            IndexError::Index(message) => Self::NotFound(message),
            IndexError::Serialization(error) => Self::Serialization(error),
        }
    }
}
