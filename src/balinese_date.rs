// src/balinese_date.rs
//
// BalineseDate — the primary public struct.
// Integrates all calendar subsystems into a single immutable value type.
//
// Design: immutable, computed on construction, zero runtime allocation
// after build (all enums are Copy).

use chrono::{Datelike, NaiveDate};

use crate::error::BalineseDateError;
use crate::utils::{gregorian_to_jdn, jdn_to_gregorian};
use crate::wewaran::{
    pawukon_day as pawukon_day_raw,
    Ekawara, Dwiwara, Triwara, Caturwara, Pancawara,
    Sadwara, Saptawara, Astawara, Sangawara, Dasawara,
};
use crate::pawukon::{Wuku, wuku_day_of_week};
use crate::sasih::{Sasih, SasihDayInfo, SasihResult};
use crate::paringkelan::{
    Jejepan, Ingkel, WatekMadya, WatekAlit,
    Lintang, PancaSuda, Pararasan, Rakam,
};
use crate::rahinan::Rahinan;

// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub struct BalineseDate {
    // ── Gregorian ────────────────────────────────────────────────────────────
    pub gregorian_year:  i32,
    pub gregorian_month: u32,
    pub gregorian_day:   u32,
    pub jdn:             i64,

    // ── Pawukon ──────────────────────────────────────────────────────────────
    pub pawukon_day:     u16,   // 0–209 absolute position in 210-day cycle
    pub wuku:            Wuku,
    pub wuku_day:        u8,    // 0–6 day within wuku

    // ── Wewaran (concurrent week cycles) ─────────────────────────────────────
    pub ekawara:    Ekawara,
    pub dwiwara:    Dwiwara,
    pub triwara:    Triwara,
    pub caturwara:  Caturwara,
    pub pancawara:  Pancawara,
    pub sadwara:    Sadwara,
    pub saptawara:  Saptawara,
    pub astawara:   Astawara,
    pub sangawara:  Sangawara,
    pub dasawara:   Dasawara,

    // ── Paringkelan ───────────────────────────────────────────────────────────
    pub jejepan:     Jejepan,
    pub ingkel:      Ingkel,
    pub watek_madya: WatekMadya,
    pub watek_alit:  WatekAlit,
    pub lintang:     Lintang,
    pub panca_suda:  PancaSuda,
    pub pararasan:   Pararasan,
    pub rakam:       Rakam,

    // ── Sasih / Saka ──────────────────────────────────────────────────────────
    pub saka_year:   i32,
    pub sasih:       Sasih,
    pub sasih_day:   SasihDayInfo,
    pub is_nampih:   bool,

    // ── Derived flags ─────────────────────────────────────────────────────────
    pub is_purnama:  bool,
    pub is_tilem:    bool,

    // ── Rahinan (holy days) ───────────────────────────────────────────────────
    pub rahinan:     Vec<Rahinan>,
}

impl BalineseDate {
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

    // ── Today ─────────────────────────────────────────────────────────────────

    /// Create for today's date (uses system clock via chrono).
    pub fn today() -> Result<Self, BalineseDateError> {
        let today = chrono::Local::now().date_naive();
        Self::from_naive_date(today)
    }

    // ── Internal construction ─────────────────────────────────────────────────

    fn from_jdn_unchecked(jdn: i64, year: i32, month: u32, day: u32) -> Self {
        // Wewaran
        let ekawara   = Ekawara::from_jdn(jdn);
        let dwiwara   = Dwiwara::from_jdn(jdn);
        let triwara   = Triwara::from_jdn(jdn);
        let caturwara = Caturwara::from_jdn(jdn);
        let pancawara = Pancawara::from_jdn(jdn);
        let sadwara   = Sadwara::from_jdn(jdn);
        let saptawara = Saptawara::from_jdn(jdn);
        let astawara  = Astawara::from_jdn(jdn);
        let sangawara = Sangawara::from_jdn(jdn);
        let dasawara  = Dasawara::from_wewaran(&pancawara, &saptawara);

        // Pawukon
        let pd       = pawukon_day_raw(jdn);
        let wuku     = Wuku::from_jdn(jdn);
        let wuku_day = wuku_day_of_week(jdn);

        // Paringkelan
        let jejepan    = Jejepan::from_jdn(jdn);
        let ingkel     = Ingkel::from_jdn(jdn);
        let watek_madya = WatekMadya::from_jdn(jdn);
        let watek_alit  = WatekAlit::from_jdn(jdn);
        let lintang    = Lintang::from_jdn(jdn);
        let panca_suda = PancaSuda::from_jdn(jdn);
        let pararasan  = Pararasan::from_jdn(jdn);
        let rakam      = Rakam::from_jdn(jdn);

        // Sasih
        let sasih_result = SasihResult::from_jdn(jdn);
        let is_purnama   = sasih_result.day_info.is_purnama();
        let is_tilem     = sasih_result.day_info.is_tilem();

        // Rahinan
        let rahinan = Rahinan::detect(
            &wuku, &pancawara, &saptawara, &triwara,
            &sasih_result.sasih, &sasih_result.day_info,
        );

        BalineseDate {
            gregorian_year:  year,
            gregorian_month: month,
            gregorian_day:   day,
            jdn,
            pawukon_day: pd,
            wuku,
            wuku_day,
            ekawara, dwiwara, triwara, caturwara, pancawara,
            sadwara, saptawara, astawara, sangawara, dasawara,
            jejepan, ingkel, watek_madya, watek_alit,
            lintang, panca_suda, pararasan, rakam,
            saka_year:  sasih_result.saka_year,
            sasih:      sasih_result.sasih,
            sasih_day:  sasih_result.day_info,
            is_nampih:  sasih_result.is_nampih,
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
        let tithi = match &self.sasih_day {
            SasihDayInfo::Penanggal(n)  => format!("Penanggal {n}"),
            SasihDayInfo::Purnama       => "Purnama".to_string(),
            SasihDayInfo::Pangelong(n)  => format!("Pangelong {n}"),
            SasihDayInfo::Tilem         => "Tilem".to_string(),
            SasihDayInfo::Ngunaratri { primary, .. } => {
                format!("Ngunaratri ({})", match primary.as_ref() {
                    SasihDayInfo::Penanggal(n) => format!("Penanggal {n}"),
                    SasihDayInfo::Purnama      => "Purnama".to_string(),
                    SasihDayInfo::Pangelong(n) => format!("Pangelong {n}"),
                    SasihDayInfo::Tilem        => "Tilem".to_string(),
                    _                          => "?".to_string(),
                })
            }
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
            gregorian_year:   self.gregorian_year,
            gregorian_month:  self.gregorian_month,
            gregorian_day:    self.gregorian_day,
            jdn:              self.jdn,
            saka_year:        self.saka_year,
            sasih_id:         self.sasih as u8,
            sasih_name:       self.sasih.name(),
            sasih_season_tag: self.sasih.season_tag(),
            pancaroba_flag:   self.sasih.is_pancaroba(),
            is_purnama:       self.is_purnama,
            is_tilem:         self.is_tilem,
            is_ngunaratri:    self.sasih_day.is_ngunaratri(),
            is_nampih:        self.is_nampih,
            tithi:            self.sasih_day.tithi_number(),
            pawukon_day:      self.pawukon_day,
            wuku_id:          self.wuku as u8,
            wuku_name:        self.wuku.name(),
            wuku_ecology_tag: self.wuku.ecology_tag(),
            pancawara_id:     self.pancawara as u8,
            pancawara_name:   self.pancawara.name(),
            pancawara_urip:   self.pancawara.urip(),
            saptawara_id:     self.saptawara as u8,
            saptawara_name:   self.saptawara.name(),
            saptawara_urip:   self.saptawara.urip(),
            dasawara_name:    self.dasawara.name(),
        }
    }
}

/// Flat record struct for columnar data serialization.
/// Suitable for Arrow, Parquet, and other columnar formats.
/// Add `#[derive(serde::Serialize)]` when enabling the `serde` feature.
#[derive(Debug, Clone, PartialEq)]
pub struct FlatRecord {
    pub gregorian_year:   i32,
    pub gregorian_month:  u32,
    pub gregorian_day:    u32,
    pub jdn:              i64,
    pub saka_year:        i32,
    pub sasih_id:         u8,
    pub sasih_name:       &'static str,
    pub sasih_season_tag: &'static str,
    pub pancaroba_flag:   bool,
    pub is_purnama:       bool,
    pub is_tilem:         bool,
    pub is_ngunaratri:    bool,
    pub is_nampih:        bool,
    pub tithi:            u8,
    pub pawukon_day:      u16,
    pub wuku_id:          u8,
    pub wuku_name:        &'static str,
    pub wuku_ecology_tag: &'static str,
    pub pancawara_id:     u8,
    pub pancawara_name:   &'static str,
    pub pancawara_urip:   u8,
    pub saptawara_id:     u8,
    pub saptawara_name:   &'static str,
    pub saptawara_urip:   u8,
    pub dasawara_name:    &'static str,
}
