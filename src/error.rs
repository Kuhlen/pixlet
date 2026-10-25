use crate::core::CoreError;

/// Single error type for the whole app. `Display` is for logs; pages map it to user text.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Core(#[from] CoreError),
    #[error("Failed to read file: {0}")]
    FileRead(String),
    #[error("Failed to download file: {0}")]
    Download(String),
}
