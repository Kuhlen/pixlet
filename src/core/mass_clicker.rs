//! About page easter egg: electron mass accumulated per click.

const ELECTRON_MASS_KG: f64 = 9.109e-31;

/// Total mass in kg, scientific notation. "0" before the first click.
pub fn total_mass(clicks: u64) -> String {
    if clicks == 0 {
        "0".to_string()
    } else {
        format!("{:.3e}", clicks as f64 * ELECTRON_MASS_KG)
    }
}

/// Encouragement level. Page maps each to its text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MassTier {
    Start,
    KeepGoing,
    Somewhere,
    Lots,
    Okay,
    Concerning,
    Legend,
}

pub fn tier(clicks: u64) -> MassTier {
    match clicks {
        0 => MassTier::Start,
        1..=9 => MassTier::KeepGoing,
        10..=49 => MassTier::Somewhere,
        50..=99 => MassTier::Lots,
        100..=499 => MassTier::Okay,
        500..=999 => MassTier::Concerning,
        _ => MassTier::Legend,
    }
}
