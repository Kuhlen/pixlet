use pixlet::core::stage::ConversionStage;

#[test]
fn progress_is_monotonic() {
    let order = [
        ConversionStage::Idle,
        ConversionStage::Decoding,
        ConversionStage::Encoding,
        ConversionStage::Done,
    ];
    assert!(order.windows(2).all(|w| w[0].progress() < w[1].progress()));
    assert!(!ConversionStage::Idle.is_running());
    assert!(ConversionStage::Done.is_running());
}
