// src/boundary.rs
//
// DayBoundary — controls how the Balinese day boundary (sunrise) is resolved
// when converting a wall-clock instant to a Balinese calendar date.

/// Controls how the Balinese day boundary (sunrise) is resolved
/// when converting a wall-clock instant to a Balinese calendar date.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DayBoundary {
    /// Raw Gregorian midnight. Legacy behaviour; useful for tests and
    /// date-only inputs that are already sunrise-adjusted by the caller.
    Midnight,

    /// Shift the UTC instant forward by `hour` hours before taking
    /// the calendar date. `FixedSunrise(6)` is the crate default:
    ///   Bali WITA = UTC+8, sunrise ≈ 06:00 → effective offset = UTC+2.
    /// Valid range: 0–23.
    FixedSunrise(u8),

    /// Compute the actual astronomical sunrise for the given coordinates
    /// and treat that as the day boundary. Requires the `sunrise` feature flag.
    #[cfg(feature = "astronomical")]
    Astronomical { lat: f64, lon: f64 },
}

impl Default for DayBoundary {
    fn default() -> Self {
        Self::FixedSunrise(6)
    }
}
