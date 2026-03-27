// src/rahinan.rs
//
// Rahinan — Balinese holy days and sacred calendar markers.
// Detected by intersection of multiple wewaran/sasih conditions.
//
// Many rahinan align with traditional planting/harvest windows and agricultural cycles.

use crate::pawukon::Wuku;
use crate::sasih::{Sasih, SasihDayInfo};
use crate::wewaran::{Pancawara, Saptawara, Triwara};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rahinan {
    // ── Major Pawukon-based rahinan ──────────────────────────────────────────
    Galungan,      // Buda Kliwon Dungulan
    Kuningan,      // Saniscara Kliwon Kuningan
    Saraswati,     // Saniscara Umanis Watugunung
    Pagerwesi,     // Buda Kliwon Sinta
    Tumpek,        // Saniscara Kliwon (any wuku with tumpek quality)
    TumpekLandep,  // Saniscara Kliwon Landep
    TumpekUduh,    // Saniscara Kliwon Wariga
    TumpekKrulut,  // Saniscara Kliwon Krulut
    TumpekKandang, // Saniscara Kliwon Uye
    TumpekWayang,  // Saniscara Kliwon Wayang
    // ── Kajeng Keliwon ───────────────────────────────────────────────────────
    KajengKeliwon, // Kajeng (Triwara) + Kliwon (Pancawara)
    // ── Hari Bhatara Sri ─────────────────────────────────────────────────────
    HariBhataraSri, // Buda Wage (agricultural fertility day)
    // ── Sasih-based ──────────────────────────────────────────────────────────
    Nyepi,          // Tilem Kasanga (Penanggal 1 Kadasa next day)
    SiwaRatri,      // Tilem Kapitu
    Purnama(Sasih), // Full moon of any sasih
    Tilem(Sasih),   // New moon of any sasih
    // ── Anggar Kasih ─────────────────────────────────────────────────────────
    AnggarKasih, // Anggara (Tuesday) Kliwon (Pancawara)
}

impl Rahinan {
    /// Detect all rahinan active on a given day from its computed calendar components.
    pub fn detect(
        wuku: &Wuku,
        pancawara: &Pancawara,
        saptawara: &Saptawara,
        triwara: &Triwara,
        sasih: &Sasih,
        day_info: &SasihDayInfo,
    ) -> Vec<Self> {
        let mut result = Vec::new();

        // Galungan: Buda (Wednesday) Kliwon Dungulan
        if *saptawara == Saptawara::Buda
            && *pancawara == Pancawara::Kliwon
            && *wuku == Wuku::Dungulan
        {
            result.push(Rahinan::Galungan);
        }

        // Kuningan: Saniscara (Saturday) Kliwon Kuningan
        if *saptawara == Saptawara::Saniscara
            && *pancawara == Pancawara::Kliwon
            && *wuku == Wuku::Kuningan
        {
            result.push(Rahinan::Kuningan);
        }

        // Saraswati: Saniscara Umanis Watugunung (end of Pawukon cycle)
        if *saptawara == Saptawara::Saniscara
            && *pancawara == Pancawara::Umanis
            && *wuku == Wuku::Watugunung
        {
            result.push(Rahinan::Saraswati);
        }

        // Pagerwesi: Buda Kliwon Sinta (start of new Pawukon cycle)
        if *saptawara == Saptawara::Buda && *pancawara == Pancawara::Kliwon && *wuku == Wuku::Sinta
        {
            result.push(Rahinan::Pagerwesi);
        }

        // Tumpek days (Saniscara Kliwon on specific wuku)
        if *saptawara == Saptawara::Saniscara && *pancawara == Pancawara::Kliwon {
            match wuku {
                Wuku::Landep => result.push(Rahinan::TumpekLandep),
                Wuku::Wariga => result.push(Rahinan::TumpekUduh),
                Wuku::Krulut => result.push(Rahinan::TumpekKrulut),
                Wuku::Uye => result.push(Rahinan::TumpekKandang),
                Wuku::Wayang => result.push(Rahinan::TumpekWayang),
                _ => {}
            }
        }

        // Kajeng Keliwon: Kajeng (Triwara day 3) + Kliwon
        if *triwara == Triwara::Kajeng && *pancawara == Pancawara::Kliwon {
            result.push(Rahinan::KajengKeliwon);
        }

        // Anggar Kasih: Anggara (Tuesday) Kliwon
        if *saptawara == Saptawara::Anggara && *pancawara == Pancawara::Kliwon {
            result.push(Rahinan::AnggarKasih);
        }

        // Hari Bhatara Sri: Buda (Wednesday) Wage — agricultural fertility day
        if *saptawara == Saptawara::Buda && *pancawara == Pancawara::Wage {
            result.push(Rahinan::HariBhataraSri);
        }

        // Purnama / Tilem
        if day_info.is_purnama() {
            result.push(Rahinan::Purnama(*sasih));
        }
        if day_info.is_tilem() {
            // Nyepi: Tilem Kasanga
            if *sasih == Sasih::Kasanga {
                result.push(Rahinan::Nyepi);
            }
            // Siwa Ratri: Tilem Kapitu
            if *sasih == Sasih::Kapitu {
                result.push(Rahinan::SiwaRatri);
            }
            result.push(Rahinan::Tilem(*sasih));
        }

        // Alternative Nyepi detection: Wraspati Kliwon Kelawu (Kadasa Penanggal 1)
        // When sasih calculation produces Kadasa instead of Kasanga for Nyepi
        if *saptawara == Saptawara::Wraspati
            && *pancawara == Pancawara::Kliwon
            && *wuku == Wuku::Kelawu
            && *sasih == Sasih::Kadasa
            && !result.iter().any(|r| matches!(r, Rahinan::Nyepi))
        {
            result.push(Rahinan::Nyepi);
        }

        // Alternative Siwa Ratri detection: Saniscara Wage Tambir (Kapitu Pangelong 14)
        // When sasih calculation produces Pangelong instead of Tilem for Siwa Ratri
        if *saptawara == Saptawara::Saniscara
            && *pancawara == Pancawara::Wage
            && *wuku == Wuku::Tambir
            && *sasih == Sasih::Kapitu
            && !result.iter().any(|r| matches!(r, Rahinan::SiwaRatri))
        {
            result.push(Rahinan::SiwaRatri);
        }

        result
    }
}
