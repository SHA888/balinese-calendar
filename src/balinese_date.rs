// src/balinese_date.rs
//
// BalineseDate — the primary public struct.
// Integrates all calendar subsystems into a single immutable value type.
//
// Design: immutable, computed on construction, zero runtime allocation
// after build (all enums are Copy).

use chrono::{Datelike, NaiveDate};

use crate::boundary::DayBoundary;
use crate::error::BalineseDateError;
use crate::paringkelan::{
    Ingkel, Jejepan, Lintang, PancaSuda, Pararasan, Rakam, WatekAlit, WatekMadya,
};
use crate::pawukon::{Wuku, wuku_day_of_week};
use crate::rahinan::Rahinan;
use crate::sasih::{Sasih, SasihDayInfo, SasihResult};
use crate::utils::{gregorian_to_jdn, jdn_to_gregorian};
use crate::wewaran::{
    Astawara, Caturwara, Dasawara, Dwiwara, Ekawara, Pancawara, Sadwara, Sangawara, Saptawara,
    Triwara, pawukon_day as pawukon_day_raw,
};

// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BalineseDate {
    // ── Gregorian ────────────────────────────────────────────────────────────
    pub gregorian_year: i32,
    pub gregorian_month: u32,
    pub gregorian_day: u32,
    pub jdn: i64,

    // ── Pawukon ──────────────────────────────────────────────────────────────
    pub pawukon_day: u16, // 0–209 absolute position in 210-day cycle
    pub wuku: Wuku,
    pub wuku_day: u8, // 0–6 day within wuku

    // ── Wewaran (concurrent week cycles) ─────────────────────────────────────
    pub ekawara: Ekawara,
    pub dwiwara: Dwiwara,
    pub triwara: Triwara,
    pub caturwara: Caturwara,
    pub pancawara: Pancawara,
    pub sadwara: Sadwara,
    pub saptawara: Saptawara,
    pub astawara: Astawara,
    pub sangawara: Sangawara,
    pub dasawara: Dasawara,

    // ── Paringkelan ───────────────────────────────────────────────────────────
    pub jejepan: Jejepan,
    pub ingkel: Ingkel,
    pub watek_madya: WatekMadya,
    pub watek_alit: WatekAlit,
    pub lintang: Lintang,
    pub panca_suda: PancaSuda,
    pub pararasan: Pararasan,
    pub rakam: Rakam,

    // ── Sasih / Saka ──────────────────────────────────────────────────────────
    pub saka_year: i32,
    pub sasih: Sasih,
    pub sasih_day: SasihDayInfo,
    pub is_nampih: bool,

    // ── Derived flags ─────────────────────────────────────────────────────────
    pub is_purnama: bool,
    pub is_tilem: bool,

    // ── Rahinan (holy days) ───────────────────────────────────────────────────
    pub rahinan: Vec<Rahinan>,
}

impl BalineseDate {
    // ── Constants ─────────────────────────────────────────────────────────────

    /// Bali time zone offset from UTC in hours (WITA = UTC+8).
    const BALI_UTC_OFFSET_HOURS: i64 = 8;

    // ── Constructors ─────────────────────────────────────────────────────────

    /// Create from a Gregorian calendar date.
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<Self, BalineseDateError> {
        let jdn = gregorian_to_jdn(year, month, day)?;
        Ok(Self::from_jdn_unchecked(jdn, year, month, day))
    }

    /// Create from a `chrono::NaiveDate`.
    pub fn from_naive_date(date: NaiveDate) -> Result<Self, BalineseDateError> {
        Self::from_ymd(date.year(), date.month(), date.day())
    }

    /// Create from a Julian Day Number (already validated externally).
    pub fn from_jdn(jdn: i64) -> Self {
        let (y, m, d) = jdn_to_gregorian(jdn);
        Self::from_jdn_unchecked(jdn, y, m, d)
    }

    /// Computes a Balinese date from Gregorian year, month, day using an explicit day boundary.
    ///
    /// The Balinese calendar day begins at sunrise rather than midnight. This method allows
    /// customizing how the sunrise is determined:
    ///
    /// - `DayBoundary::Midnight`: Uses Gregorian midnight (legacy behavior)
    /// - `DayBoundary::FixedSunrise(hour)`: Uses a fixed hour offset (e.g., 6 for 06:00 UTC)
    /// - `DayBoundary::Astronomical`: Calculates actual astronomical sunrise for given coordinates
    ///
    /// # Arguments
    ///
    /// * `year` - Gregorian year (1800-2200 CE)
    /// * `month` - Gregorian month (1-12)
    /// * `day` - Gregorian day (1-31, depending on month)
    /// * `boundary` - The day boundary definition to apply
    ///
    /// # Example
    ///
    /// ```rust
    /// use balinese_calendar::{BalineseDate, DayBoundary};
    /// let boundary = DayBoundary::FixedSunrise(6);
    /// let date = BalineseDate::from_ymd_with_boundary(2026, 3, 26, &boundary)?;
    /// # Ok::<(), balinese_calendar::BalineseDateError>(())
    /// ```
    pub fn from_ymd_with_boundary(
        year: i32,
        month: u32,
        day: u32,
        boundary: &DayBoundary,
    ) -> Result<Self, BalineseDateError> {
        // Convert Gregorian date to UTC datetime at midnight, then apply boundary
        let naive_date = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .ok_or(BalineseDateError::InvalidDate { year, month, day })?;
        let utc_datetime = naive_date
            .and_hms_opt(0, 0, 0)
            .ok_or(BalineseDateError::InvalidDate { year, month, day })?
            .and_utc();

        Self::from_utc_datetime_with_boundary(utc_datetime, boundary)
    }

    // ── Today ─────────────────────────────────────────────────────────────────

    /// Returns today's Balinese date using the default day boundary:
    /// `FixedSunrise(6)` — Bali sunrise offset over WITA (UTC+8).
    pub fn today() -> Result<Self, BalineseDateError> {
        Self::today_with_boundary(&DayBoundary::default())
    }

    /// Returns today's Balinese date using an explicit [`DayBoundary`].
    ///
    /// # Example
    /// ```rust
    /// use balinese_calendar::{BalineseDate, DayBoundary};
    /// let date = BalineseDate::today_with_boundary(&DayBoundary::FixedSunrise(6))?;
    /// # Ok::<(), balinese_calendar::BalineseDateError>(())
    /// ```
    pub fn today_with_boundary(boundary: &DayBoundary) -> Result<Self, BalineseDateError> {
        use chrono::Utc;
        let utc_now = Utc::now();
        Self::from_utc_datetime_with_boundary(utc_now, boundary)
    }

    /// Computes a Balinese date from a provided UTC datetime using an explicit [`DayBoundary`].
    ///
    /// This helper enables deterministic tests by injecting a fixed UTC instant.
    pub(crate) fn from_utc_datetime_with_boundary(
        utc_now: chrono::DateTime<chrono::Utc>,
        boundary: &DayBoundary,
    ) -> Result<Self, BalineseDateError> {
        let date = match boundary {
            DayBoundary::Midnight => {
                (utc_now + chrono::Duration::hours(Self::BALI_UTC_OFFSET_HOURS)).date_naive()
            }
            DayBoundary::FixedSunrise(hour) => {
                if *hour > 23 {
                    return Err(BalineseDateError::InvalidBoundaryHour(*hour));
                }
                let offset_hours = Self::BALI_UTC_OFFSET_HOURS - (*hour as i64);
                (utc_now + chrono::Duration::hours(offset_hours)).date_naive()
            }
            #[cfg(feature = "astronomical")]
            DayBoundary::Astronomical { lat, lon } => {
                // Calculate astronomical sunrise for the given coordinates
                use sunrise::{Coordinates, SolarDay, SolarEvent};

                let utc_date = utc_now.date_naive();
                let coordinates = Coordinates::new(*lat, *lon)
                    .ok_or(BalineseDateError::AstronomicalCalculationFailed)?;
                let solar_day = SolarDay::new(coordinates, utc_date);

                // Calculate sunrise time in UTC for the given date and coordinates
                let sunrise_datetime = solar_day
                    .event_time(SolarEvent::Sunrise)
                    .ok_or(BalineseDateError::AstronomicalCalculationFailed)?;

                // The Balinese day starts at astronomical sunrise.
                // Determine which calendar day we're in based on whether utc_now
                // is before or after today's sunrise.
                if utc_now < sunrise_datetime {
                    // Before today's sunrise, we're still in yesterday's Balinese day
                    (sunrise_datetime - chrono::Duration::days(1)).date_naive()
                } else {
                    // After today's sunrise, we're in today's Balinese day
                    sunrise_datetime.date_naive()
                }
            }
        };
        Self::from_naive_date(date)
    }

    // ── Internal construction ─────────────────────────────────────────────────

    fn from_jdn_unchecked(jdn: i64, year: i32, month: u32, day: u32) -> Self {
        // Compute pawukon_day once and reuse for performance optimization
        let pawukon_day = pawukon_day_raw(jdn);

        // Wewaran - pass pawukon_day to avoid recomputation
        let ekawara = Ekawara::from_jdn(jdn);
        let dwiwara = Dwiwara::from_jdn(jdn);
        let triwara = Triwara::from_jdn(jdn);
        let caturwara = Caturwara::from_jdn(jdn);
        let pancawara = Pancawara::from_jdn(jdn);
        let sadwara = Sadwara::from_jdn(jdn);
        let saptawara = Saptawara::from_jdn(jdn);
        let astawara = Astawara::from_jdn(jdn);
        let sangawara = Sangawara::from_jdn(jdn);
        let dasawara = Dasawara::from_wewaran(&pancawara, &saptawara);

        // Pawukon - reuse computed pawukon_day
        let wuku = Wuku::from_jdn(jdn);
        let wuku_day = wuku_day_of_week(jdn);

        // Paringkelan - use optimized constructors with precomputed pawukon_day
        let jejepan = Jejepan::from_pawukon_day(pawukon_day);
        let ingkel = Ingkel::from_pawukon_day(pawukon_day);
        let watek_madya = WatekMadya::from_pawukon_day(pawukon_day);
        let watek_alit = WatekAlit::from_pawukon_day(pawukon_day);
        let lintang = Lintang::from_wewaran(&pancawara, &saptawara);
        let panca_suda = PancaSuda::from_wewaran(&pancawara, &saptawara);
        let pararasan = Pararasan::from_wewaran(&pancawara, &saptawara);
        let rakam = Rakam::from_wewaran(&pancawara, &saptawara);

        // Sasih
        let sasih_result = SasihResult::from_jdn(jdn);
        let is_purnama = sasih_result.day_info.is_purnama();
        let is_tilem = sasih_result.day_info.is_tilem();

        // Rahinan
        let rahinan = Rahinan::detect(
            &wuku,
            &pancawara,
            &saptawara,
            &triwara,
            &sasih_result.sasih,
            &sasih_result.day_info,
        );

        BalineseDate {
            gregorian_year: year,
            gregorian_month: month,
            gregorian_day: day,
            jdn,
            pawukon_day, // Store the computed value
            wuku,
            wuku_day,
            ekawara,
            dwiwara,
            triwara,
            caturwara,
            pancawara,
            sadwara,
            saptawara,
            astawara,
            sangawara,
            dasawara,
            jejepan,
            ingkel,
            watek_madya,
            watek_alit,
            lintang,
            panca_suda,
            pararasan,
            rakam,
            saka_year: sasih_result.saka_year,
            sasih: sasih_result.sasih,
            sasih_day: sasih_result.day_info,
            is_nampih: sasih_result.is_nampih,
            is_purnama,
            is_tilem,
            rahinan,
        }
    }

    // ── Display helpers ───────────────────────────────────────────────────────

    /// Full traditional Balinese date string.
    /// Format: "Saptawara Pancawara Wuku, Tithi Sasih Saka-year"
    /// Example: "Kamis Umanis Sungsang, Penanggal 15 Kasanga 1948"
    pub fn to_balinese_string(&self) -> String {
        let tithi = match self.sasih_day {
            SasihDayInfo::Single(phase) => phase.to_string(),
            SasihDayInfo::Ngunaratri { primary, .. } => format!("Ngunaratri ({primary})"),
        };

        format!(
            "{} {} {}, {} {} Saka {}",
            self.saptawara.name(),
            self.pancawara.name(),
            self.wuku.name(),
            tithi,
            self.sasih.name(),
            self.saka_year,
        )
    }

    /// Flat record for data analysis and serialization.
    /// Returns a flattened structure suitable for serialization to Parquet, Arrow IPC, or other columnar formats.
    pub fn to_flat_record(&self) -> FlatRecord {
        FlatRecord {
            gregorian_year: self.gregorian_year,
            gregorian_month: self.gregorian_month,
            gregorian_day: self.gregorian_day,
            jdn: self.jdn,
            saka_year: self.saka_year,
            sasih_id: self.sasih as u8,
            sasih_name: self.sasih.name(),
            sasih_season_tag: self.sasih.season_tag(),
            pancaroba_flag: self.sasih.is_pancaroba(),
            is_purnama: self.is_purnama,
            is_tilem: self.is_tilem,
            is_ngunaratri: self.sasih_day.is_ngunaratri(),
            is_nampih: self.is_nampih,
            tithi: self.sasih_day.tithi_number(),
            pawukon_day: self.pawukon_day,
            wuku_id: self.wuku as u8,
            wuku_name: self.wuku.name(),
            wuku_ecology_tag: self.wuku.ecology_tag(),
            pancawara_id: self.pancawara as u8,
            pancawara_name: self.pancawara.name(),
            pancawara_urip: self.pancawara.urip(),
            saptawara_id: self.saptawara as u8,
            saptawara_name: self.saptawara.name(),
            saptawara_urip: self.saptawara.urip(),
            dasawara_name: self.dasawara.name(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    /// 2026-03-08 00:30 WITA = 2026-03-07 16:30 UTC
    /// With FixedSunrise(6): UTC+2 offset → date = 2026-03-07
    /// With Midnight:        UTC+8 offset → date = 2026-03-08 (Gregorian)
    #[test]
    fn fixed_sunrise_before_dawn_returns_prior_gregorian_day() {
        let utc = chrono::DateTime::parse_from_rfc3339("2026-03-07T16:30:00Z").unwrap();
        let date = BalineseDate::from_utc_datetime_with_boundary(
            utc.with_timezone(&chrono::Utc),
            &DayBoundary::FixedSunrise(6),
        )
        .unwrap();
        assert_eq!(date.gregorian_year, 2026);
        assert_eq!(date.gregorian_month, 3);
        assert_eq!(date.gregorian_day, 7);
    }

    #[test]
    fn midnight_boundary_returns_gregorian_date_unchanged() {
        let utc = chrono::DateTime::parse_from_rfc3339("2026-03-07T16:30:00Z").unwrap();
        let date = BalineseDate::from_utc_datetime_with_boundary(
            utc.with_timezone(&chrono::Utc),
            &DayBoundary::Midnight,
        )
        .unwrap();
        assert_eq!(date.gregorian_year, 2026);
        assert_eq!(date.gregorian_month, 3);
        assert_eq!(date.gregorian_day, 8);
    }

    #[test]
    fn fixed_sunrise_after_dawn_returns_same_day() {
        // 2026-03-08 07:15 WITA = 2026-03-07 23:15 UTC
        let utc = chrono::DateTime::parse_from_rfc3339("2026-03-07T23:15:00Z").unwrap();
        let date = BalineseDate::from_utc_datetime_with_boundary(
            utc.with_timezone(&chrono::Utc),
            &DayBoundary::FixedSunrise(6),
        )
        .unwrap();
        assert_eq!(date.gregorian_year, 2026);
        assert_eq!(date.gregorian_month, 3);
        assert_eq!(date.gregorian_day, 8);
    }

    #[test]
    fn fixed_sunrise_valid_range() {
        assert!(
            BalineseDate::from_utc_datetime_with_boundary(
                chrono::Utc.with_ymd_and_hms(2026, 3, 7, 0, 0, 0).unwrap(),
                &DayBoundary::FixedSunrise(0),
            )
            .is_ok()
        );
        assert!(
            BalineseDate::from_utc_datetime_with_boundary(
                chrono::Utc.with_ymd_and_hms(2026, 3, 7, 0, 0, 0).unwrap(),
                &DayBoundary::FixedSunrise(23),
            )
            .is_ok()
        );
        let err = BalineseDate::from_utc_datetime_with_boundary(
            chrono::Utc.with_ymd_and_hms(2026, 3, 7, 0, 0, 0).unwrap(),
            &DayBoundary::FixedSunrise(24),
        )
        .unwrap_err();
        assert!(matches!(err, BalineseDateError::InvalidBoundaryHour(24)));
    }

    #[test]
    fn day_boundary_default_is_fixed_sunrise_6() {
        assert_eq!(DayBoundary::default(), DayBoundary::FixedSunrise(6));
    }

    #[test]
    fn day_boundary_midnight_variant() {
        assert_eq!(DayBoundary::Midnight, DayBoundary::Midnight);
    }

    #[test]
    fn day_boundary_clone_and_debug() {
        let boundary = DayBoundary::FixedSunrise(6);
        let cloned = boundary.clone();
        assert_eq!(boundary, cloned);
        assert_eq!(format!("{boundary:?}"), "FixedSunrise(6)");
    }
}

/// Flat record struct for columnar data serialization.
/// Suitable for Arrow, Parquet, and other columnar formats.
///
/// # Serde Limitations
///
/// When the `serde` feature is enabled, this struct can be serialized to JSON
/// but **cannot be deserialized** from JSON due to the `&'static str` fields
/// (`sasih_name`, `sasih_season_tag`, etc.). This is a design constraint
/// required for zero-copy string references. For API integration, use this
/// struct for JSON output only.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlatRecord {
    pub gregorian_year: i32,
    pub gregorian_month: u32,
    pub gregorian_day: u32,
    pub jdn: i64,
    pub saka_year: i32,
    pub sasih_id: u8,
    pub sasih_name: &'static str,
    pub sasih_season_tag: &'static str,
    pub pancaroba_flag: bool,
    pub is_purnama: bool,
    pub is_tilem: bool,
    pub is_ngunaratri: bool,
    pub is_nampih: bool,
    pub tithi: u8,
    pub pawukon_day: u16,
    pub wuku_id: u8,
    pub wuku_name: &'static str,
    pub wuku_ecology_tag: &'static str,
    pub pancawara_id: u8,
    pub pancawara_name: &'static str,
    pub pancawara_urip: u8,
    pub saptawara_id: u8,
    pub saptawara_name: &'static str,
    pub saptawara_urip: u8,
    pub dasawara_name: &'static str,
}
