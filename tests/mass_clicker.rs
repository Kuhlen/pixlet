use pixlet::core::mass_clicker::{MassTier, tier, total_mass};

#[test]
fn mass_scales_with_clicks() {
    assert_eq!(total_mass(0), "0");
    assert_eq!(total_mass(1), "9.109e-31");
    assert_eq!(total_mass(1000), "9.109e-28");
}

#[test]
fn tier_thresholds() {
    assert_eq!(tier(0), MassTier::Start);
    assert_eq!(tier(9), MassTier::KeepGoing);
    assert_eq!(tier(10), MassTier::Somewhere);
    assert_eq!(tier(999), MassTier::Concerning);
    assert_eq!(tier(1000), MassTier::Legend);
}
