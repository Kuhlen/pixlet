use super::CoreError;
use super::format::OutputFormat;

/// Image picked by the user, already read into memory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceImage {
    pub name: String,
    pub bytes: Vec<u8>,
}

impl SourceImage {
    /// Browsers often report an empty MIME for bmp/tiff, so fall back to the extension.
    pub fn check_supported(name: &str, mime: &str) -> Result<(), CoreError> {
        let by_mime = mime.starts_with("image/");
        let by_ext = name.ends_with(".bmp") || name.ends_with(".tiff");
        if by_mime || by_ext {
            Ok(())
        } else {
            Err(CoreError::UnsupportedFile)
        }
    }

    /// Output file name: swap the extension, append one if missing.
    pub fn output_name(&self, format: OutputFormat) -> String {
        let stem = self
            .name
            .rsplit_once('.')
            .map_or(self.name.as_str(), |(stem, _)| stem);
        format!("{stem}.{}", format.extension())
    }
}
