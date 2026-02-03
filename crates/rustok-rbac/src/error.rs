use thiserror::Error;

#[derive(Debug, Error)]
pub enum RbacError {
    #[error("permission denied")]
    PermissionDenied,
}
