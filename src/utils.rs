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
// The 210-day Pawukon cycle is anchored to a known Redite (Sunday) Kliwon Sinta day.
// Epoch: JDN 2232407 = Sunday, Kliwon, Wuku Sinta (day 0 of the Pawukon cycle).
// Source: "Pokok-Pokok Wariga" (Ardhana, 2005), cross-validated against babadbali.com.
//
// Verification test vector (validate against kalenderbali.org after cloning):
//   JDN 2451545 (Jan 1, 2000 CE) → Wuku Sungsang (9), Pancawara Paing (1),
//                                   Saptawara Saniscara (6)
pub const PAWUKON_EPOCH_JDN: i64 = 2_232_407;

// ── Sasih / Saka Epoch ────────────────────────────────────────────────────────
//
// The Sasih (lunar month) calculation uses the lunation count anchored to:
// JDN 2415021 = January 1, 1900 CE = 1 Kasa Saka 1821 (approximately).
//
// The actual epoch used in the peradnya implementation is JDN 2440588
// (January 1, 1969) = Penanggal 1 Sasih Kasa Saka 1890.
// We use the 1969 epoch to match the JS/Java library behaviour for >= 2003 dates.
//
// IMPORTANT: Intercalary month (Nampih Sasih) placement changed over time:
//   - Before 1993:   Malamasa rules apply
//   - 1993–2002:     Sasih Kesinambungan rules apply
//   - 2003–present:  Nampih Sasih (standard intercalary, PHDI-regulated)
// For production use, verify intercalary months annually against PHDI calendar.
pub const SASIH_EPOCH_JDN: i64    = 2_440_588; // Jan 1, 1969 CE
pub const NGUNARATRI_PERIOD: i64  = 63;         // every 63 solar days, one ngunaratri
pub const LUNATION_DAYS: f64      = 29.530_588_853; // mean synodic month

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
    if month < 1 || month > 12 || day < 1 || day > 31 {
        return Err(BalineseDateError::InvalidDate { year, month, day });
    }

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
    fn test_nyepi_2026_jdn() {
        // Nyepi 2026 = March 19, 2026
        let jdn = gregorian_to_jdn(2026, 3, 19).unwrap();
        assert_eq!(jdn, 2_461_119);
    }
}
