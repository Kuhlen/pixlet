use image::{DynamicImage, Rgba, RgbaImage};
use pixlet::core::CoreError;
use pixlet::core::convert::{decode, encode};
use pixlet::core::format::OutputFormat;

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
