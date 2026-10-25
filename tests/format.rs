use pixlet::core::format::OutputFormat;

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
