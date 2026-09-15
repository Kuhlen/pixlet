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

#[cfg(test)]
mod tests {
    use super::*;

    fn src(name: &str) -> SourceImage {
        SourceImage {
            name: name.into(),
            bytes: vec![],
        }
    }

    #[test]
    fn accepts_image_mime_or_bmp_tiff_extension() {
        assert!(SourceImage::check_supported("a.png", "image/png").is_ok());
        assert!(SourceImage::check_supported("a.bmp", "").is_ok());
        assert!(SourceImage::check_supported("a.tiff", "").is_ok());
        assert_eq!(
            SourceImage::check_supported("a.txt", "text/plain"),
            Err(CoreError::UnsupportedFile)
        );
    }

    #[test]
    fn output_name_replaces_or_appends_extension() {
        assert_eq!(
            src("photo.jpeg").output_name(OutputFormat::Png),
            "photo.png"
        );
        assert_eq!(
            src("my.photo.jpeg").output_name(OutputFormat::WebP),
            "my.photo.webp"
        );
        assert_eq!(src("photo").output_name(OutputFormat::Gif), "photo.gif");
    }
}
