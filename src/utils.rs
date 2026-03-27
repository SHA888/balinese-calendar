// src/utils.rs
//
// Julian Day Number (JDN) utilities and calendar epoch constants.
//
// All Balinese calendar computations are anchored to the Julian Day Number (JDN),
// a continuous integer count of days since noon January 1, 4713 BCE (Julian calendar).
// This is the same foundation used by babadbali.com and the peradnya JS/Java libraries.
//
// References:
//   - Ardhana, I.B.S. (2005). "Pokok-Pokok Wariga". Surabaya: Paramita.
//   - babadbali.com (Yayasan Bali Galang) wewaran & paringkelan algorithms
//   - Pendit, Nyoman S. (2001). "Nyepi: kebangkitan, toleransi, dan kerukunan".
//   - Wikipedia: Balinese Saka Calendar
//   - peradnya/balinese-date-js-lib (Apache-2.0) — algorithm validation reference

use crate::error::BalineseDateError;

// ── Pawukon Epoch ─────────────────────────────────────────────────────────────
//
// The 210-day Pawukon cycle is anchored to a known Redite (Sunday) Umanis Sinta day.
// Epoch: JDN 2440976 = Sunday, day 0 of the Pawukon cycle.
//
// Derived from the peradnya/balinese-date-js-lib (Apache-2.0) pivot:
//   PIVOT_1971: Jan 27, 1971 (JDN 2440979), pawukonDay=3
//   → epoch = 2440979 - 3 = 2440976
//
// Verification test vectors (peradnya cross-validated):
//   PIVOT_2000: Jan 18, 2000 (JDN 2451562), pawukonDay=86
//   → (2451562 - 2440976) % 210 = 86 ✓
pub const PAWUKON_EPOCH_JDN: i64 = 2_440_976;

// ── Ngunaratri Period ────────────────────────────────────────────────────────
// Every 63 solar days, one ngunaratri occurs (a lunar day is skipped).
// The sasih walk-forward algorithm in `sasih.rs` uses pivot points from
// peradnya/balinese-date-js-lib for all sasih/saka calculations.
pub const NGUNARATRI_PERIOD: i64 = 63;

// ── Saka Year Offset ──────────────────────────────────────────────────────────
// Saka year = Gregorian year − 78  (corrected by Nyepi date within the year)
pub const SAKA_YEAR_OFFSET: i32 = 78;

// ── JDN <-> Gregorian Conversion ──────────────────────────────────────────────
// Algorithm: Fliegel & Van Flandern (1968), proleptic Gregorian calendar.

/// Convert a Gregorian calendar date to its Julian Day Number.
/// Returns Err if the date is invalid or out of supported range.
pub fn gregorian_to_jdn(year: i32, month: u32, day: u32) -> Result<i64, BalineseDateError> {
    if !(1800..=2200).contains(&year) {
        return Err(BalineseDateError::OutOfRange);
    }

    // Use chrono::NaiveDate for proper date validation (rejects Feb 30, etc.)
    use chrono::NaiveDate;
    let _naive_date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(BalineseDateError::InvalidDate { year, month, day })?;

    // Wikipedia "Julian day number" proleptic Gregorian formula.
    // All divisions operate on non-negative integers → no truncation vs floor issue.
    let y = year as i64;
    let m = month as i64;
    let d = day as i64;

    let a = (14 - m) / 12;
    let yy = y + 4_800 - a;
    let mm = m + 12 * a - 3;

    let jdn = d + (153 * mm + 2) / 5 + 365 * yy + yy / 4 - yy / 100 + yy / 400 - 32_045;
    Ok(jdn)
}

/// Convert a Julian Day Number to a Gregorian calendar date (year, month, day).
pub fn jdn_to_gregorian(jdn: i64) -> (i32, u32, u32) {
    let l = jdn + 68_569;
    let n = (4 * l) / 146_097;
    let l = l - (146_097 * n + 3) / 4;
    let i = (4_000 * (l + 1)) / 1_461_001;
    let l = l - (1_461 * i) / 4 + 31;
    let j = (80 * l) / 2_447;
    let day = l - (2_447 * j) / 80;
    let l = j / 11;
    let month = j + 2 - 12 * l;
    let year = 100 * (n - 49) + i + l;

    (year as i32, month as u32, day as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jdn_epoch_j2000() {
        // J2000.0 epoch: January 1, 2000 CE = JDN 2451545
        assert_eq!(gregorian_to_jdn(2000, 1, 1).unwrap(), 2_451_545);
    }

    #[test]
    fn test_jdn_roundtrip() {
        let cases = [(2000, 1, 1), (2026, 3, 6), (1969, 1, 1), (2026, 3, 19)];
        for (y, m, d) in cases {
            let jdn = gregorian_to_jdn(y, m, d).unwrap();
            let (yr, mo, da) = jdn_to_gregorian(jdn);
            assert_eq!((yr, mo, da), (y, m, d), "roundtrip failed for {y}-{m:02}-{d:02}");
        }
    }

    #[test]
    fn test_invalid_date_validation() {
        // Test that impossible dates like Feb 30 are properly rejected
        assert!(gregorian_to_jdn(2026, 2, 30).is_err()); // Feb 30 doesn't exist
        assert!(gregorian_to_jdn(2026, 4, 31).is_err()); // April has only 30 days
        assert!(gregorian_to_jdn(2026, 13, 1).is_err()); // Month 13 doesn't exist
        assert!(gregorian_to_jdn(2026, 0, 1).is_err()); // Month 0 doesn't exist
        assert!(gregorian_to_jdn(2026, 1, 0).is_err()); // Day 0 doesn't exist
        assert!(gregorian_to_jdn(2026, 1, 32).is_err()); // Day 32 doesn't exist

        // Leap year validation
        assert!(gregorian_to_jdn(2024, 2, 29).is_ok()); // 2024 is a leap year
        assert!(gregorian_to_jdn(2026, 2, 29).is_err()); // 2026 is not a leap year
    }

    #[test]
    fn test_nyepi_2026_jdn() {
        // Nyepi 2026 = March 19, 2026
        let jdn = gregorian_to_jdn(2026, 3, 19).unwrap();
        assert_eq!(jdn, 2_461_119);
    }
}
