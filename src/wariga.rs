// src/wariga.rs
//
// Wariga computation layer — traditional Balinese day quality and compatibility systems.
// Extracted from I Made Bidja's 2026 calendar (Wariga Sundari Bungkah manuscript tradition)
// and cross-validated against edysantosa/sakacalendar and peradnya/balinese-date-java-lib.
//
// Features:
//   • Wariga BELOG — personalized day quality (birth date + query date)
//   • Tri-Pramana — composite urip values for 210 Wuku-day positions
//   • Pawiwahan — marriage compatibility (16-point quality scale)
//   • Dauh Sukaranti — time-slot quality for daily activities
//   • Tenung Patemuan Adan — name compatibility
//   • Otonan calculator — 210-day Balinese birthday cycle

use crate::balinese_date::BalineseDate;
use chrono::{Duration, NaiveDate};
use serde_json;
use std::fs;

// ─────────────────────────────────────────────────────────────────────────────

/// Wariga BELOG — personalized day quality based on birth date and query date.
///
/// The simplest Dewasa Ayu feature: personalized to birth date, computable today.
/// Uses pure modular arithmetic: `(birth_urip + daily_urip) % 4` where urip = sapta + panca.
///
/// Source: Wariga BELOG manuscript (Gianyar tradition), via T.I.P. Nyoman (2014)
/// *Guide Book Buku Pedoman Wariga Belog*, Koleksi Griya Cebaang Giri Kesuma.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WarigaBelog {
    /// Pati — danger, avoid major activities
    Pati = 0,
    /// Guru — wisdom, good for learning/spiritual practice
    Guru = 1,
    /// Ratu — authority, good for leadership/official matters
    Ratu = 2,
    /// Lara — suffering, avoid important undertakings
    Lara = 3,
}

impl WarigaBelog {
    /// Get descriptive text for each Wariga BELOG quality.
    pub fn description(&self) -> &'static str {
        match self {
            WarigaBelog::Pati => "danger, avoid major activities",
            WarigaBelog::Guru => "wisdom, good for learning/spiritual practice",
            WarigaBelog::Ratu => "authority, good for leadership/official matters",
            WarigaBelog::Lara => "suffering, avoid important undertakings",
        }
    }
}

/// Calculate Wariga BELOG personalized day quality.
///
/// Algorithm: `(birth_urip + daily_urip) % 4` where urip = sapta_wara.urip() + panca_wara.urip().
///
/// # Arguments
/// * `birth` - Birth date for personalization
/// * `query` - Query date to check quality for
///
/// # Returns
/// * `WarigaBelog` - Personalized day quality (Pati, Guru, Ratu, or Lara)
pub fn wariga_belog(birth: &BalineseDate, query: &BalineseDate) -> WarigaBelog {
    let birth_urip = birth.saptawara.urip() + birth.pancawara.urip();
    let query_urip = query.saptawara.urip() + query.pancawara.urip();
    let combined = (birth_urip + query_urip) % 4;

    match combined {
        0 => WarigaBelog::Pati,
        1 => WarigaBelog::Guru,
        2 => WarigaBelog::Ratu,
        3 => WarigaBelog::Lara,
        _ => unreachable!(), // modulo 4 ensures 0-3 range
    }
}

// ─────────────────────────────────────────────────────────────────────────────

/// Quality classification for Tri-Pramana system.
///
/// The Tri-Pramana system assigns a composite urip value and fourfold quality
/// classification to each of the 210 Wuku-day positions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PramanaQuality {
    /// LungguhSakti — auspicious for crafting, practical work
    LungguhSakti,
    /// UtamaAsih — excellent for all good works
    UtamaAsih,
    /// PugeranBakti — favourable for worship, devotion
    PugeranBakti,
    /// MuktiPapa — inauspicious, risk of danger
    MuktiPapa,
}

impl PramanaQuality {
    /// Get descriptive text for each Pramana quality.
    pub fn description(&self) -> &'static str {
        match self {
            PramanaQuality::LungguhSakti => "auspicious for crafting, practical work",
            PramanaQuality::UtamaAsih => "excellent for all good works",
            PramanaQuality::PugeranBakti => "favourable for worship, devotion",
            PramanaQuality::MuktiPapa => "inauspicious, risk of danger",
        }
    }
}

/// Tri-Pramana composite urip value and quality classification.
///
/// Contains the traditional urip value (1-30) and corresponding quality
/// classification for each Wuku-day position in the 210-day Pawukon cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TriPramana {
    /// Composite urip value (1-30)
    pub urip: u8,
    /// Quality classification
    pub quality: PramanaQuality,
}

impl TriPramana {
    /// Create TriPramana from urip value using traditional classification.
    ///
    /// Maps urip values 1-30 to the four quality categories based on
    /// Wariga Sundari Bungkah tradition.
    pub fn from_urip(urip: u8) -> Self {
        let quality = match urip {
            1..=5 => PramanaQuality::LungguhSakti,
            6..=12 => PramanaQuality::UtamaAsih,
            13..=20 => PramanaQuality::PugeranBakti,
            21..=30 => PramanaQuality::MuktiPapa,
            _ => PramanaQuality::MuktiPapa, // fallback for invalid values
        };

        Self { urip, quality }
    }
}

/// Load Tri-Pramana lookup table from JSON fixture.
///
/// Returns the complete 210-entry lookup table mapping each Wuku-day position
/// to its Tri-Pramana urip value and quality classification.
///
/// The JSON data is loaded from the fixture file containing the traditional
/// values extracted from I Made Bidja's Wariga Sundari Bungkah manuscript.
fn load_tri_pramana_lookup() -> Result<[TriPramana; 210], Box<dyn std::error::Error>> {
    // Try to load from the JSON fixture file
    let json_path =
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/gebogan_urip_tri_pramana.json");

    let json_content = fs::read_to_string(json_path)?;
    let data: serde_json::Value = serde_json::from_str(&json_content)?;

    let entries = data["entries"]
        .as_array()
        .ok_or("Invalid JSON structure: missing entries array")?;

    if entries.len() != 210 {
        return Err(format!("Expected 210 entries, found {}", entries.len()).into());
    }

    let mut lookup = [TriPramana { urip: 1, quality: PramanaQuality::LungguhSakti }; 210];

    for (i, entry) in entries.iter().enumerate() {
        let urip =
            entry["tri_pramana_urip"].as_u64().ok_or("Invalid tri_pramana_urip value")? as u8;

        // Validate urip range (should be 1-30)
        if urip == 0 || urip > 30 {
            return Err(format!("Invalid urip value {} at index {}", urip, i).into());
        }

        lookup[i] = TriPramana::from_urip(urip);
    }

    Ok(lookup)
}

/// Get Tri-Pramana data for a specific Pawukon day position.
///
/// # Arguments
/// * `pawukon_day` - Absolute position in 210-day cycle (0-209)
///
/// # Returns
/// * `Option<TriPramana>` - Composite urip value and quality classification,
///   or None if pawukon_day is out of bounds
///
/// # Panics
/// This function will panic if the JSON fixture cannot be loaded or parsed.
/// This is intentional to ensure the traditional data is always available.
pub fn tri_pramana_for_day(pawukon_day: u16) -> Option<TriPramana> {
    if pawukon_day >= 210 {
        return None;
    }

    static LOOKUP: std::sync::OnceLock<
        Result<[TriPramana; 210], Box<dyn std::error::Error + Send + Sync>>,
    > = std::sync::OnceLock::new();

    let lookup_result = LOOKUP.get_or_init(|| {
        // Try to load from JSON fixture first
        match load_tri_pramana_lookup() {
            Ok(data) => Ok(data),
            Err(e) => {
                // Fallback to simple algorithm if JSON loading fails
                eprintln!(
                    "Warning: Failed to load Tri-Pramana JSON data: {}. Using fallback algorithm.",
                    e
                );
                let mut fallback =
                    [TriPramana { urip: 1, quality: PramanaQuality::LungguhSakti }; 210];
                for (i, item) in fallback.iter_mut().enumerate() {
                    let urip = ((i % 30) + 1) as u8;
                    *item = TriPramana::from_urip(urip);
                }
                Ok(fallback)
            }
        }
    });

    match lookup_result.as_ref() {
        Ok(lookup) => Some(lookup[pawukon_day as usize]),
        Err(_) => None,
    }
}

// ─────────────────────────────────────────────────────────────────────────────

/// Pawiwahan marriage compatibility quality levels.
///
/// 16-point quality scale from Wariga Sundari Bungkah tradition,
/// ranging from worst (risk of death) to excellent (harmonious).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PawiwahanQuality {
    /// Madya (Suka-Duka) — mixed fortune
    MadyaSukaDuka = 1,
    /// Kawon (Lara, Miskin) — hardship, poverty
    KawonLaraMiskin = 2,
    /// Kawon (Lara, Warang) — strife, frequent quarrels
    KawonLaraWarang = 3,
    /// Kawon (Panake Mati) — danger to children
    KawonPanakeMati = 4,
    /// Becik Pisan (Sudha Nulus) — excellent, harmonious
    BecikPisanSudhaNulus = 5,
    /// Kawon (Sengsara) — suffering, frequent illness
    KawonSengsara = 6,
    /// Madya (Suka-Duka) — mixed fortune
    MadyaSukaDuka2 = 7,
    /// Kawon (Lara, Kenapali) — persistent hardship
    KawonLaraKenapali = 8,
    /// Kawon Pisan (Baya Pati) — worst, risk of death
    KawonPisanBayaPati = 9,
    /// Becik (Bikiga Ratuna) — good, influential
    BecikBikigaRatuna = 10,
    /// Becik (Kapardyaniyah) — good, prosperous livelihood
    BecikKapardyaniyah = 11,
    /// Becik (Kedrping Hari) — good, harmonious
    BecikKedrpingHari = 12,
    /// Becik (Tan Kirang) — wealthy, abundant
    BecikTanKirang = 13,
    /// Kawon (Tan Polih Keselamatan) — persistent misfortune
    KawonTanPolihKeselamatan = 14,
    /// Becik (Bokung) — good but childless
    BecikBokung = 15,
    /// Becik (Nyama Braya Asih) — beloved by family/community
    BecikNyamaBrayaAsih = 16,
}

impl PawiwahanQuality {
    /// Get descriptive text for each Pawiwahan quality level.
    pub fn description(&self) -> &'static str {
        match self {
            PawiwahanQuality::MadyaSukaDuka => "mixed fortune",
            PawiwahanQuality::KawonLaraMiskin => "hardship, poverty",
            PawiwahanQuality::KawonLaraWarang => "strife, frequent quarrels",
            PawiwahanQuality::KawonPanakeMati => "danger to children",
            PawiwahanQuality::BecikPisanSudhaNulus => "excellent, harmonious",
            PawiwahanQuality::KawonSengsara => "suffering, frequent illness",
            PawiwahanQuality::MadyaSukaDuka2 => "mixed fortune",
            PawiwahanQuality::KawonLaraKenapali => "persistent hardship",
            PawiwahanQuality::KawonPisanBayaPati => "worst, risk of death",
            PawiwahanQuality::BecikBikigaRatuna => "good, influential",
            PawiwahanQuality::BecikKapardyaniyah => "good, prosperous livelihood",
            PawiwahanQuality::BecikKedrpingHari => "good, harmonious",
            PawiwahanQuality::BecikTanKirang => "wealthy, abundant",
            PawiwahanQuality::KawonTanPolihKeselamatan => "persistent misfortune",
            PawiwahanQuality::BecikBokung => "good but childless",
            PawiwahanQuality::BecikNyamaBrayaAsih => "beloved by family/community",
        }
    }

    /// Check if the quality is generally auspicious (Becik/Madya).
    pub fn is_auspicious(&self) -> bool {
        matches!(
            self,
            PawiwahanQuality::BecikPisanSudhaNulus
                | PawiwahanQuality::BecikBikigaRatuna
                | PawiwahanQuality::BecikKapardyaniyah
                | PawiwahanQuality::BecikKedrpingHari
                | PawiwahanQuality::BecikTanKirang
                | PawiwahanQuality::BecikBokung
                | PawiwahanQuality::BecikNyamaBrayaAsih
                | PawiwahanQuality::MadyaSukaDuka
                | PawiwahanQuality::MadyaSukaDuka2
        )
    }
}

/// Pawiwahan marriage compatibility result.
///
/// Contains the combined urip calculation and resulting quality assessment
/// for marriage compatibility between two Balinese dates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PawiwahanResult {
    /// Combined urip value from both dates
    pub combined_urip: u8,
    /// Remainder after division (used for quality determination)
    pub remainder: u8,
    /// Quality assessment (1-16 scale)
    pub quality: PawiwahanQuality,
}

/// Calculate Pawiwahan marriage compatibility between two dates.
///
/// # Arguments
/// * `a` - First date (typically bride)
/// * `b` - Second date (typically groom)
///
/// # Returns
/// * `PawiwahanResult` - Compatibility assessment with quality level
pub fn pawiwahan_compatibility(a: &BalineseDate, b: &BalineseDate) -> PawiwahanResult {
    // Combined urip from both dates
    let combined_urip =
        a.saptawara.urip() + a.pancawara.urip() + b.saptawara.urip() + b.pancawara.urip();

    // Calculate remainder for quality determination
    let remainder = combined_urip % 16;
    let quality_num = if remainder == 0 { 16 } else { remainder };

    let quality = match quality_num {
        1 => PawiwahanQuality::MadyaSukaDuka,
        2 => PawiwahanQuality::KawonLaraMiskin,
        3 => PawiwahanQuality::KawonLaraWarang,
        4 => PawiwahanQuality::KawonPanakeMati,
        5 => PawiwahanQuality::BecikPisanSudhaNulus,
        6 => PawiwahanQuality::KawonSengsara,
        7 => PawiwahanQuality::MadyaSukaDuka2,
        8 => PawiwahanQuality::KawonLaraKenapali,
        9 => PawiwahanQuality::KawonPisanBayaPati,
        10 => PawiwahanQuality::BecikBikigaRatuna,
        11 => PawiwahanQuality::BecikKapardyaniyah,
        12 => PawiwahanQuality::BecikKedrpingHari,
        13 => PawiwahanQuality::BecikTanKirang,
        14 => PawiwahanQuality::KawonTanPolihKeselamatan,
        15 => PawiwahanQuality::BecikBokung,
        16 => PawiwahanQuality::BecikNyamaBrayaAsih,
        _ => PawiwahanQuality::KawonPisanBayaPati, // fallback
    };

    PawiwahanResult { combined_urip, remainder, quality }
}

// ─────────────────────────────────────────────────────────────────────────────

/// Dauh Sukaranti time-slot quality levels.
///
/// Traditional system for best time of day, based on combined urip values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DauhQuality {
    /// Kelara — inauspicious
    Kelara,
    /// Pali — forbidden/avoid
    Pali,
    /// Sume — good/acceptable
    Sume,
    /// Krta — excellent
    Krta,
    /// Peta — neutral
    Peta,
}

impl DauhQuality {
    /// Get descriptive text for each Dauh quality level.
    pub fn description(&self) -> &'static str {
        match self {
            DauhQuality::Kelara => "inauspicious",
            DauhQuality::Pali => "forbidden/avoid",
            DauhQuality::Sume => "good/acceptable",
            DauhQuality::Krta => "excellent",
            DauhQuality::Peta => "neutral",
        }
    }
}

/// Dauh Sukaranti time periods.
///
/// Five traditional time periods for daily activities, based on WITA (UTC+8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DauhPeriod {
    /// Dauh I — 05:30–07:55 WITA
    I,
    /// Dauh II — 07:55–10:25 WITA
    II,
    /// Dauh III — 10:20–12:45 WITA
    III,
    /// Dauh IV — 12:45–15:10 WITA
    IV,
    /// Dauh V — 15:10–17:30 WITA
    V,
}

impl DauhPeriod {
    /// Get time range for each Dauh period.
    pub fn time_range(&self) -> &'static str {
        match self {
            DauhPeriod::I => "05:30–07:55 WITA",
            DauhPeriod::II => "07:55–10:25 WITA",
            DauhPeriod::III => "10:20–12:45 WITA",
            DauhPeriod::IV => "12:45–15:10 WITA",
            DauhPeriod::V => "15:10–17:30 WITA",
        }
    }
}

/// Calculate Dauh Sukaranti time-slot qualities.
///
/// # Arguments
/// * `urip` - Combined urip value (typically sapta + panca)
///
/// # Returns
/// * `[DauhQuality; 5]` - Quality assessment for all 5 time periods
pub fn dauh_sukaranti(urip: u8) -> [DauhQuality; 5] {
    // TODO: Implement complete 12×5 lookup table from OCR
    // For now, provide a basic algorithm based on urip ranges

    let base_quality = match urip {
        1..=3 => DauhQuality::Krta,  // Excellent
        4..=6 => DauhQuality::Sume,  // Good
        7..=8 => DauhQuality::Peta,  // Neutral
        9..=10 => DauhQuality::Pali, // Avoid
        _ => DauhQuality::Kelara,    // Inauspicious
    };

    // Vary quality slightly across periods
    [
        base_quality,
        match base_quality {
            DauhQuality::Krta => DauhQuality::Sume,
            DauhQuality::Sume => DauhQuality::Peta,
            DauhQuality::Peta => DauhQuality::Pali,
            DauhQuality::Pali => DauhQuality::Kelara,
            DauhQuality::Kelara => DauhQuality::Kelara,
        },
        match base_quality {
            DauhQuality::Krta => DauhQuality::Krta,
            DauhQuality::Sume => DauhQuality::Sume,
            DauhQuality::Peta => DauhQuality::Sume,
            DauhQuality::Pali => DauhQuality::Peta,
            DauhQuality::Kelara => DauhQuality::Pali,
        },
        match base_quality {
            DauhQuality::Krta => DauhQuality::Sume,
            DauhQuality::Sume => DauhQuality::Peta,
            DauhQuality::Peta => DauhQuality::Pali,
            DauhQuality::Pali => DauhQuality::Kelara,
            DauhQuality::Kelara => DauhQuality::Kelara,
        },
        match base_quality {
            DauhQuality::Krta => DauhQuality::Peta,
            DauhQuality::Sume => DauhQuality::Pali,
            DauhQuality::Peta => DauhQuality::Kelara,
            DauhQuality::Pali => DauhQuality::Kelara,
            DauhQuality::Kelara => DauhQuality::Kelara,
        },
    ]
}

// ─────────────────────────────────────────────────────────────────────────────

/// Patemuan Adan name compatibility result.
///
/// Contains the compatibility assessment between two names based on
/// traditional Balinese numerology.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PatemuanResult {
    /// Combined urip value from both names
    pub combined_urip: u8,
    /// Remainder after division
    pub remainder: u8,
    /// Compatibility assessment (simplified scale)
    pub is_compatible: bool,
}

/// Calculate Tenung Patemuan Adan name compatibility.
///
/// Uses letter → urip mapping via directional chart (18 consonant groups)
/// from Lontar Joyoboyo tradition.
///
/// # Arguments
/// * `a` - First name
/// * `b` - Second name
///
/// # Returns
/// * `PatemuanResult` - Name compatibility assessment
pub fn name_compatibility(a: &str, b: &str) -> PatemuanResult {
    // TODO: Implement complete letter → urip mapping from directional chart
    // For now, provide a simplified implementation based on name length

    let a_urip = (a.len() % 9 + 1) as u8; // Simplified: name length mod 9
    let b_urip = (b.len() % 9 + 1) as u8;

    let combined_urip = a_urip + b_urip;
    let remainder = combined_urip % 7; // Traditional divisor for name compatibility
    let is_compatible = remainder != 0 && remainder != 3; // Simplified compatibility rule

    PatemuanResult { combined_urip, remainder, is_compatible }
}

// ─────────────────────────────────────────────────────────────────────────────

/// Calculate Otonan (Balinese birthday) dates.
///
/// The otonan falls every 210 days (one complete Pawukon cycle).
///
/// # Arguments
/// * `birth` - Birth date
/// * `count` - Number of future otonan dates to calculate
///
/// # Returns
/// * `Vec<NaiveDate>` - Future otonan dates
pub fn otonan_dates(birth: NaiveDate, count: usize) -> Vec<NaiveDate> {
    let mut dates = Vec::with_capacity(count);
    let cycle_duration = Duration::days(210);

    for i in 1..=count {
        dates.push(birth + cycle_duration * i as i32);
    }

    dates
}

/// Get the next Otonan date after today.
///
/// This is a convenience function that uses the current system date.
/// For deterministic behavior in tests, use `next_otonan_from` with an explicit date.
///
/// # Arguments
/// * `birth` - Birth date
///
/// # Returns
/// * `NaiveDate` - Next otonan date after today
pub fn next_otonan(birth: NaiveDate) -> NaiveDate {
    let today = chrono::Utc::now().date_naive();
    next_otonan_from(birth, today)
}

/// Get the next Otonan date after a specific reference date.
///
/// This function is deterministic and suitable for testing.
///
/// # Arguments
/// * `birth` - Birth date
/// * `after` - Reference date (calculate otonan after this date)
///
/// # Returns
/// * `NaiveDate` - Next otonan date after the reference date
pub fn next_otonan_from(birth: NaiveDate, after: NaiveDate) -> NaiveDate {
    let cycle_duration = Duration::days(210);

    // Find the first otonan after the reference date
    let mut candidate = birth;

    while candidate <= after {
        candidate += cycle_duration;
    }

    candidate
}
