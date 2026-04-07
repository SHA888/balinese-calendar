// src/sasih.rs
//
// Sasih — Balinese lunar month system.
// This is the most algorithmically complex part of the Balinese calendar.
//
// Key concepts:
//   Penanggal  — waxing phase (tithi 1–15), starts day after Tilem (new moon)
//   Pangelong  — waning phase (tithi 1–15), starts day after Purnama (full moon)
//   Purnama    — full moon (end of penanggal 15)
//   Tilem      — new moon (end of pangelong 15)
//   Ngunaratri — "minus one night": every 63 solar days, a lunar day is skipped
//                (two lunar days fall on one solar day)
//   Nampih Sasih — intercalary (13th) month inserted to re-sync with solar year
//
// Algorithm basis:
//   - peradnya/balinese-date-js-lib (Apache-2.0) — walk-forward pivot algorithm
//   - Pendit, N.S. (2001). "Nyepi: kebangkitan, toleransi, dan kerukunan."
//   - babadbali.com Sasih algorithm
//
// The algorithm uses two pivot points (known dates with all sasih parameters)
// and walks forward or backward to the target date, counting sasih boundaries
// and determining nampih (intercalary) months algorithmically from the
// 19-year Metonic-like cycle (saka_year % 19).
//
// ⚠ PRODUCTION NOTE:
//   The Nampih Sasih placement computed algorithmically may differ from the
//   official PHDI calendar by 1 sasih in some years. Verify critical dates
//   against the PHDI calendar or kalenderbali.org.

use crate::utils::{NGUNARATRI_PERIOD, SAKA_YEAR_OFFSET};

// ── Sasih names ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Sasih {
    Kasa = 0,
    Karo = 1,
    Katiga = 2,
    Kapat = 3,
    Kalima = 4,
    Kanem = 5,
    Kapitu = 6,
    Kawolu = 7,
    Kasanga = 8,
    Kadasa = 9,
    Desta = 10, // Jyestha
    Sada = 11,  // Sadha
    // Intercalary months (Nampih Sasih):
    NampihDesta = 12, // Mala/Nampih Jyestha — inserted after Desta
    NampihSada = 13,  // Mala/Nampih Sadha   — inserted after Sada
}

impl Sasih {
    pub fn name(&self) -> &'static str {
        match self {
            Sasih::Kasa => "Kasa",
            Sasih::Karo => "Karo",
            Sasih::Katiga => "Katiga",
            Sasih::Kapat => "Kapat",
            Sasih::Kalima => "Kalima",
            Sasih::Kanem => "Kanem",
            Sasih::Kapitu => "Kapitu",
            Sasih::Kawolu => "Kawolu",
            Sasih::Kasanga => "Kasanga",
            Sasih::Kadasa => "Kadasa",
            Sasih::Desta => "Desta",
            Sasih::Sada => "Sada",
            Sasih::NampihDesta => "Nampih Desta",
            Sasih::NampihSada => "Nampih Sada",
        }
    }

    /// Traditional season association
    ///
    /// Based on academic validation from lontar texts, peer-reviewed research, and ritual tradition:
    /// - Purwaputera et al. (2025), IJMRA Vol. 08 Issue 08 (Udayana/BMKG)
    /// - Ginaya (2018), IJLLC 4(3), 24-37
    /// - Lontar Purwaka Bumi, Cuda Mani, Purana Bali Dwipa
    /// - Nangluk Merana / Caru Sasih Kalima–Kanem ritual tradition
    ///
    /// Pancaroba (transitional season, dry→wet) spans Sasih Kalima and Sasih Kanem,
    /// approximately October–December Gregorian. This is the ONLY academically validated
    /// pancaroba in the Balinese Saka Calendar.
    ///
    /// Season mapping:
    /// - Dry (kemarau): Jyesta, Sadha, Kasa, Karo, Katiga, Kapat (≈Apr-Oct)
    /// - Pancaroba (transitional): Kalima, Kanem (≈Oct-Dec)
    /// - Wet (hujan): Kapitu, Kawulu, Kasanga, Kadasa (≈Dec-Mar)
    pub fn season_tag(&self) -> &'static str {
        match self {
            Sasih::Kasa => "dry",         // Peak dry season (June–Aug)
            Sasih::Karo => "dry",         // Peak dry season
            Sasih::Katiga => "dry",       // Dry season
            Sasih::Kapat => "dry",        // Late dry, "blabur" possible
            Sasih::Kalima => "pancaroba", // Early transition, onset of rain
            Sasih::Kanem => "pancaroba",  // Full transition, Nangluk Merana period
            Sasih::Kapitu => "wet",       // Peak wet season (Dec–Feb)
            Sasih::Kawolu => "wet",       // Peak wet season
            Sasih::Kasanga => "wet",      // Late wet season
            Sasih::Kadasa => "wet",       // Waning wet, pre-Nyepi
            Sasih::Desta => "dry",        // Early dry season (Jyesta)
            Sasih::Sada => "dry",         // Dry season
            Sasih::NampihDesta => "dry",  // Intercalary month follows dry season
            Sasih::NampihSada => "dry",   // Intercalary month follows dry season
        }
    }

    /// Returns true if this sasih falls in the traditional pancaroba period.
    ///
    /// The pancaroba (transitional season, dry→wet) spans Sasih Kalima and
    /// Sasih Kanem, approximately October–December Gregorian. This is the
    /// only academically and ritually validated pancaroba in the Balinese
    /// Saka Calendar.
    ///
    /// Sources:
    /// - Lontar Purwaka Bumi, Lontar Cuda Mani, Purana Bali Dwipa
    /// - Nangluk Merana / Caru Sasih Kalima–Kanem ceremony tradition
    /// - Purwaputera et al. (2025), IJMRA Vol. 08 Issue 08
    /// - Ginaya (2018), IJLLC 4(3), 24-37
    /// - I Wayan Tusan (1974), Ala Ayuning Sasih / Wariga tradition
    ///
    /// Note: There is NO traditional second pancaroba for wet→dry transition.
    /// BMKG meteorological classification (April pancaroba) ≠ Balinese tradition.
    pub fn is_pancaroba(&self) -> bool {
        matches!(self, Sasih::Kalima | Sasih::Kanem)
    }

    pub fn is_planting_signal(&self) -> bool {
        matches!(self, Sasih::Katiga | Sasih::Kapat | Sasih::Kalima)
    }

    fn from_index(idx: i32) -> Self {
        match ((idx % 12) + 12) % 12 {
            0 => Sasih::Kasa,
            1 => Sasih::Karo,
            2 => Sasih::Katiga,
            3 => Sasih::Kapat,
            4 => Sasih::Kalima,
            5 => Sasih::Kanem,
            6 => Sasih::Kapitu,
            7 => Sasih::Kawolu,
            8 => Sasih::Kasanga,
            9 => Sasih::Kadasa,
            10 => Sasih::Desta,
            _ => Sasih::Sada,
        }
    }
}

// ── Tithi phase representations ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TithiPhase {
    /// Waxing moon — day 1–14 (Penanggal 1–14)
    Penanggal(u8),
    /// Full moon (Penanggal 15 = Purnama)
    Purnama,
    /// Waning moon — day 1–14 (Pangelong 1–14)
    Pangelong(u8),
    /// New moon (Pangelong 15 = Tilem)
    Tilem,
}

impl TithiPhase {
    pub fn tithi_number(&self) -> u8 {
        match self {
            TithiPhase::Penanggal(n) => *n,
            TithiPhase::Purnama => 15,
            TithiPhase::Pangelong(n) => 15 + n,
            TithiPhase::Tilem => 30,
        }
    }
}

impl std::fmt::Display for TithiPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TithiPhase::Penanggal(n) => write!(f, "Penanggal {n}"),
            TithiPhase::Purnama => write!(f, "Purnama"),
            TithiPhase::Pangelong(n) => write!(f, "Pangelong {n}"),
            TithiPhase::Tilem => write!(f, "Tilem"),
        }
    }
}

// ── SasihDayInfo — phase within the lunar month, including ngunaratri ────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SasihDayInfo {
    /// Single tithi on this solar day
    Single(TithiPhase),
    /// Ngunaratri — two lunar days on one solar day; primary + secondary tithi.
    Ngunaratri { primary: TithiPhase, secondary: TithiPhase },
}

impl std::fmt::Display for SasihDayInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SasihDayInfo::Single(TithiPhase::Penanggal(n)) => write!(f, "Penanggal {n}"),
            SasihDayInfo::Single(TithiPhase::Purnama) => write!(f, "Purnama"),
            SasihDayInfo::Single(TithiPhase::Pangelong(n)) => write!(f, "Pangelong {n}"),
            SasihDayInfo::Single(TithiPhase::Tilem) => write!(f, "Tilem"),
            SasihDayInfo::Ngunaratri { primary, .. } => write!(f, "Ngunaratri ({primary})"),
        }
    }
}

impl SasihDayInfo {
    pub fn is_purnama(&self) -> bool {
        matches!(
            self,
            SasihDayInfo::Single(TithiPhase::Purnama)
                | SasihDayInfo::Ngunaratri { primary: TithiPhase::Purnama, .. }
        )
    }
    pub fn is_tilem(&self) -> bool {
        matches!(
            self,
            SasihDayInfo::Single(TithiPhase::Tilem)
                | SasihDayInfo::Ngunaratri { primary: TithiPhase::Tilem, .. }
        )
    }
    pub fn is_ngunaratri(&self) -> bool {
        matches!(self, SasihDayInfo::Ngunaratri { .. })
    }

    pub fn tithi_number(&self) -> u8 {
        match self {
            SasihDayInfo::Single(phase) => phase.tithi_number(),
            SasihDayInfo::Ngunaratri { primary, .. } => primary.tithi_number(),
        }
    }
}

// ── Full Sasih calculation result ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SasihResult {
    pub saka_year: i32,
    pub sasih: Sasih,
    pub day_info: SasihDayInfo,
    pub is_nampih: bool, // true if this day falls in an intercalary month
}

// ── Pivot points ──────────────────────────────────────────────────────────────
// Known dates with fully characterized sasih parameters.
// Source: peradnya/balinese-date-js-lib (Apache-2.0).

struct Pivot {
    jdn: i64,
    sasih_day: i64,      // day within the 30-day sasih at this pivot (0-based)
    ngunaratri_day: i64, // ngunaratri offset at this pivot
    saka: i32,
    sasih_id: i32, // 0-based sasih index (Kasa=0 .. Sada=11)
    is_nampih: bool,
}

// PIVOT_1971: January 27, 1971 — Penanggal 1 Kapitu Saka 1892
const PIVOT_1971: Pivot = Pivot {
    jdn: 2_440_979, // gregorian_to_jdn(1971, 1, 27)
    sasih_day: 0,
    ngunaratri_day: 0,
    saka: 1892,
    sasih_id: 6, // Kapitu
    is_nampih: false,
};

// PIVOT_2000: January 18, 2000 — Pangelong 12 Kapitu Saka 1921
const PIVOT_2000: Pivot = Pivot {
    jdn: 2_451_562, // gregorian_to_jdn(2000, 1, 18)
    sasih_day: 12,
    ngunaratri_day: 0,
    saka: 1921,
    sasih_id: 6, // Kapitu
    is_nampih: false,
};

// Sasih Kesinambungan (SK) period boundaries (JDN)
// During this period (1993-01-24 to 2003-01-03), different nampih rules apply.
const SK_START_JDN: i64 = 2_448_998; // 1993-01-24
const SK_END_JDN: i64 = 2_452_642; // 2003-01-03

// ── Euclidean modulo helper ──────────────────────────────────────────────────
fn emod(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

// ── Core calculation ──────────────────────────────────────────────────────────

impl SasihResult {
    pub fn from_jdn(jdn: i64) -> Self {
        // Select the best pivot point
        let pivot = if jdn < 2_451_550 {
            // Before ~Jan 6, 2000 (Pangalantaka Paing)
            &PIVOT_1971
        } else {
            &PIVOT_2000
        };

        let day_diff = jdn - pivot.jdn;

        // ── 1. Compute sasih day info (tithi, phase, ngunaratri) ─────────
        let day_skip = if day_diff >= 0 {
            // ceil(day_diff / 63) for positive
            (day_diff + NGUNARATRI_PERIOD - 1) / NGUNARATRI_PERIOD
        } else {
            // ceil for negative = -floor(abs/63)
            -((-day_diff) / NGUNARATRI_PERIOD)
        };
        let day_total = pivot.sasih_day + day_diff + day_skip;

        let raw_day = emod(day_total, 30) as u8; // 0–29
        let is_pangelong = raw_day == 0 || raw_day > 15;
        let is_ngunaratri = emod(day_diff, NGUNARATRI_PERIOD) == 0 && day_diff != 0;

        let tithi_in_phase = emod(day_total, 15) as u8; // 0–14
        let tithi = if tithi_in_phase == 0 { 15 } else { tithi_in_phase };

        let day_info = Self::build_day_info(tithi, is_pangelong, is_ngunaratri);

        // ── 2. Walk-forward to determine sasih and saka year ─────────────
        let pivot_offset = if pivot.sasih_day == 0 && pivot.ngunaratri_day == 0 { 0 } else { 1 };

        let total_sasih = if day_diff >= 0 {
            (day_total + 29) / 30 - pivot_offset // ceil(day_total / 30) - offset
        } else {
            -((-day_total) / 30) - pivot_offset
        };

        let mut current_sasih = pivot.sasih_id;
        let mut current_saka = pivot.saka - if current_sasih == 9 { 1 } else { 0 }; // Kadasa offset
        let mut nampih_count: i32 = if pivot.is_nampih { 1 } else { 0 };
        let mut in_sk = pivot.jdn >= SK_START_JDN && pivot.jdn < SK_END_JDN;

        let mut remaining = total_sasih;

        while remaining != 0 {
            if day_diff >= 0 {
                // Walking forward
                if nampih_count == 0 || nampih_count == 2 {
                    nampih_count = 0;
                    current_sasih = emod(current_sasih as i64 + 1, 12) as i32;
                }
                remaining -= 1;

                if current_sasih == 9 && nampih_count == 0 {
                    // Kadasa = start of new Saka year
                    current_saka += 1;
                }

                // SK period tracking
                if current_sasih == 7 && current_saka == 1914 {
                    in_sk = true;
                } else if current_sasih == 7 && current_saka == 1924 {
                    in_sk = false;
                }
            } else {
                // Walking backward
                if nampih_count == 0 || nampih_count == 2 {
                    nampih_count = 0;
                    current_sasih = emod(current_sasih as i64 - 1, 12) as i32;
                }
                remaining += 1;

                if current_sasih == 8 && nampih_count == 0 {
                    // Kasanga going backward = leaving this Saka year
                    current_saka -= 1;
                }

                // SK period tracking (reverse)
                if current_sasih == 6 && current_saka == 1914 {
                    in_sk = false;
                } else if current_sasih == 6 && current_saka == 1924 {
                    in_sk = true;
                }
            }

            // Nampih detection via 19-year Metonic-like cycle
            nampih_count += Self::nampih_increment(current_saka, current_sasih, in_sk);
        }

        let is_nampih = nampih_count == 1;
        let sasih = if is_nampih {
            match Sasih::from_index(current_sasih) {
                Sasih::Desta => Sasih::NampihDesta,
                Sasih::Sada => Sasih::NampihSada,
                other => other, // fallback — shouldn't happen with correct algorithm
            }
        } else {
            Sasih::from_index(current_sasih)
        };

        SasihResult { saka_year: current_saka, sasih, day_info, is_nampih }
    }

    /// Determine if the current sasih in the given saka year triggers a nampih.
    /// Returns 1 if nampih should be inserted, 0 otherwise.
    /// Based on the 19-year Metonic-like cycle from peradnya.
    fn nampih_increment(saka: i32, sasih_id: i32, in_sk: bool) -> i32 {
        let cycle = ((saka % 19) + 19) % 19;
        match cycle {
            0 | 6 | 11 => {
                match (sasih_id, in_sk, saka != 1925) {
                    (10, false, true) => 1, // Desta
                    _ => 0,
                }
            }
            3 | 8 | 14 | 16 => {
                match (sasih_id, in_sk) {
                    (11, false) => 1, // Sada
                    _ => 0,
                }
            }
            // SK-period specific rules
            2 | 10 => {
                match (sasih_id, in_sk) {
                    (10, true) => 1, // Desta during SK
                    _ => 0,
                }
            }
            4 => {
                match (sasih_id, in_sk) {
                    (2, true) => 1, // Katiga during SK
                    _ => 0,
                }
            }
            7 => {
                match (sasih_id, in_sk) {
                    (0, true) => 1, // Kasa during SK
                    _ => 0,
                }
            }
            13 => {
                match (sasih_id, in_sk) {
                    (9, true) => 1, // Kadasa during SK
                    _ => 0,
                }
            }
            15 => {
                match (sasih_id, in_sk) {
                    (1, true) => 1, // Karo during SK
                    _ => 0,
                }
            }
            _ => 0,
        }
    }

    fn build_day_info(tithi: u8, is_pangelong: bool, is_ngunaratri: bool) -> SasihDayInfo {
        let primary_phase = if is_pangelong {
            if tithi == 15 || (tithi == 14 && is_ngunaratri) {
                TithiPhase::Tilem
            } else {
                TithiPhase::Pangelong(tithi)
            }
        } else if tithi == 15 || (tithi == 14 && is_ngunaratri) {
            TithiPhase::Purnama
        } else {
            TithiPhase::Penanggal(tithi)
        };

        if is_ngunaratri {
            let next_tithi = if tithi == 15 { 1 } else { tithi + 1 };
            let secondary_phase = if is_pangelong {
                if next_tithi > 14 { TithiPhase::Tilem } else { TithiPhase::Pangelong(next_tithi) }
            } else if next_tithi > 14 {
                TithiPhase::Purnama
            } else {
                TithiPhase::Penanggal(next_tithi)
            };
            SasihDayInfo::Ngunaratri { primary: primary_phase, secondary: secondary_phase }
        } else {
            SasihDayInfo::Single(primary_phase)
        }
    }
}

// ── Convenience helpers ───────────────────────────────────────────────────────

/// Compute approximate Saka year from Gregorian year.
/// Exact Saka year flips at Nyepi (mid-March), not January 1.
pub fn approx_saka_year(gregorian_year: i32) -> i32 {
    gregorian_year - SAKA_YEAR_OFFSET
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::gregorian_to_jdn;

    #[test]
    fn test_pivot_1971_sasih() {
        // PIVOT_1971: Jan 27, 1971 = Penanggal 1 Kapitu Saka 1892
        let result = SasihResult::from_jdn(PIVOT_1971.jdn);
        assert_eq!(result.sasih, Sasih::Kapitu);
        assert_eq!(result.saka_year, 1892);
    }

    #[test]
    fn test_pivot_2000_sasih() {
        // PIVOT_2000: Jan 18, 2000 = Kapitu Saka 1921
        let result = SasihResult::from_jdn(PIVOT_2000.jdn);
        assert_eq!(result.sasih, Sasih::Kapitu);
        assert_eq!(result.saka_year, 1921);
    }

    #[test]
    fn test_nyepi_2026_is_kadasa() {
        // Nyepi 2026 = March 19, 2026 = Penanggal 1 Kadasa Saka 1948
        let jdn = gregorian_to_jdn(2026, 3, 19).unwrap();
        let result = SasihResult::from_jdn(jdn);
        assert_eq!(result.saka_year, 1948, "Saka year for Nyepi 2026 must be 1948");
        assert_eq!(result.sasih, Sasih::Kadasa, "Nyepi 2026 must be Kadasa");
    }

    #[test]
    fn test_today_is_kasanga() {
        // March 6, 2026 = before Nyepi (March 19) = Saka 1947, Kasanga
        // Source: kalenderbali.org
        let jdn = gregorian_to_jdn(2026, 3, 6).unwrap();
        let result = SasihResult::from_jdn(jdn);
        assert_eq!(result.saka_year, 1947);
        assert_eq!(result.sasih, Sasih::Kasanga);
    }

    #[test]
    fn cross_validate_kalenderbali_org() {
        // 2026-03-03: Purnama Kasanga per kalenderbali.org
        let d1 = SasihResult::from_jdn(gregorian_to_jdn(2026, 3, 3).unwrap());
        assert_eq!(d1.sasih, Sasih::Kasanga, "2026-03-03 should be Kasanga");
        assert!(d1.day_info.is_purnama(), "2026-03-03 should be Purnama");

        // 2026-03-19: Nyepi Tahun Baru Saka 1948
        let d2 = SasihResult::from_jdn(gregorian_to_jdn(2026, 3, 19).unwrap());
        assert_eq!(d2.saka_year, 1948, "2026-03-19 Nyepi should be Saka 1948");
        assert_eq!(d2.sasih, Sasih::Kadasa, "2026-03-19 should be Kadasa");

        // 2026-03-18: Tilem Kasanga (day before Nyepi)
        let d3 = SasihResult::from_jdn(gregorian_to_jdn(2026, 3, 18).unwrap());
        assert!(d3.day_info.is_tilem(), "2026-03-18 should be Tilem");
        assert_eq!(d3.sasih, Sasih::Kasanga, "2026-03-18 should be Kasanga");

        // 2026-01-01: Saka 1947
        let d4 = SasihResult::from_jdn(gregorian_to_jdn(2026, 1, 1).unwrap());
        assert_eq!(d4.saka_year, 1947, "2026-01-01 should be Saka 1947");

        // 2025-12-15: Saka 1947
        let d5 = SasihResult::from_jdn(gregorian_to_jdn(2025, 12, 15).unwrap());
        assert_eq!(d5.saka_year, 1947, "2025-12-15 should be Saka 1947");
    }
}
