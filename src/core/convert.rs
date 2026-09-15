use std::io::Cursor;

use image::codecs::{
    bmp::BmpEncoder, gif::GifEncoder, jpeg::JpegEncoder, png::PngEncoder, tiff::TiffEncoder,
    webp::WebPEncoder,
};
use image::{DynamicImage, ImageError};

use super::CoreError;
use super::format::OutputFormat;

/// Sniff the format from the bytes, then decode.
pub fn decode(bytes: &[u8]) -> Result<DynamicImage, CoreError> {
    image::ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| CoreError::DetectFormat(e.to_string()))?
        .decode()
        .map_err(|e| CoreError::Decode(e.to_string()))
}

/// `quality` only affects JPEG. Other formats ignore it.
pub fn encode(img: &DynamicImage, format: OutputFormat, quality: u8) -> Result<Vec<u8>, CoreError> {
    let mut buf = Vec::new();
    let result = match format {
        OutputFormat::Jpg => {
            img.write_with_encoder(JpegEncoder::new_with_quality(&mut buf, quality))
        }
        OutputFormat::Png => img.write_with_encoder(PngEncoder::new(&mut buf)),
        OutputFormat::WebP => img.write_with_encoder(WebPEncoder::new_lossless(&mut buf)),
        OutputFormat::Gif => img.write_with_encoder(GifEncoder::new(&mut buf)),
        OutputFormat::Bmp => img.write_with_encoder(BmpEncoder::new(&mut buf)),
        // TiffEncoder needs Seek, Vec<u8> alone does not have it
        OutputFormat::Tiff => img.write_with_encoder(TiffEncoder::new(Cursor::new(&mut buf))),
    };
    result.map_err(|e: ImageError| CoreError::Encode {
        format: format.label(),
        reason: e.to_string(),
    })?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    fn sample() -> DynamicImage {
        let mut img = RgbaImage::new(4, 4);
        img.put_pixel(1, 1, Rgba([255, 0, 0, 255]));
        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn every_format_encodes_then_decodes() {
        for format in OutputFormat::ALL {
            let bytes = encode(&sample(), format, 80).unwrap();
            let back = decode(&bytes).unwrap_or_else(|e| panic!("{format:?}: {e}"));
            assert_eq!((back.width(), back.height()), (4, 4), "{format:?}");
        }
    }

    #[test]
    fn garbage_is_rejected() {
        assert!(matches!(
            decode(b"not an image"),
            Err(CoreError::Decode(_) | CoreError::DetectFormat(_))
        ));
    }
}
