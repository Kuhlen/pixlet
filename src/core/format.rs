use std::str::FromStr;

/// Supported output formats. `ALL` order is the button order in the UI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    Jpg,
    Png,
    WebP,
    Gif,
    Bmp,
    Tiff,
}

impl OutputFormat {
    pub const ALL: [OutputFormat; 6] = [
        Self::Jpg,
        Self::Png,
        Self::WebP,
        Self::Gif,
        Self::Bmp,
        Self::Tiff,
    ];

    /// Display name for the format picker.
    pub fn label(self) -> &'static str {
        match self {
            Self::Jpg => "JPG",
            Self::Png => "PNG",
            Self::WebP => "WebP",
            Self::Gif => "GIF",
            Self::Bmp => "BMP",
            Self::Tiff => "TIFF",
        }
    }

    /// File extension without the dot. Also the `FromStr` key.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Jpg => "jpg",
            Self::Png => "png",
            Self::WebP => "webp",
            Self::Gif => "gif",
            Self::Bmp => "bmp",
            Self::Tiff => "tiff",
        }
    }

    /// MIME type for the download blob.
    pub fn mime_type(self) -> &'static str {
        match self {
            Self::Jpg => "image/jpeg",
            Self::Png => "image/png",
            Self::WebP => "image/webp",
            Self::Gif => "image/gif",
            Self::Bmp => "image/bmp",
            Self::Tiff => "image/tiff",
        }
    }

    /// Only JPEG takes a quality parameter. WebP is encoded lossless.
    pub fn supports_quality(self) -> bool {
        matches!(self, Self::Jpg)
    }
}

impl FromStr for OutputFormat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL.into_iter().find(|f| f.extension() == s).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_roundtrips_extension() {
        for f in OutputFormat::ALL {
            assert_eq!(f.extension().parse::<OutputFormat>(), Ok(f));
        }
        assert_eq!("exe".parse::<OutputFormat>(), Err(()));
    }

    #[test]
    fn only_jpg_has_quality() {
        let with_quality: Vec<_> = OutputFormat::ALL
            .into_iter()
            .filter(|f| f.supports_quality())
            .collect();
        assert_eq!(with_quality, vec![OutputFormat::Jpg]);
    }
}
