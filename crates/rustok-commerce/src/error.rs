use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum CommerceError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Product not found: {0}")]
    ProductNotFound(Uuid),

    #[error("Variant not found: {0}")]
    VariantNotFound(Uuid),

    #[error("Duplicate handle: {handle} already exists for locale {locale}")]
    DuplicateHandle { handle: String, locale: String },

    #[error("Duplicate SKU: {0}")]
    DuplicateSku(String),

    #[error("Invalid price: {0}")]
    InvalidPrice(String),

    #[error("Insufficient inventory: requested {requested}, available {available}")]
    InsufficientInventory { requested: i32, available: i32 },

    #[error("Invalid option combination")]
    InvalidOptionCombination,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Product must have at least one variant")]
    NoVariants,

    #[error("Cannot delete published product")]
    CannotDeletePublished,
}

pub type CommerceResult<T> = Result<T, CommerceError>;
