/// Business rule errors. Messages are shown to the user as-is (product language is English).
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
