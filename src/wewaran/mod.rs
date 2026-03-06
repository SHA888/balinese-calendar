// src/wewaran/mod.rs
//
// Pawewaran — the system of concurrent cyclic weeks in Balinese timekeeping.
// All cycles run simultaneously; a day's "character" is the intersection of all.
//
// Cycles implemented (1-day through 10-day):
//   Ekawara   (1)  — always Luang
//   Dwiwara   (2)  — Menga / Pepet
//   Triwara   (3)  — Pasah / Beteng / Kajeng
//   Caturwara (4)  — Sri / Laba / Jaya / Menala
//   Pancawara (5)  — Umanis / Paing / Pon / Wage / Kliwon
//   Sadwara   (6)  — Tungleh / Aryang / Urukung / Paniron / Was / Maulu
//   Saptawara (7)  — Redite / Soma / Anggara / Buda / Wraspati / Sukra / Saniscara
//   Astawara  (8)  — Sri / Indra / Guru / Yama / Ludra / Brahma / Kala / Uma
//   Sangawara (9)  — Dangu / Jangur / Gigis / Nohan / Ogan / Erangan / Urungan / Tulus / Dadi
//   Dasawara  (10) — derived from Pancawara × Saptawara urip combination
//
// Source: babadbali.com (Yayasan Bali Galang) wewaran algorithms.

use crate::utils::PAWUKON_EPOCH_JDN;

// ─────────────────────────────────────────────────────────────────────────────
// Shared computation: Pawukon day index (0–209) from JDN
// ─────────────────────────────────────────────────────────────────────────────

pub fn pawukon_day(jdn: i64) -> u16 {
    (((jdn - PAWUKON_EPOCH_JDN) % 210 + 210) % 210) as u16
}

// ─────────────────────────────────────────────────────────────────────────────
// EKAWARA  (1-day cycle)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ekawara {
    Luang,
}
impl Ekawara {
    pub fn from_jdn(_jdn: i64) -> Self {
        Ekawara::Luang
    }
    pub fn name(&self) -> &'static str {
        "Luang"
    }
    pub fn urip(&self) -> u8 {
        1
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DWIWARA  (2-day cycle)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dwiwara {
    Menga = 0,
    Pepet = 1,
}

impl Dwiwara {
    pub fn from_jdn(jdn: i64) -> Self {
        match pawukon_day(jdn) % 2 {
            0 => Dwiwara::Menga,
            _ => Dwiwara::Pepet,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Dwiwara::Menga => "Menga",
            Dwiwara::Pepet => "Pepet",
        }
    }
    pub fn urip(&self) -> u8 {
        match self {
            Dwiwara::Menga => 5,
            Dwiwara::Pepet => 4,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// TRIWARA  (3-day cycle)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Triwara {
    Pasah = 0,
    Beteng = 1,
    Kajeng = 2,
}

impl Triwara {
    pub fn from_jdn(jdn: i64) -> Self {
        match pawukon_day(jdn) % 3 {
            0 => Triwara::Pasah,
            1 => Triwara::Beteng,
            _ => Triwara::Kajeng,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Triwara::Pasah => "Pasah",
            Triwara::Beteng => "Beteng",
            Triwara::Kajeng => "Kajeng",
        }
    }
    pub fn urip(&self) -> u8 {
        match self {
            Triwara::Pasah => 9,
            Triwara::Beteng => 4,
            Triwara::Kajeng => 7,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// CATURWARA  (4-day cycle)
// Lookup table — not a simple modulo; follows traditional sequence.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Caturwara {
    Sri = 0,
    Laba = 1,
    Jaya = 2,
    Menala = 3,
}

impl Caturwara {
    pub fn from_jdn(jdn: i64) -> Self {
        // Caturwara cycles mod 4 through the Pawukon day.
        // NOTE: The classical Caturwara has a complex lookup that skips certain
        // positions in a 17-day pattern. This simplified mod-4 implementation
        // matches the majority of days and requires validation against a full
        // babadbali.com-derived lookup table. TODO: replace with authoritative table.
        match pawukon_day(jdn) % 4 {
            0 => Caturwara::Sri,
            1 => Caturwara::Laba,
            2 => Caturwara::Jaya,
            _ => Caturwara::Menala,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Caturwara::Sri => "Sri",
            Caturwara::Laba => "Laba",
            Caturwara::Jaya => "Jaya",
            Caturwara::Menala => "Menala",
        }
    }
    pub fn urip(&self) -> u8 {
        match self {
            Caturwara::Sri => 6,
            Caturwara::Laba => 5,
            Caturwara::Jaya => 8,
            Caturwara::Menala => 9,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PANCAWARA  (5-day cycle)
// The most agriculturally significant cycle. Market days, planting signals.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pancawara {
    Umanis = 0,
    Paing = 1,
    Pon = 2,
    Wage = 3,
    Kliwon = 4,
}

impl Pancawara {
    pub fn from_jdn(jdn: i64) -> Self {
        match pawukon_day(jdn) % 5 {
            0 => Pancawara::Umanis,
            1 => Pancawara::Paing,
            2 => Pancawara::Pon,
            3 => Pancawara::Wage,
            _ => Pancawara::Kliwon,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Pancawara::Umanis => "Umanis",
            Pancawara::Paing => "Paing",
            Pancawara::Pon => "Pon",
            Pancawara::Wage => "Wage",
            Pancawara::Kliwon => "Kliwon",
        }
    }
    /// Traditional urip (life force value) — used in Dasawara and ala-ayuning dewasa.
    pub fn urip(&self) -> u8 {
        match self {
            Pancawara::Umanis => 5,
            Pancawara::Paing => 9,
            Pancawara::Pon => 7,
            Pancawara::Wage => 4,
            Pancawara::Kliwon => 8,
        }
    }
    /// Agricultural signal — traditional planting / activity guidance.
    pub fn agri_signal(&self) -> &'static str {
        match self {
            Pancawara::Umanis => "planting_favorable",
            Pancawara::Paing => "neutral",
            Pancawara::Pon => "harvest_favorable",
            Pancawara::Wage => "rest_avoid_planting",
            Pancawara::Kliwon => "ceremony_spiritual",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SADWARA  (6-day cycle)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sadwara {
    Tungleh = 0,
    Aryang = 1,
    Urukung = 2,
    Paniron = 3,
    Was = 4,
    Maulu = 5,
}

impl Sadwara {
    pub fn from_jdn(jdn: i64) -> Self {
        match pawukon_day(jdn) % 6 {
            0 => Sadwara::Tungleh,
            1 => Sadwara::Aryang,
            2 => Sadwara::Urukung,
            3 => Sadwara::Paniron,
            4 => Sadwara::Was,
            _ => Sadwara::Maulu,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Sadwara::Tungleh => "Tungleh",
            Sadwara::Aryang => "Aryang",
            Sadwara::Urukung => "Urukung",
            Sadwara::Paniron => "Paniron",
            Sadwara::Was => "Was",
            Sadwara::Maulu => "Maulu",
        }
    }
    pub fn urip(&self) -> u8 {
        match self {
            Sadwara::Tungleh => 7,
            Sadwara::Aryang => 6,
            Sadwara::Urukung => 8,
            Sadwara::Paniron => 9,
            Sadwara::Was => 3,
            Sadwara::Maulu => 5,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SAPTAWARA  (7-day week) — aligns with ISO weekday
// Redite = Sunday, Soma = Monday ... Saniscara = Saturday
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Saptawara {
    Redite = 0,    // Sunday
    Soma = 1,      // Monday
    Anggara = 2,   // Tuesday
    Buda = 3,      // Wednesday
    Wraspati = 4,  // Thursday
    Sukra = 5,     // Friday
    Saniscara = 6, // Saturday
}

impl Saptawara {
    /// JDN modular mapping — calibrated against J2000.0 (Jan 1, 2000 = Saturday = Saniscara).
    /// JDN 2451545 % 7 = 5 → Saniscara. All other days follow cyclically.
    pub fn from_jdn(jdn: i64) -> Self {
        match ((jdn % 7) + 7) % 7 {
            0 => Saptawara::Soma,
            1 => Saptawara::Anggara,
            2 => Saptawara::Buda,
            3 => Saptawara::Wraspati,
            4 => Saptawara::Sukra,
            5 => Saptawara::Saniscara,
            _ => Saptawara::Redite,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Saptawara::Redite => "Redite",
            Saptawara::Soma => "Soma",
            Saptawara::Anggara => "Anggara",
            Saptawara::Buda => "Buda",
            Saptawara::Wraspati => "Wraspati",
            Saptawara::Sukra => "Sukra",
            Saptawara::Saniscara => "Saniscara",
        }
    }
    pub fn name_indonesian(&self) -> &'static str {
        match self {
            Saptawara::Redite => "Minggu",
            Saptawara::Soma => "Senin",
            Saptawara::Anggara => "Selasa",
            Saptawara::Buda => "Rabu",
            Saptawara::Wraspati => "Kamis",
            Saptawara::Sukra => "Jumat",
            Saptawara::Saniscara => "Sabtu",
        }
    }
    pub fn urip(&self) -> u8 {
        match self {
            Saptawara::Redite => 5,
            Saptawara::Soma => 4,
            Saptawara::Anggara => 3,
            Saptawara::Buda => 7,
            Saptawara::Wraspati => 8,
            Saptawara::Sukra => 6,
            Saptawara::Saniscara => 9,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ASTAWARA  (8-day cycle) — lookup table
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Astawara {
    Sri = 0,
    Indra = 1,
    Guru = 2,
    Yama = 3,
    Ludra = 4,
    Brahma = 5,
    Kala = 6,
    Uma = 7,
}

impl Astawara {
    pub fn from_jdn(jdn: i64) -> Self {
        // Astawara follows a complex pattern derived from the pawukon day.
        // The 8-day and 210-day cycles share LCM 840; after 4 Pawukon cycles (840 days)
        // the Astawara resets. Uses a known lookup approach.
        let pd = pawukon_day(jdn) as usize;
        // Pattern: within each 8-day block, sequence is Sri,Indra,Guru,Yama,Ludra,Brahma,Kala,Uma
        // Special adjustment at day 63 boundaries (ngunaratri-aligned)
        let adjusted = if pd < 63 {
            pd
        } else if pd < 126 {
            pd + 1
        } else if pd < 189 {
            pd + 2
        } else {
            pd + 3
        };
        match adjusted % 8 {
            0 => Astawara::Sri,
            1 => Astawara::Indra,
            2 => Astawara::Guru,
            3 => Astawara::Yama,
            4 => Astawara::Ludra,
            5 => Astawara::Brahma,
            6 => Astawara::Kala,
            _ => Astawara::Uma,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Astawara::Sri => "Sri",
            Astawara::Indra => "Indra",
            Astawara::Guru => "Guru",
            Astawara::Yama => "Yama",
            Astawara::Ludra => "Ludra",
            Astawara::Brahma => "Brahma",
            Astawara::Kala => "Kala",
            Astawara::Uma => "Uma",
        }
    }
    pub fn urip(&self) -> u8 {
        match self {
            Astawara::Sri => 6,
            Astawara::Indra => 5,
            Astawara::Guru => 8,
            Astawara::Yama => 9,
            Astawara::Ludra => 3,
            Astawara::Brahma => 7,
            Astawara::Kala => 1,
            Astawara::Uma => 4,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// SANGAWARA  (9-day cycle)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sangawara {
    Dangu = 0,
    Jangur = 1,
    Gigis = 2,
    Nohan = 3,
    Ogan = 4,
    Erangan = 5,
    Urungan = 6,
    Tulus = 7,
    Dadi = 8,
}

impl Sangawara {
    pub fn from_jdn(jdn: i64) -> Self {
        let pd = pawukon_day(jdn) as usize;
        // Sangawara starts at day 3 of the pawukon cycle
        let adjusted = (pd + 210 - 3) % 9;
        match adjusted {
            0 => Sangawara::Dangu,
            1 => Sangawara::Jangur,
            2 => Sangawara::Gigis,
            3 => Sangawara::Nohan,
            4 => Sangawara::Ogan,
            5 => Sangawara::Erangan,
            6 => Sangawara::Urungan,
            7 => Sangawara::Tulus,
            _ => Sangawara::Dadi,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Sangawara::Dangu => "Dangu",
            Sangawara::Jangur => "Jangur",
            Sangawara::Gigis => "Gigis",
            Sangawara::Nohan => "Nohan",
            Sangawara::Ogan => "Ogan",
            Sangawara::Erangan => "Erangan",
            Sangawara::Urungan => "Urungan",
            Sangawara::Tulus => "Tulus",
            Sangawara::Dadi => "Dadi",
        }
    }
    pub fn urip(&self) -> u8 {
        match self {
            Sangawara::Dangu => 5,
            Sangawara::Jangur => 8,
            Sangawara::Gigis => 9,
            Sangawara::Nohan => 3,
            Sangawara::Ogan => 7,
            Sangawara::Erangan => 1,
            Sangawara::Urungan => 4,
            Sangawara::Tulus => 6,
            Sangawara::Dadi => 8,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// DASAWARA  (10-day cycle)
// Derived from combined urip of Pancawara + Saptawara → lookup table.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dasawara {
    Pandita = 0,
    Pati = 1,
    Suka = 2,
    Duka = 3,
    Sri = 4,
    Manuh = 5,
    Manusa = 6,
    Raja = 7,
    Dewa = 8,
    Raksasa = 9,
}

impl Dasawara {
    /// Derived from (pancawara.urip + saptawara.urip) % 10
    pub fn from_wewaran(pancawara: &Pancawara, saptawara: &Saptawara) -> Self {
        let sum = (pancawara.urip() + saptawara.urip()) % 10;
        match sum {
            0 => Dasawara::Pandita,
            1 => Dasawara::Pati,
            2 => Dasawara::Suka,
            3 => Dasawara::Duka,
            4 => Dasawara::Sri,
            5 => Dasawara::Manuh,
            6 => Dasawara::Manusa,
            7 => Dasawara::Raja,
            8 => Dasawara::Dewa,
            _ => Dasawara::Raksasa,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Dasawara::Pandita => "Pandita",
            Dasawara::Pati => "Pati",
            Dasawara::Suka => "Suka",
            Dasawara::Duka => "Duka",
            Dasawara::Sri => "Sri",
            Dasawara::Manuh => "Manuh",
            Dasawara::Manusa => "Manusa",
            Dasawara::Raja => "Raja",
            Dasawara::Dewa => "Dewa",
            Dasawara::Raksasa => "Raksasa",
        }
    }
    pub fn urip(&self) -> u8 {
        match self {
            Dasawara::Pandita => 7,
            Dasawara::Pati => 1,
            Dasawara::Suka => 4,
            Dasawara::Duka => 8,
            Dasawara::Sri => 6,
            Dasawara::Manuh => 3,
            Dasawara::Manusa => 5,
            Dasawara::Raja => 8,
            Dasawara::Dewa => 8,
            Dasawara::Raksasa => 9,
        }
    }
}
