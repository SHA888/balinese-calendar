//! WASM bindings for JavaScript interop
//!
//! This module provides JavaScript-friendly wrappers around the core calendar functionality.
//! Enable with the `wasm` feature flag.

use crate::BalineseDate;
use wasm_bindgen::prelude::*;

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
        self.date.rahinan.iter().map(|r| JsValue::from_str(&format!("{r:?}"))).collect()
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
