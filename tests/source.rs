use pixlet::core::CoreError;
use pixlet::core::format::OutputFormat;
use pixlet::core::source::SourceImage;

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
