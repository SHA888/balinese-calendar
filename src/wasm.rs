//! WASM bindings for JavaScript interop
//!
//! This module provides JavaScript-friendly wrappers around the core calendar functionality.
//! Enable with the `wasm` feature flag.

use crate::BalineseDate;
use crate::boundary::DayBoundary;
use chrono::NaiveDate;
use wasm_bindgen::prelude::*;

#[cfg(feature = "astronomical")]
use chrono::Utc;

/// JavaScript-friendly wrapper for BalineseDate
#[wasm_bindgen]
pub struct WasmBalineseDate {
    date: BalineseDate,
}

#[wasm_bindgen]
impl WasmBalineseDate {
    /// Create a Balinese date from Gregorian year, month, day
    /// Returns null if the date is invalid or out of range
    #[wasm_bindgen(constructor)]
    pub fn new(year: i32, month: u32, day: u32) -> Result<WasmBalineseDate, JsValue> {
        BalineseDate::from_ymd(year, month, day)
            .map(|date| WasmBalineseDate { date })
            .map_err(|e| JsValue::from_str(&format!("{e}")))
    }

    /// Get today's Balinese date
    #[wasm_bindgen(js_name = today)]
    pub fn today() -> Result<WasmBalineseDate, JsValue> {
        BalineseDate::today()
            .map(|date| WasmBalineseDate { date })
            .map_err(|e| JsValue::from_str(&format!("{e}")))
    }

    /// Get Gregorian year
    #[wasm_bindgen(getter)]
    pub fn gregorian_year(&self) -> i32 {
        self.date.gregorian_year
    }

    /// Get Gregorian month (1-12)
    #[wasm_bindgen(getter)]
    pub fn gregorian_month(&self) -> u32 {
        self.date.gregorian_month
    }

    /// Get Gregorian day (1-31)
    #[wasm_bindgen(getter)]
    pub fn gregorian_day(&self) -> u32 {
        self.date.gregorian_day
    }

    /// Get Saka year
    #[wasm_bindgen(getter)]
    pub fn saka_year(&self) -> i32 {
        self.date.saka_year
    }

    /// Get Sasih (lunar month) name
    #[wasm_bindgen(getter)]
    pub fn sasih_name(&self) -> String {
        self.date.sasih.name().to_string()
    }

    /// Get Wuku (week) name
    #[wasm_bindgen(getter)]
    pub fn wuku_name(&self) -> String {
        self.date.wuku.name().to_string()
    }

    /// Get Saptawara (day of week) name
    #[wasm_bindgen(getter)]
    pub fn saptawara_name(&self) -> String {
        self.date.saptawara.name().to_string()
    }

    /// Get Pancawara (5-day cycle) name
    #[wasm_bindgen(getter)]
    pub fn pancawara_name(&self) -> String {
        self.date.pancawara.name().to_string()
    }

    /// Get formatted Balinese string representation
    /// Example: "Kamis Umanis Sungsang, Penanggal 15 Kasanga 1948"
    #[wasm_bindgen]
    pub fn to_balinese_string(&self) -> String {
        self.date.to_balinese_string()
    }

    /// Get formatted ISO string representation
    /// Example: "2026-03-26"
    #[wasm_bindgen]
    pub fn to_iso_string(&self) -> String {
        let year = self.date.gregorian_year;
        let month = self.date.gregorian_month;
        let day = self.date.gregorian_day;
        format!("{year:04}-{month:02}-{day:02}")
    }

    /// Get list of rahinan (holy days) for this date
    #[wasm_bindgen]
    pub fn rahinan(&self) -> Vec<JsValue> {
        self.date.rahinan.iter().map(|r| JsValue::from_str(r.name())).collect()
    }

    /// Check if this date is a pancaroba (transitional season)
    #[wasm_bindgen]
    pub fn is_pancaroba(&self) -> bool {
        self.date.sasih.is_pancaroba()
    }

    /// Get JSON representation (requires serde feature)
    #[wasm_bindgen]
    pub fn to_json(&self) -> Result<JsValue, JsValue> {
        #[cfg(feature = "serde")]
        {
            serde_wasm_bindgen::to_value(&self.date).map_err(|e| JsValue::from_str(&format!("{e}")))
        }
        #[cfg(not(feature = "serde"))]
        {
            Err(JsValue::from_str("serde feature not enabled"))
        }
    }

    /// Create a Balinese date with custom fixed sunrise boundary
    /// hour: 0-23 (Bali sunrise offset in hours, 6 = default)
    #[wasm_bindgen]
    pub fn with_fixed_sunrise(
        year: i32,
        month: u32,
        day: u32,
        hour: u8,
    ) -> Result<WasmBalineseDate, JsValue> {
        if hour > 23 {
            return Err(JsValue::from_str("Fixed sunrise hour must be 0-23"));
        }

        let boundary = DayBoundary::FixedSunrise(hour);
        // Convert Gregorian date to UTC datetime at midnight, then apply boundary
        let naive_date = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| JsValue::from_str("Invalid Gregorian date"))?;
        let utc_datetime = naive_date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| JsValue::from_str("Invalid time"))?
            .and_utc();

        BalineseDate::from_utc_datetime_with_boundary(utc_datetime, &boundary)
            .map(|date| WasmBalineseDate { date })
            .map_err(|e| JsValue::from_str(&format!("{e}")))
    }

    /// Create a Balinese date with astronomical sunrise boundary
    /// Requires both wasm and astronomical features
    /// lat: latitude in degrees (-90 to 90)
    /// lon: longitude in degrees (-180 to 180)
    #[cfg(all(feature = "astronomical", feature = "wasm"))]
    #[wasm_bindgen]
    pub fn with_astronomical_sunrise(
        year: i32,
        month: u32,
        day: u32,
        lat: f64,
        lon: f64,
    ) -> Result<WasmBalineseDate, JsValue> {
        if lat < -90.0 || lat > 90.0 {
            return Err(JsValue::from_str("Latitude must be between -90 and 90"));
        }
        if lon < -180.0 || lon > 180.0 {
            return Err(JsValue::from_str("Longitude must be between -180 and 180"));
        }

        let boundary = DayBoundary::Astronomical { lat, lon };
        // Convert Gregorian date to UTC datetime at midnight, then apply boundary
        let naive_date = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| JsValue::from_str("Invalid Gregorian date"))?;
        let utc_datetime = naive_date
            .and_hms_opt(0, 0, 0)
            .ok_or_else(|| JsValue::from_str("Invalid time"))?
            .and_utc();

        BalineseDate::from_utc_datetime_with_boundary(utc_datetime, &boundary)
            .map(|date| WasmBalineseDate { date })
            .map_err(|e| JsValue::from_str(&format!("{e}")))
    }

    /// Get today's Balinese date with astronomical sunrise
    /// Uses Bali centroid coordinates by default
    #[cfg(all(feature = "astronomical", feature = "wasm"))]
    #[wasm_bindgen]
    pub fn today_astronomical() -> Result<WasmBalineseDate, JsValue> {
        // Bali centroid: lat -8.3405, lon 115.0920
        let boundary = DayBoundary::Astronomical { lat: -8.3405, lon: 115.0920 };
        let utc_now = Utc::now();
        BalineseDate::from_utc_datetime_with_boundary(utc_now, &boundary)
            .map(|date| WasmBalineseDate { date })
            .map_err(|e| JsValue::from_str(&format!("{e}")))
    }
}

/// Get today's Balinese date
#[wasm_bindgen]
pub fn today() -> Result<WasmBalineseDate, JsValue> {
    BalineseDate::today()
        .map(|date| WasmBalineseDate { date })
        .map_err(|e| JsValue::from_str(&format!("{e}")))
}

/// Create a Balinese date from Gregorian year, month, day
#[wasm_bindgen]
pub fn from_ymd(year: i32, month: u32, day: u32) -> Result<WasmBalineseDate, JsValue> {
    BalineseDate::from_ymd(year, month, day)
        .map(|date| WasmBalineseDate { date })
        .map_err(|e| JsValue::from_str(&format!("{e}")))
}

/// Helper to convert Gregorian date to UTC datetime at midnight
fn gregorian_to_utc_midnight(
    year: i32,
    month: u32,
    day: u32,
) -> Result<chrono::DateTime<chrono::Utc>, JsValue> {
    let naive_date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| JsValue::from_str("Invalid Gregorian date"))?;
    Ok(naive_date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| JsValue::from_str("Invalid time"))?
        .and_utc())
}

/// Create a Balinese date with custom fixed sunrise boundary
#[wasm_bindgen]
pub fn from_ymd_fixed_sunrise(
    year: i32,
    month: u32,
    day: u32,
    hour: u8,
) -> Result<WasmBalineseDate, JsValue> {
    if hour > 23 {
        return Err(JsValue::from_str("Fixed sunrise hour must be 0-23"));
    }

    let boundary = DayBoundary::FixedSunrise(hour);
    let utc_datetime = gregorian_to_utc_midnight(year, month, day)?;

    BalineseDate::from_utc_datetime_with_boundary(utc_datetime, &boundary)
        .map(|date| WasmBalineseDate { date })
        .map_err(|e| JsValue::from_str(&format!("{e}")))
}

/// Create a Balinese date with astronomical sunrise boundary
/// Requires both wasm and astronomical features
#[cfg(all(feature = "astronomical", feature = "wasm"))]
#[wasm_bindgen]
pub fn from_ymd_astronomical(
    year: i32,
    month: u32,
    day: u32,
    lat: f64,
    lon: f64,
) -> Result<WasmBalineseDate, JsValue> {
    if lat < -90.0 || lat > 90.0 {
        return Err(JsValue::from_str("Latitude must be between -90 and 90"));
    }
    if lon < -180.0 || lon > 180.0 {
        return Err(JsValue::from_str("Longitude must be between -180 and 180"));
    }

    let boundary = DayBoundary::Astronomical { lat, lon };
    let utc_datetime = gregorian_to_utc_midnight(year, month, day)?;

    BalineseDate::from_utc_datetime_with_boundary(utc_datetime, &boundary)
        .map(|date| WasmBalineseDate { date })
        .map_err(|e| JsValue::from_str(&format!("{e}")))
}

/// Get today's Balinese date with astronomical sunrise
/// Uses Bali centroid coordinates by default
#[cfg(all(feature = "astronomical", feature = "wasm"))]
#[wasm_bindgen]
pub fn today_astronomical() -> Result<WasmBalineseDate, JsValue> {
    // Bali centroid: lat -8.3405, lon 115.0920
    let boundary = DayBoundary::Astronomical { lat: -8.3405, lon: 115.0920 };
    let utc_now = Utc::now();
    BalineseDate::from_utc_datetime_with_boundary(utc_now, &boundary)
        .map(|date| WasmBalineseDate { date })
        .map_err(|e| JsValue::from_str(&format!("{e}")))
}
