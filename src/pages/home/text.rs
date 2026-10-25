use crate::core::CoreError;
use crate::core::stage::ConversionStage;
use crate::error::AppError;

/// Status text under the progress bar.
pub fn stage_text(s: ConversionStage) -> &'static str {
    match s {
        ConversionStage::Idle => "",
        ConversionStage::Decoding => "Decoding image...",
        ConversionStage::Encoding => "Encoding to target format...",
        ConversionStage::Done => "Conversion complete! Downloading...",
    }
}

/// User-facing text for an error. Technical detail goes to the console, not here.
pub fn error_text(e: &AppError) -> String {
    match e {
        AppError::Core(CoreError::UnsupportedFile) => "Please select a valid image file.".into(),
        AppError::Core(CoreError::DetectFormat(_) | CoreError::Decode(_)) => {
            "This image could not be read. It may be damaged.".into()
        }
        AppError::Core(CoreError::Encode { format, .. }) => {
            format!("Could not convert the image to {format}.")
        }
        AppError::FileRead(_) => "Could not read the selected file.".into(),
        AppError::Download(_) => "Could not start the download.".into(),
    }
}
