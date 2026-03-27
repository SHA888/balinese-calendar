#![allow(clippy::manual_range_contains, clippy::manual_rem_euclid, missing_docs)]
// src/lib.rs
//
// balinese-date — native Rust implementation of the Balinese Saka Calendar.
//
// The first Rust crate for Kalender Bali. Covers:
//   • Pawukon (210-day cycle): 30 Wuku × 7 days
//   • Pawewaran: all 10 concurrent week cycles (Eka through Dasa Wara)
//   • Sasih: lunar months, Penanggal/Pangelong/Purnama/Tilem, Ngunaratri
//   • Nampih Sasih: intercalary month detection (≥ 2003 / Saka 1925)
//   • Saka year calculation
//   • Paringkelan: Jejepan, Ingkel, Watek, Lintang, PancaSuda, Pararasan, Rakam
//   • Rahinan: holy day detection (Galungan, Kuningan, Saraswati, etc.)
//   • FlatRecord: columnar data serialization for Arrow, Parquet, and other formats
//
// Algorithm basis:
//   - Ardhana, I.B.S. (2005). "Pokok-Pokok Wariga". Surabaya: Paramita.
//   - babadbali.com (Yayasan Bali Galang) wewaran & paringkelan algorithms.
//   - Pendit, N.S. (2001). "Nyepi: kebangkitan, toleransi, dan kerukunan."
//   - peradnya/balinese-date-js-lib & java-lib (Apache-2.0) — behaviour reference.
//   - Wikipedia: Balinese Saka Calendar (intercalary month rules)
//
// Validation target: kalenderbali.org (I Wayan Nuarsa, Universitas Udayana)
//
// ⚠ PRODUCTION NOTE (Nampih Sasih):
//   Intercalary month placement is computed algorithmically from a 19-year
//   Metonic-like cycle (saka_year % 19), matching the peradnya reference
//   implementation. The algorithmic estimate may differ from the PHDI
//   (Parisada Hindu Dharma Indonesia) declaration by 1 sasih in rare years.
#![forbid(unsafe_code)]

mod balinese_date;
pub mod boundary;
pub mod error;
pub mod paringkelan;
pub mod pawukon;
pub mod rahinan;
pub mod sasih;
pub mod utils;
pub mod wewaran;

#[cfg(feature = "wasm")]
pub mod wasm;

// ── Re-exports: primary API ───────────────────────────────────────────────────

pub use balinese_date::{BalineseDate, FlatRecord};
pub use boundary::DayBoundary;
pub use error::BalineseDateError;

// ── Re-exports: enums (for pattern matching in consumer crates) ───────────────

pub use paringkelan::{
    Ingkel, Jejepan, Lintang, PancaSuda, Pararasan, Rakam, WatekAlit, WatekMadya,
};
pub use pawukon::Wuku;
pub use rahinan::Rahinan;
pub use sasih::{Sasih, SasihDayInfo};
pub use wewaran::{
    Astawara, Caturwara, Dasawara, Dwiwara, Ekawara, Pancawara, Sadwara, Sangawara, Saptawara,
    Triwara,
};

// ── Version ───────────────────────────────────────────────────────────────────

/// Library version, mirrors `Cargo.toml`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
