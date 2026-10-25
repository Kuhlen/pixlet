/// Business rule errors. `Display` is for logs; pages map it to user text.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum CoreError {
    #[error("Please select a valid image file")]
    UnsupportedFile,
    #[error("Failed to detect format: {0}")]
    DetectFormat(String),
    #[error("Failed to decode image: {0}")]
    Decode(String),
    #[error("{format} encode error: {reason}")]
    Encode {
        format: &'static str,
        reason: String,
    },
}
