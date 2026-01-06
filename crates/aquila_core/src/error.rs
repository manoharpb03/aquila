use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Manifest serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Path not found: {0}")]
    NotFound(String),

    #[error("Storage backend error: {0}")]
    Generic(String),
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,

    #[error("Insufficient permissions: {0}")]
    Forbidden(String),

    #[error("Authentication provider error: {0}")]
    Generic(String),
}
