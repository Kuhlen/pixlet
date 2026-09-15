/// Conversion stage. Progress and label derive from this, never stored separately.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConversionStage {
    #[default]
    Idle,
    Decoding,
    Encoding,
    Done,
}

impl ConversionStage {
    /// True for every stage except `Idle`.
    pub fn is_running(self) -> bool {
        self != Self::Idle
    }

    /// Progress bar width in percent.
    pub fn progress(self) -> u8 {
        match self {
            Self::Idle => 0,
            Self::Decoding => 30,
            Self::Encoding => 60,
            Self::Done => 100,
        }
    }

    /// Status text under the progress bar.
    pub fn label(self) -> &'static str {
        match self {
            Self::Idle => "",
            Self::Decoding => "Decoding image...",
            Self::Encoding => "Encoding to target format...",
            Self::Done => "Conversion complete! Downloading...",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
