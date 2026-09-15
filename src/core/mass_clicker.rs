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

/// Encouragement line by click count.
pub fn message(clicks: u64) -> &'static str {
    match clicks {
        0 => "Click the button to start massing!",
        1..=9 => "Keep going...",
        10..=49 => "You're getting somewhere!",
        50..=99 => "That's a lot of electrons!",
        100..=499 => "Are you okay?",
        500..=999 => "This is concerning.",
        _ => "You absolute legend.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mass_scales_with_clicks() {
        assert_eq!(total_mass(0), "0");
        assert_eq!(total_mass(1), "9.109e-31");
        assert_eq!(total_mass(1000), "9.109e-28");
    }

    #[test]
    fn message_thresholds() {
        assert_eq!(message(0), "Click the button to start massing!");
        assert_eq!(message(10), "You're getting somewhere!");
        assert_eq!(message(1000), "You absolute legend.");
    }
}
