use image::DynamicImage;
use image::ImageReader;
use std::io::Cursor;

pub fn decode_image(bytes: &[u8]) -> Result<DynamicImage, String> {
    let reader = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|e| format!("Failed to detect format: {e}"))?;
    reader
        .decode()
        .map_err(|e| format!("Failed to decode image: {e}"))
}
