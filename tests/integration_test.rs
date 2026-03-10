// tests/integration_test.rs
//
// Integration tests — cross-validated against:
//   - kalenderbali.org (I Wayan Nuarsa, Udayana University)
//   - lokerbali.id 2026 calendar
//   - peradnya/balinese-date-js-lib known test vectors
//
// To re-run: `cargo test`
// To run with output: `cargo test -- --nocapture`

use balinese_calendar::{BalineseDate, Pancawara, Rahinan, Saptawara, Sasih, Wuku};
use chrono::{Datelike, NaiveDate};

// ── Helper ────────────────────────────────────────────────────────────────────

fn date(y: i32, m: u32, d: u32) -> BalineseDate {
    BalineseDate::from_ymd(y, m, d).expect("valid date")
}

// ── JDN / Gregorian roundtrip ─────────────────────────────────────────────────

#[test]
fn test_j2000_epoch() {
    use balinese_calendar::utils::gregorian_to_jdn;
    assert_eq!(gregorian_to_jdn(2000, 1, 1).unwrap(), 2_451_545);
}

// ── Nyepi 2026 (March 19) ────────────────────────────────────────────────────
// Nyepi = Tilem Kasanga = new year Saka 1948 → first day of Kadasa

#[test]
fn test_nyepi_2026_saka_year() {
    let d = date(2026, 3, 19);
    assert_eq!(d.saka_year, 1948);
}

// ── Ngunaratri edge cases ─────────────────────────────────────────────────────

fn find_ngunaratri(start: NaiveDate, days: i32) -> Vec<BalineseDate> {
    (0..days)
        .filter_map(|offset| {
            let d = start + chrono::Duration::days(offset as i64);
            BalineseDate::from_ymd(d.year(), d.month(), d.day())
                .ok()
                .filter(|b| b.sasih_day.is_ngunaratri())
        })
        .collect()
}

#[test]
fn test_ngunaratri_spacing_and_tithis() {
    // Scan ~2 years starting from pivot era to capture multiple ngunaratri events.
    let start = NaiveDate::from_ymd_opt(1971, 1, 1).unwrap();
    let ngunaratri_days = find_ngunaratri(start, 500);

    // Expect multiple occurrences and constant 63-day spacing between them.
    assert!(
        ngunaratri_days.len() >= 5,
        "expected at least 5 ngunaratri occurrences"
    );

    for window in ngunaratri_days.windows(2) {
        let diff = window[1].jdn - window[0].jdn;
        assert_eq!(diff, 63, "ngunaratri spacing should be 63 days");
    }

    // Primary/secondary tithi must progress by +1 in sequence (Tilem wraps to Penanggal 1).
    for d in ngunaratri_days {
        if let balinese_calendar::SasihDayInfo::Ngunaratri { primary, secondary } = d.sasih_day {
            let expected_next = if primary.tithi_number() == 30 {
                1
            } else {
                primary.tithi_number() + 1
            };
            assert_eq!(
                secondary.tithi_number(),
                expected_next,
                "secondary tithi should be primary+1 ({} -> {}) on {}",
                primary,
                secondary,
                d.gregorian_day
            );
        } else {
            unreachable!("filtered to ngunaratri only");
        }
    }
}

#[test]
fn test_nyepi_2026_sasih_kadasa() {
    // Nyepi falls on Penanggal 1 Kadasa (first day of new Saka year)
    let d = date(2026, 3, 19);
    assert_eq!(d.sasih, Sasih::Kadasa);
    // ⚠ Exact tithi requires epoch calibration against kalenderbali.org
}

#[test]
fn test_nyepi_2026_saptawara() {
    // March 19, 2026 = Kamis (Thursday) = Wraspati
    let d = date(2026, 3, 19);
    assert_eq!(d.saptawara, Saptawara::Wraspati);
}

// ── March 6, 2026 — pancaroba context day ────────────────────────────────────

#[test]
fn test_today_pancaroba() {
    let d = date(2026, 3, 6);
    assert_eq!(d.sasih, Sasih::Kasanga);
    assert!(d.sasih.is_pancaroba(), "Kasanga should be pancaroba");
}

#[test]
fn test_today_wuku() {
    // 2026-03-06 = Wuku Ugu (harvest_2) with corrected Pawukon epoch
    let d = date(2026, 3, 6);
    assert_eq!(d.wuku, Wuku::Ugu);
    assert_eq!(d.wuku.ecology_tag(), "harvest_2");
}

#[test]
fn test_today_saptawara() {
    // March 6, 2026 = Jumat (Friday) = Sukra
    let d = date(2026, 3, 6);
    assert_eq!(d.saptawara, Saptawara::Sukra);
}

// ── Known Galungan (Buda Kliwon Dungulan) ─────────────────────────────────────
// Galungan 2025: April 23, 2025 (peradnya-aligned Pawukon epoch)

#[test]
fn test_galungan_2025() {
    let d = date(2025, 4, 23);
    assert_eq!(d.wuku, Wuku::Dungulan);
    assert_eq!(d.saptawara, Saptawara::Buda);
    assert_eq!(d.pancawara, Pancawara::Kliwon);
    assert!(
        d.rahinan.contains(&Rahinan::Galungan),
        "April 23, 2025 should be Galungan"
    );
}

#[test]
fn test_kuningan_2025() {
    // Kuningan = 10 days after Galungan (April 23 + 10 = May 3)
    let d = date(2025, 5, 3);
    assert_eq!(d.wuku, Wuku::Kuningan);
    assert_eq!(d.saptawara, Saptawara::Saniscara);
    assert_eq!(d.pancawara, Pancawara::Kliwon);
    assert!(d.rahinan.contains(&Rahinan::Kuningan));
}

// ── Saraswati (Saniscara Umanis Watugunung) ───────────────────────────────
// Saraswati = Saniscara Umanis Watugunung
// 2025-09-06 (peradnya-aligned Pawukon epoch)

#[test]
fn test_saraswati_detection() {
    let d = date(2025, 9, 6);
    assert_eq!(d.wuku, Wuku::Watugunung);
    assert_eq!(d.saptawara, Saptawara::Saniscara);
    assert_eq!(d.pancawara, Pancawara::Umanis);
    assert!(
        d.rahinan.contains(&Rahinan::Saraswati),
        "2025-09-06 should be Saraswati"
    );
}

// ── Pawukon cycle integrity ───────────────────────────────────────────────────

#[test]
fn test_pawukon_210_day_cycle() {
    // Any date + 210 days should have the same wuku and all wewaran
    let d1 = date(2026, 1, 1);
    let d2 = BalineseDate::from_jdn(d1.jdn + 210);
    assert_eq!(d1.wuku, d2.wuku);
    assert_eq!(d1.pancawara, d2.pancawara);
    assert_eq!(d1.saptawara, d2.saptawara);
    assert_eq!(d1.triwara, d2.triwara);
    assert_eq!(d1.pawukon_day, d2.pawukon_day);
}

#[test]
fn test_astawara_sangawara_reference_values() {
    let samples = [
        (2026, 1, 1, "Guru", "Dadi"),
        (2026, 3, 6, "Guru", "Dangu"),
        (2026, 3, 7, "Yama", "Jangur"),
        (2026, 3, 19, "Uma", "Ogan"),
        (2026, 5, 10, "Yama", "Dadi"),
    ];

    for (y, m, d, expected_asta, expected_sanga) in samples {
        let bd = date(y, m, d);
        assert_eq!(
            bd.astawara.name(),
            expected_asta,
            "{y}-{m:02}-{d:02} astawara"
        );
        assert_eq!(
            bd.sangawara.name(),
            expected_sanga,
            "{y}-{m:02}-{d:02} sangawara"
        );
    }
}

#[test]
fn test_saptawara_7_day_cycle() {
    let d1 = date(2026, 3, 6); // Sukra (Friday)
    let d7 = BalineseDate::from_jdn(d1.jdn + 7);
    assert_eq!(
        d1.saptawara, d7.saptawara,
        "Saptawara must repeat every 7 days"
    );
}

#[test]
fn test_pancawara_5_day_cycle() {
    let d1 = date(2026, 3, 6);
    let d5 = BalineseDate::from_jdn(d1.jdn + 5);
    assert_eq!(
        d1.pancawara, d5.pancawara,
        "Pancawara must repeat every 5 days"
    );
}

// ── FlatRecord output ─────────────────────────────────────────────────────────

#[test]
fn test_flat_record_pancaroba_flag() {
    let rec = date(2026, 3, 6).to_flat_record();
    assert!(
        rec.pancaroba_flag,
        "FlatRecord should have pancaroba_flag=true"
    );
    assert_eq!(rec.sasih_season_tag, "pancaroba_1");
    assert_eq!(rec.wuku_ecology_tag, "harvest_2");
}

#[test]
fn test_flat_record_fields_populated() {
    let rec = date(2026, 3, 19).to_flat_record();
    assert_eq!(rec.gregorian_year, 2026);
    assert_eq!(rec.gregorian_month, 3);
    assert_eq!(rec.gregorian_day, 19);
    assert_eq!(rec.saka_year, 1948);
    assert!(!rec.sasih_name.is_empty());
    assert!(!rec.wuku_name.is_empty());
}

// ── Display string ────────────────────────────────────────────────────────────

#[test]
fn test_balinese_string_format() {
    let s = date(2026, 3, 6).to_balinese_string();
    assert!(s.contains("Kasanga"), "String should mention current sasih");
    assert!(s.contains("1947"), "String should mention Saka year 1947");
}

// ── Urip values ───────────────────────────────────────────────────────────────

#[test]
fn test_pancawara_urip_values() {
    use balinese_calendar::Pancawara::*;
    assert_eq!(Umanis.urip(), 5);
    assert_eq!(Paing.urip(), 9);
    assert_eq!(Pon.urip(), 7);
    assert_eq!(Wage.urip(), 4);
    assert_eq!(Kliwon.urip(), 8);
}

#[test]
fn test_saptawara_urip_values() {
    use balinese_calendar::Saptawara::*;
    assert_eq!(Redite.urip(), 5);
    assert_eq!(Soma.urip(), 4);
    assert_eq!(Anggara.urip(), 3);
    assert_eq!(Buda.urip(), 7);
    assert_eq!(Wraspati.urip(), 8);
    assert_eq!(Sukra.urip(), 6);
    assert_eq!(Saniscara.urip(), 9);
}

// ── Error handling ────────────────────────────────────────────────────────────

#[test]
fn test_out_of_range_returns_error() {
    use balinese_calendar::BalineseDateError;
    let result = BalineseDate::from_ymd(1700, 1, 1);
    assert!(matches!(result, Err(BalineseDateError::OutOfRange)));
}

#[test]
fn test_invalid_month_returns_error() {
    use balinese_calendar::BalineseDateError;
    let result = BalineseDate::from_ymd(2026, 13, 1);
    assert!(matches!(
        result,
        Err(BalineseDateError::InvalidDate {
            year: 2026,
            month: 13,
            day: 1
        })
    ));
}
