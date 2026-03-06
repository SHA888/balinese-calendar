// tests/integration.rs

use balinese_calendar::{BalineseDate, Sasih, Wuku};

#[test]
fn test_today_march_6_2026() {
    let d = BalineseDate::from_ymd(2026, 3, 6).unwrap();
    assert_eq!(d.saka_year, 1948);
    assert_eq!(d.sasih, Sasih::Kasanga);
    assert!(d.sasih.is_pancaroba());
    assert_eq!(d.wuku, Wuku::Sungsang);
    assert_eq!(d.wuku.ecology_tag(), "wind_watch");
}

#[test]
fn test_nyepi_2026() {
    // Nyepi = Tilem Kasanga → Penanggal 1 Kadasa
    // The eve (tilem) is March 18; Nyepi silence is March 29 in 2026
    // Here we check the Saka structure is consistent
    let d = BalineseDate::from_ymd(2026, 3, 19).unwrap();
    assert_eq!(d.saka_year, 1948);
    assert_eq!(d.sasih, Sasih::Kadasa);
}

#[test]
fn test_flat_record_fields() {
    let d  = BalineseDate::from_ymd(2026, 3, 6).unwrap();
    let r  = d.to_flat_record();
    assert_eq!(r.saka_year, 1948);
    assert_eq!(r.sasih_name, "Kasanga");
    assert_eq!(r.sasih_season_tag, "pancaroba_1");
    assert!(r.pancaroba_flag);
    assert_eq!(r.wuku_ecology_tag, "wind_watch");
}

#[test]
fn test_balinese_string_contains_key_parts() {
    let d = BalineseDate::from_ymd(2026, 3, 6).unwrap();
    let s = d.to_balinese_string();
    assert!(s.contains("Sungsang"), "wuku missing: {s}");
    assert!(s.contains("Kasanga"),  "sasih missing: {s}");
    assert!(s.contains("1948"),     "saka year missing: {s}");
}

#[test]
fn test_out_of_range() {
    assert!(BalineseDate::from_ymd(1700, 1, 1).is_err());
    assert!(BalineseDate::from_ymd(2300, 1, 1).is_err());
}

#[test]
fn test_epoch_1969() {
    // Epoch: Jan 1, 1969 = Penanggal 1 Kasa Saka 1890
    let d = BalineseDate::from_ymd(1969, 1, 1).unwrap();
    assert_eq!(d.saka_year, 1890);
    assert_eq!(d.sasih, Sasih::Kasa);
}

#[test]
fn test_wuku_cycle_210_days() {
    // After 210 days, Wuku resets to same position
    use balinese_calendar::utils::gregorian_to_jdn;
    let jdn0 = gregorian_to_jdn(2026, 1, 1).unwrap();
    let d0   = BalineseDate::from_jdn(jdn0);
    let d210 = BalineseDate::from_jdn(jdn0 + 210);
    assert_eq!(d0.wuku, d210.wuku);
    assert_eq!(d0.pancawara, d210.pancawara);
    assert_eq!(d0.saptawara, d210.saptawara);
}
