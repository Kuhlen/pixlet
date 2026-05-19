use image::DynamicImage;
use image::codecs::bmp::BmpEncoder;
use image::codecs::gif::GifEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::codecs::tiff::TiffEncoder;
use std::io::Cursor;

use super::formats::OutputFormat;

pub fn encode_image(
    img: &DynamicImage,
    format: OutputFormat,
    quality: u8,
) -> Result<Vec<u8>, String> {
    let mut buf: Vec<u8> = Vec::new();
    match format {
        OutputFormat::Jpg => {
            let encoder = JpegEncoder::new_with_quality(&mut buf, quality);
            img.write_with_encoder(encoder)
                .map_err(|e| format!("JPEG encode error: {e}"))?;
        }
        OutputFormat::Png => {
            let encoder = PngEncoder::new(&mut buf);
            img.write_with_encoder(encoder)
                .map_err(|e| format!("PNG encode error: {e}"))?;
        }
        OutputFormat::WebP => {
            let encoder = image::codecs::webp::WebPEncoder::new_lossless(&mut buf);
            img.write_with_encoder(encoder)
                .map_err(|e| format!("WebP encode error: {e}"))?;
        }
        OutputFormat::Gif => {
            let encoder = GifEncoder::new(&mut buf);
            img.write_with_encoder(encoder)
                .map_err(|e| format!("GIF encode error: {e}"))?;
        }
        OutputFormat::Bmp => {
            let encoder = BmpEncoder::new(&mut buf);
            img.write_with_encoder(encoder)
                .map_err(|e| format!("BMP encode error: {e}"))?;
        }
        OutputFormat::Tiff => {
            let mut cursor = Cursor::new(&mut buf);
            let encoder = TiffEncoder::new(&mut cursor);
            img.write_with_encoder(encoder)
                .map_err(|e| format!("TIFF encode error: {e}"))?;
        }
    }
    Ok(buf)
}
