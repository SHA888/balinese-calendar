// src/paringkelan/mod.rs
//
// Paringkelan — additional cyclic systems used for auspicious day determination.
// These run alongside Pawewaran and provide finer-grained activity guidance.
//
// Source: babadbali.com (Yayasan Bali Galang) paringkelan algorithm.

use crate::wewaran::{pawukon_day, Pancawara, Saptawara};

// ─────────────────────────────────────────────────────────────────────────────
// JEJEPAN  (6-day cycle, agricultural activities)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Jejepan {
    Mina = 0,
    Paksian = 1,
    Sato = 2,
    Cokcok = 3,
    Godel = 4,
    Lembu = 5,
}

impl Jejepan {
    pub fn from_jdn(jdn: i64) -> Self {
        match pawukon_day(jdn) % 6 {
            0 => Jejepan::Mina,
            1 => Jejepan::Paksian,
            2 => Jejepan::Sato,
            3 => Jejepan::Cokcok,
            4 => Jejepan::Godel,
            _ => Jejepan::Lembu,
        }
    }

    /// Optimized constructor that accepts precomputed pawukon_day
    pub fn from_pawukon_day(pawukon_day: u16) -> Self {
        match pawukon_day % 6 {
            0 => Jejepan::Mina,
            1 => Jejepan::Paksian,
            2 => Jejepan::Sato,
            3 => Jejepan::Cokcok,
            4 => Jejepan::Godel,
            _ => Jejepan::Lembu,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Jejepan::Mina => "Mina",
            Jejepan::Paksian => "Paksian",
            Jejepan::Sato => "Sato",
            Jejepan::Cokcok => "Cokcok",
            Jejepan::Godel => "Godel",
            Jejepan::Lembu => "Lembu",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// INGKEL  (7-day cycle, domain of activity)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ingkel {
    Wong = 0,
    Sato = 1,
    Mina = 2,
    Manuk = 3,
    Taru = 4,
    Buku = 5,
    Uled = 6,
}

impl Ingkel {
    pub fn from_jdn(jdn: i64) -> Self {
        match (pawukon_day(jdn) / 7) % 7 {
            0 => Ingkel::Wong,
            1 => Ingkel::Sato,
            2 => Ingkel::Mina,
            3 => Ingkel::Manuk,
            4 => Ingkel::Taru,
            5 => Ingkel::Buku,
            _ => Ingkel::Uled,
        }
    }

    /// Optimized constructor that accepts precomputed pawukon_day
    pub fn from_pawukon_day(pawukon_day: u16) -> Self {
        match (pawukon_day / 7) % 7 {
            0 => Ingkel::Wong,
            1 => Ingkel::Sato,
            2 => Ingkel::Mina,
            3 => Ingkel::Manuk,
            4 => Ingkel::Taru,
            5 => Ingkel::Buku,
            _ => Ingkel::Uled,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Ingkel::Wong => "Wong",
            Ingkel::Sato => "Sato",
            Ingkel::Mina => "Mina",
            Ingkel::Manuk => "Manuk",
            Ingkel::Taru => "Taru",
            Ingkel::Buku => "Buku",
            Ingkel::Uled => "Uled",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// WATEK  (Madya = 7-cycle, Alit = 4-cycle)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatekMadya {
    Gajah = 0,
    Watu = 1,
    Buta = 2,
    Suku = 3,
    Wong = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatekAlit {
    Lintah = 0,
    Uler = 1,
    Gajah = 2,
    Lembu = 3,
}

impl WatekMadya {
    pub fn from_jdn(jdn: i64) -> Self {
        match pawukon_day(jdn) % 5 {
            0 => WatekMadya::Gajah,
            1 => WatekMadya::Watu,
            2 => WatekMadya::Buta,
            3 => WatekMadya::Suku,
            _ => WatekMadya::Wong,
        }
    }

    /// Optimized constructor that accepts precomputed pawukon_day
    pub fn from_pawukon_day(pawukon_day: u16) -> Self {
        match pawukon_day % 5 {
            0 => WatekMadya::Gajah,
            1 => WatekMadya::Watu,
            2 => WatekMadya::Buta,
            3 => WatekMadya::Suku,
            _ => WatekMadya::Wong,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            WatekMadya::Gajah => "Gajah",
            WatekMadya::Watu => "Watu",
            WatekMadya::Buta => "Buta",
            WatekMadya::Suku => "Suku",
            WatekMadya::Wong => "Wong",
        }
    }
}

impl WatekAlit {
    pub fn from_jdn(jdn: i64) -> Self {
        match pawukon_day(jdn) % 4 {
            0 => WatekAlit::Lintah,
            1 => WatekAlit::Uler,
            2 => WatekAlit::Gajah,
            _ => WatekAlit::Lembu,
        }
    }

    /// Optimized constructor that accepts precomputed pawukon_day
    pub fn from_pawukon_day(pawukon_day: u16) -> Self {
        match pawukon_day % 4 {
            0 => WatekAlit::Lintah,
            1 => WatekAlit::Uler,
            2 => WatekAlit::Gajah,
            _ => WatekAlit::Lembu,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            WatekAlit::Lintah => "Lintah",
            WatekAlit::Uler => "Uler",
            WatekAlit::Gajah => "Gajah",
            WatekAlit::Lembu => "Lembu",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// LINTANG  (35-day cycle = 5×7, intersection of Pancawara × Saptawara)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lintang {
    Gajah = 0,
    Kiriman = 1,
    Lumbung = 2,
    Kartika = 3,
    Naga = 4,
    Angsa = 5,
    Kuda = 6,
    Asu = 7,
    Kambing = 8,
    Lembu = 9,
    Mendanu = 10,
    Jong = 11,
    Begoong = 12,
    Bubu = 13,
    Tegeh = 14,
    Magelut = 15,
    Krait = 16,
    Kelapa = 17,
    Yuyu = 18,
    Pagelaran = 19,
    Uluku = 20,
    Pedati = 21,
    Perahu = 22,
    Bojog = 23,
    Udang = 24,
    Pohpohan = 25,
    Sungenge = 26,
    Bade = 27,
    Puwuh = 28,
    Garuda = 29,
    Kukus = 30,
    Kumba = 31,
    Tangis = 32,
    Sangkatikel = 33,
    Pucang = 34,
}

impl Lintang {
    pub fn from_jdn(jdn: i64) -> Self {
        let lintang_idx =
            ((Pancawara::from_jdn(jdn) as u8) * 7 + (Saptawara::from_jdn(jdn) as u8)) % 35;
        Self::from_index(lintang_idx as usize)
    }

    /// Optimized constructor that accepts precomputed Pancawara and Saptawara
    pub fn from_wewaran(panca: &Pancawara, sapta: &Saptawara) -> Self {
        let lintang_idx = (*panca as u8 * 7 + *sapta as u8) % 35;
        Self::from_index(lintang_idx as usize)
    }

    pub fn from_index(idx: usize) -> Self {
        match idx % 35 {
            0 => Lintang::Gajah,
            1 => Lintang::Kiriman,
            2 => Lintang::Lumbung,
            3 => Lintang::Kartika,
            4 => Lintang::Naga,
            5 => Lintang::Angsa,
            6 => Lintang::Kuda,
            7 => Lintang::Asu,
            8 => Lintang::Kambing,
            9 => Lintang::Lembu,
            10 => Lintang::Mendanu,
            11 => Lintang::Jong,
            12 => Lintang::Begoong,
            13 => Lintang::Bubu,
            14 => Lintang::Tegeh,
            15 => Lintang::Magelut,
            16 => Lintang::Krait,
            17 => Lintang::Kelapa,
            18 => Lintang::Yuyu,
            19 => Lintang::Pagelaran,
            20 => Lintang::Uluku,
            21 => Lintang::Pedati,
            22 => Lintang::Perahu,
            23 => Lintang::Bojog,
            24 => Lintang::Udang,
            25 => Lintang::Pohpohan,
            26 => Lintang::Sungenge,
            27 => Lintang::Bade,
            28 => Lintang::Puwuh,
            29 => Lintang::Garuda,
            30 => Lintang::Kukus,
            31 => Lintang::Kumba,
            32 => Lintang::Tangis,
            33 => Lintang::Sangkatikel,
            _ => Lintang::Pucang,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Lintang::Gajah => "Gajah",
            Lintang::Kiriman => "Kiriman",
            Lintang::Lumbung => "Lumbung",
            Lintang::Kartika => "Kartika",
            Lintang::Naga => "Naga",
            Lintang::Angsa => "Angsa",
            Lintang::Kuda => "Kuda",
            Lintang::Asu => "Asu",
            Lintang::Kambing => "Kambing",
            Lintang::Lembu => "Lembu",
            Lintang::Mendanu => "Mendanu",
            Lintang::Jong => "Jong",
            Lintang::Begoong => "Begoong",
            Lintang::Bubu => "Bubu",
            Lintang::Tegeh => "Tegeh",
            Lintang::Magelut => "Magelut",
            Lintang::Krait => "Krait",
            Lintang::Kelapa => "Kelapa",
            Lintang::Yuyu => "Yuyu",
            Lintang::Pagelaran => "Pagelaran",
            Lintang::Uluku => "Uluku",
            Lintang::Pedati => "Pedati",
            Lintang::Perahu => "Perahu",
            Lintang::Bojog => "Bojog",
            Lintang::Udang => "Udang",
            Lintang::Pohpohan => "Pohpohan",
            Lintang::Sungenge => "Sungenge",
            Lintang::Bade => "Bade",
            Lintang::Puwuh => "Puwuh",
            Lintang::Garuda => "Garuda",
            Lintang::Kukus => "Kukus",
            Lintang::Kumba => "Kumba",
            Lintang::Tangis => "Tangis",
            Lintang::Sangkatikel => "Sangkatikel",
            Lintang::Pucang => "Pucang",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PANCASUDA  (7-day cycle from Saptawara + adjustments)
// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PancaSuda {
    Lebu = 0,
    Wisesa = 1,
    Lanang = 2,
    Liku = 3,
    Menga = 4,
    Perahu = 5,
    Wisesa2 = 6,
}

impl PancaSuda {
    pub fn from_jdn(jdn: i64) -> Self {
        let panca = Pancawara::from_jdn(jdn);
        let sapta = Saptawara::from_jdn(jdn);
        Self::from_wewaran(&panca, &sapta)
    }

    /// Construct from precomputed Pancawara and Saptawara (avoids duplicate lookups).
    pub fn from_wewaran(panca: &Pancawara, sapta: &Saptawara) -> Self {
        let idx = (*panca as u8 + *sapta as u8) % 7;
        match idx {
            0 => PancaSuda::Lebu,
            1 => PancaSuda::Wisesa,
            2 => PancaSuda::Lanang,
            3 => PancaSuda::Liku,
            4 => PancaSuda::Menga,
            5 => PancaSuda::Perahu,
            _ => PancaSuda::Wisesa2,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            PancaSuda::Lebu => "Lebu",
            PancaSuda::Wisesa => "Wisesa",
            PancaSuda::Lanang => "Lanang",
            PancaSuda::Liku => "Liku",
            PancaSuda::Menga => "Menga",
            PancaSuda::Perahu => "Perahu",
            PancaSuda::Wisesa2 => "Wisesa",
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// PARARASAN  (10-day cycle)
// ─────────────────────────────────────────────────────────────────────────────
// Two naming traditions supported:
// 1. Aryana (edysantosa/sakacalendar) - canonical scholarly tradition
// 2. Sundari Bungkah (I Made Bidja OCR corpus) - regional variant
// Source: I.B. Putra Manik Aryana, Dasar Wariga + Tenung Wariga; I.B. Supartha Ardana, Pokok-Pokok Wariga (2005)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pararasan {
    WisesaSegara = 0,
    TunggakSemi = 1,
    SatriaWibhawa = 2,
    SumurSinaba = 3,
    BumiKapetak = 4,
    SatriaWirang = 5,
    SatriaSegara = 6,
    LebuKatiupAngin = 7,
}

impl Pararasan {
    pub fn from_jdn(jdn: i64) -> Self {
        let panca = Pancawara::from_jdn(jdn);
        let sapta = Saptawara::from_jdn(jdn);
        Self::from_wewaran(&panca, &sapta)
    }

    /// Construct from precomputed Pancawara and Saptawara (avoids duplicate lookups).
    pub fn from_wewaran(panca: &Pancawara, sapta: &Saptawara) -> Self {
        let idx = (*panca as u8 + *sapta as u8) % 8;
        match idx {
            0 => Pararasan::WisesaSegara,
            1 => Pararasan::TunggakSemi,
            2 => Pararasan::SatriaWibhawa,
            3 => Pararasan::SumurSinaba,
            4 => Pararasan::BumiKapetak,
            5 => Pararasan::SatriaWirang,
            6 => Pararasan::SatriaSegara,
            _ => Pararasan::LebuKatiupAngin,
        }
    }

    /// Returns the canonical Aryana (edysantosa/sakacalendar) name - default tradition
    pub fn name(&self) -> &'static str {
        match self {
            Pararasan::WisesaSegara => "Wisesa Segara",
            Pararasan::TunggakSemi => "Tunggak Semi",
            Pararasan::SatriaWibhawa => "Satria Wibhawa",
            Pararasan::SumurSinaba => "Sumur Sinaba",
            Pararasan::BumiKapetak => "Bumi Kapetak",
            Pararasan::SatriaWirang => "Satria Wirang",
            Pararasan::SatriaSegara => "Satria Segara",
            Pararasan::LebuKatiupAngin => "Lebu Katiup Angin",
        }
    }

    /// Returns the Sundari Bungkah (I Made Bidja OCR corpus) variant name
    /// Source: I Made Bidja, Kalender Bali 2026 (IBI Cabang Kab. Badung)
    pub fn name_sundari_bungkah(&self) -> &'static str {
        match self {
            Pararasan::WisesaSegara => "Wisesa Segara",  // Match
            Pararasan::TunggakSemi => "Tunggak Semi",    // Match
            Pararasan::SatriaWibhawa => "Satria Wibawa", // Spelling variant
            Pararasan::SumurSinaba => "Sumer Sinuhe",    // Different tradition
            Pararasan::BumiKapetak => "Bumi Kapetak",    // Match
            Pararasan::SatriaWirang => "Satria Wirang",  // Match
            Pararasan::SatriaSegara => "Satria Segara",  // Match
            Pararasan::LebuKatiupAngin => "Lelu Kalung Angis", // Different tradition
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// RAKAM  (5-day cycle from Pancawara + Saptawara)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rakam {
    Payu = 0,
    Caka = 1,
    Dora = 2,
    Rangdan = 3,
    Tiwa = 4,
}

impl Rakam {
    pub fn from_jdn(jdn: i64) -> Self {
        let panca = Pancawara::from_jdn(jdn);
        let sapta = Saptawara::from_jdn(jdn);
        Self::from_wewaran(&panca, &sapta)
    }

    /// Construct from precomputed Pancawara and Saptawara (avoids duplicate lookups).
    pub fn from_wewaran(panca: &Pancawara, sapta: &Saptawara) -> Self {
        let idx = (*panca as u8 + *sapta as u8) % 5;
        match idx {
            0 => Rakam::Payu,
            1 => Rakam::Caka,
            2 => Rakam::Dora,
            3 => Rakam::Rangdan,
            _ => Rakam::Tiwa,
        }
    }
    pub fn name(&self) -> &'static str {
        match self {
            Rakam::Payu => "Payu",
            Rakam::Caka => "Caka",
            Rakam::Dora => "Dora",
            Rakam::Rangdan => "Rangdan",
            Rakam::Tiwa => "Tiwa",
        }
    }
}
