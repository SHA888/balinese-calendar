// src/pawukon.rs
//
// Wuku — the 30-week (210-day) Pawukon calendar.
// Each week (7 days) has a name, urip value, and traditional agricultural/ecological tag.

use crate::wewaran::pawukon_day;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Wuku {
    Sinta = 0,
    Landep = 1,
    Ukir = 2,
    Kulantir = 3,
    Tolu = 4,
    Gumbreg = 5,
    Wariga = 6,
    Warigadean = 7,
    Julungwangi = 8,
    Sungsang = 9,
    Dungulan = 10,
    Kuningan = 11,
    Langkir = 12,
    Medangsia = 13,
    Pujut = 14,
    Pahang = 15,
    Krulut = 16,
    Merakih = 17,
    Tambir = 18,
    Medangkungan = 19,
    Matal = 20,
    Uye = 21,
    Menail = 22,
    Prangbakat = 23,
    Bala = 24,
    Ugu = 25,
    Wayang = 26,
    Kelawu = 27,
    Dukut = 28,
    Watugunung = 29,
}

impl Wuku {
    pub fn from_jdn(jdn: i64) -> Self {
        let wuku_idx = (pawukon_day(jdn) / 7) as usize;
        Self::from_index(wuku_idx)
    }

    pub fn from_index(idx: usize) -> Self {
        match idx % 30 {
            0 => Wuku::Sinta,
            1 => Wuku::Landep,
            2 => Wuku::Ukir,
            3 => Wuku::Kulantir,
            4 => Wuku::Tolu,
            5 => Wuku::Gumbreg,
            6 => Wuku::Wariga,
            7 => Wuku::Warigadean,
            8 => Wuku::Julungwangi,
            9 => Wuku::Sungsang,
            10 => Wuku::Dungulan,
            11 => Wuku::Kuningan,
            12 => Wuku::Langkir,
            13 => Wuku::Medangsia,
            14 => Wuku::Pujut,
            15 => Wuku::Pahang,
            16 => Wuku::Krulut,
            17 => Wuku::Merakih,
            18 => Wuku::Tambir,
            19 => Wuku::Medangkungan,
            20 => Wuku::Matal,
            21 => Wuku::Uye,
            22 => Wuku::Menail,
            23 => Wuku::Prangbakat,
            24 => Wuku::Bala,
            25 => Wuku::Ugu,
            26 => Wuku::Wayang,
            27 => Wuku::Kelawu,
            28 => Wuku::Dukut,
            _ => Wuku::Watugunung,
        }
    }

    /// Returns 0-based week index within the 210-day Pawukon
    pub fn index(&self) -> usize {
        *self as usize
    }

    /// Day offset of this wuku's start within the 210-day Pawukon cycle
    pub fn start_day(&self) -> u16 {
        (self.index() as u16) * 7
    }

    pub fn name(&self) -> &'static str {
        match self {
            Wuku::Sinta => "Sinta",
            Wuku::Landep => "Landep",
            Wuku::Ukir => "Ukir",
            Wuku::Kulantir => "Kulantir",
            Wuku::Tolu => "Tolu",
            Wuku::Gumbreg => "Gumbreg",
            Wuku::Wariga => "Wariga",
            Wuku::Warigadean => "Warigadean",
            Wuku::Julungwangi => "Julungwangi",
            Wuku::Sungsang => "Sungsang",
            Wuku::Dungulan => "Dungulan",
            Wuku::Kuningan => "Kuningan",
            Wuku::Langkir => "Langkir",
            Wuku::Medangsia => "Medangsia",
            Wuku::Pujut => "Pujut",
            Wuku::Pahang => "Pahang",
            Wuku::Krulut => "Krulut",
            Wuku::Merakih => "Merakih",
            Wuku::Tambir => "Tambir",
            Wuku::Medangkungan => "Medangkungan",
            Wuku::Matal => "Matal",
            Wuku::Uye => "Uye",
            Wuku::Menail => "Menail",
            Wuku::Prangbakat => "Prangbakat",
            Wuku::Bala => "Bala",
            Wuku::Ugu => "Ugu",
            Wuku::Wayang => "Wayang",
            Wuku::Kelawu => "Kelawu",
            Wuku::Dukut => "Dukut",
            Wuku::Watugunung => "Watugunung",
        }
    }

    pub fn urip(&self) -> u8 {
        // Traditional wuku urip values from "Pokok-Pokok Wariga" (Ardhana, 2005)
        const URIP: [u8; 30] = [
            7, 1, 4, 6, 5, 8, 9, 3, 7, 8, // Sinta–Sungsang
            5, 7, 5, 5, 7, 8, 8, 9, 5, 6, // Dungulan–Medangkungan
            5, 4, 9, 8, 5, 5, 6, 7, 5, 7, // Matal–Watugunung
        ];
        URIP[self.index()]
    }

    /// Traditional agricultural/ecological association
    pub fn ecology_tag(&self) -> &'static str {
        match self {
            Wuku::Sinta => "planting_start",
            Wuku::Landep => "sharpening",
            Wuku::Ukir => "growth_watch",
            Wuku::Kulantir => "soil_fertility",
            Wuku::Tolu => "water_watch",
            Wuku::Gumbreg => "pest_alert",
            Wuku::Wariga => "weather_change",
            Wuku::Warigadean => "weather_change",
            Wuku::Julungwangi => "flowering",
            Wuku::Sungsang => "wind_watch", // ← current week as of 2026-03-06
            Wuku::Dungulan => "harvest_prep",
            Wuku::Kuningan => "harvest",
            Wuku::Langkir => "post_harvest",
            Wuku::Medangsia => "soil_rest",
            Wuku::Pujut => "soil_rest",
            Wuku::Pahang => "replanting",
            Wuku::Krulut => "water_mgmt",
            Wuku::Merakih => "pest_watch",
            Wuku::Tambir => "rain_watch",
            Wuku::Medangkungan => "drought_watch",
            Wuku::Matal => "planting_2",
            Wuku::Uye => "growth_watch",
            Wuku::Menail => "soil_fertility",
            Wuku::Prangbakat => "wind_alert",
            Wuku::Bala => "storm_watch",
            Wuku::Ugu => "harvest_2",
            Wuku::Wayang => "spiritual_peak",
            Wuku::Kelawu => "transition",
            Wuku::Dukut => "soil_prep",
            Wuku::Watugunung => "cycle_end",
        }
    }
}

/// Day within the current Wuku (0–6, where 0 = Redite/Sunday of that wuku)
pub fn wuku_day_of_week(jdn: i64) -> u8 {
    (pawukon_day(jdn) % 7) as u8
}

/// Absolute Pawukon position (0–209) within the 210-day cycle
pub fn pawukon_position(jdn: i64) -> u16 {
    pawukon_day(jdn)
}
