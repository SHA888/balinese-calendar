// tests/integration.rs

use balinese_calendar::{BalineseDate, Rahinan, Sasih, Wuku};

#[test]
fn test_march_6_2026() {
    let d = BalineseDate::from_ymd(2026, 3, 6).unwrap();
    assert_eq!(d.saka_year, 1947);
    assert_eq!(d.sasih, Sasih::Kasanga);
    assert_eq!(d.wuku, Wuku::Ugu);
    assert_eq!(d.wuku.ecology_tag(), "harvest_2");
}

#[test]
fn test_nyepi_2026() {
    // Nyepi 2026 = March 19 = Penanggal 1 Kadasa Saka 1948
    let d = BalineseDate::from_ymd(2026, 3, 19).unwrap();
    assert_eq!(d.saka_year, 1948);
    assert_eq!(d.sasih, Sasih::Kadasa);
}

#[test]
fn test_nyepi_2025() {
    // Nyepi 2025 = March 29 = Penanggal 1 Kadasa Saka 1947
    let d = BalineseDate::from_ymd(2025, 3, 29).unwrap();
    assert_eq!(d.saka_year, 1947);
    assert_eq!(d.sasih, Sasih::Kadasa);

    // Day before Nyepi = Tilem Kasanga
    let eve = BalineseDate::from_ymd(2025, 3, 28).unwrap();
    assert_eq!(eve.saka_year, 1946);
    assert_eq!(eve.sasih, Sasih::Kasanga);
    assert!(eve.is_tilem);
}

#[test]
fn test_flat_record_fields() {
    let d = BalineseDate::from_ymd(2026, 3, 6).unwrap();
    let r = d.to_flat_record();
    assert_eq!(r.saka_year, 1947);
    assert_eq!(r.sasih_name, "Kasanga");
    assert!(!r.sasih_season_tag.is_empty());
    assert_eq!(r.wuku_ecology_tag, "harvest_2");
}

#[test]
fn test_balinese_string_contains_key_parts() {
    let d = BalineseDate::from_ymd(2026, 3, 6).unwrap();
    let s = d.to_balinese_string();
    assert!(s.contains("Ugu"), "wuku missing: {s}");
    assert!(s.contains("Kasanga"), "sasih missing: {s}");
    assert!(s.contains("1947"), "saka year missing: {s}");
}

#[test]
fn test_out_of_range() {
    assert!(BalineseDate::from_ymd(1700, 1, 1).is_err());
    assert!(BalineseDate::from_ymd(2300, 1, 1).is_err());
}

#[test]
fn test_epoch_1969() {
    let d = BalineseDate::from_ymd(1969, 1, 1).unwrap();
    assert_eq!(d.saka_year, 1890);
    assert_eq!(d.sasih, Sasih::Kapitu);
}

#[test]
fn test_wuku_cycle_210_days() {
    // After 210 days, Wuku resets to same position
    use balinese_calendar::utils::gregorian_to_jdn;
    let jdn0 = gregorian_to_jdn(2026, 1, 1).unwrap();
    let d0 = BalineseDate::from_jdn(jdn0);
    let d210 = BalineseDate::from_jdn(jdn0 + 210);
    assert_eq!(d0.wuku, d210.wuku);
    assert_eq!(d0.pancawara, d210.pancawara);
    assert_eq!(d0.saptawara, d210.saptawara);
}

// ── Multi-year sasih reference table (kalenderbali.org) ─────────────────────

#[test]
fn test_sasih_reference_table() {
    // Ground-truth dates sourced from kalenderbali.org.
    let cases: &[(i32, u32, u32, i32, Sasih, &str)] = &[
        // 2026
        (2026, 3, 3, 1947, Sasih::Kasanga, "Purnama Kasanga"),
        (
            2026,
            3,
            18,
            1947,
            Sasih::Kasanga,
            "Tilem Kasanga (eve of Nyepi)",
        ),
        (
            2026,
            3,
            19,
            1948,
            Sasih::Kadasa,
            "Nyepi — Penanggal 1 Kadasa",
        ),
        // 2025
        (
            2025,
            3,
            29,
            1947,
            Sasih::Kadasa,
            "Nyepi 2025 — Penanggal 1 Kadasa",
        ),
        (2025, 3, 28, 1946, Sasih::Kasanga, "Tilem Kasanga 2025"),
    ];

    for &(y, m, d, expected_saka, expected_sasih, desc) in cases {
        let bd = BalineseDate::from_ymd(y, m, d).unwrap();
        assert_eq!(
            bd.saka_year, expected_saka,
            "{desc}: saka year for {y}-{m:02}-{d:02}"
        );
        assert_eq!(
            bd.sasih, expected_sasih,
            "{desc}: sasih for {y}-{m:02}-{d:02}"
        );
    }
}

// ── Pawukon-based rahinan tests ─────────────────────────────────────────────

#[test]
fn test_rahinan_galungan() {
    // Galungan = Buda Kliwon Dungulan
    // 2025-04-23 is a Galungan date (verified with new Pawukon epoch)
    let d = BalineseDate::from_ymd(2025, 4, 23).unwrap();
    assert_eq!(d.wuku, Wuku::Dungulan);
    assert!(
        d.rahinan.contains(&Rahinan::Galungan),
        "2025-04-23 should be Galungan: {:?}",
        d.rahinan
    );
}

#[test]
fn test_rahinan_kuningan() {
    // Kuningan = Saniscara Kliwon Kuningan (10 days after Galungan)
    // 2025-05-03 = 10 days after 2025-04-23
    let d = BalineseDate::from_ymd(2025, 5, 3).unwrap();
    assert_eq!(d.wuku, Wuku::Kuningan);
    assert!(
        d.rahinan.contains(&Rahinan::Kuningan),
        "2025-05-03 should be Kuningan: {:?}",
        d.rahinan
    );
}

#[test]
fn test_rahinan_saraswati() {
    // Saraswati = Saniscara Umanis Watugunung
    // 2025-02-08
    let d = BalineseDate::from_ymd(2025, 2, 8).unwrap();
    assert_eq!(d.wuku, Wuku::Watugunung);
    assert!(
        d.rahinan.contains(&Rahinan::Saraswati),
        "2025-02-08 should be Saraswati: {:?}",
        d.rahinan
    );
}

#[test]
fn test_rahinan_pagerwesi() {
    // Pagerwesi = Buda Kliwon Sinta
    // 2025-02-12
    let d = BalineseDate::from_ymd(2025, 2, 12).unwrap();
    assert_eq!(d.wuku, Wuku::Sinta);
    assert!(
        d.rahinan.contains(&Rahinan::Pagerwesi),
        "2025-02-12 should be Pagerwesi: {:?}",
        d.rahinan
    );
}

#[test]
fn test_rahinan_galungan_210_day_cycle() {
    // Galungan repeats every 210 days
    use balinese_calendar::utils::gregorian_to_jdn;
    let jdn1 = gregorian_to_jdn(2025, 4, 23).unwrap();
    let d1 = BalineseDate::from_jdn(jdn1);
    let d2 = BalineseDate::from_jdn(jdn1 + 210);
    assert!(d1.rahinan.contains(&Rahinan::Galungan));
    assert!(d2.rahinan.contains(&Rahinan::Galungan));
}
