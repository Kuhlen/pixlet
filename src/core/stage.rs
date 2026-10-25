/// Conversion stage. Progress and status text derive from this, never stored separately.
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
}
