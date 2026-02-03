use thiserror::Error;

#[derive(Debug, Error)]
pub enum TenantError {
    #[error("tenant not found")]
    NotFound,
}
