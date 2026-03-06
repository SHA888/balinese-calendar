#![allow(
    clippy::manual_range_contains,
    clippy::manual_rem_euclid,
    clippy::uninlined_format_args,
    missing_docs
)]

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
//   Intercalary month placement is declared annually by PHDI (Parisada Hindu
//   Dharma Indonesia). The built-in list covers known years; verify and extend
//   the `NAMPIH_YEARS` array in `sasih.rs` each year.

#![forbid(unsafe_code)]

pub mod error;
pub mod utils;
pub mod wewaran;
pub mod pawukon;
pub mod sasih;
pub mod paringkelan;
pub mod rahinan;
mod balinese_date;

// ── Re-exports: primary API ───────────────────────────────────────────────────

pub use balinese_date::{BalineseDate, FlatRecord};
pub use error::BalineseDateError;

// ── Re-exports: enums (for pattern matching in consumer crates) ───────────────

pub use wewaran::{
    Ekawara, Dwiwara, Triwara, Caturwara, Pancawara,
    Sadwara, Saptawara, Astawara, Sangawara, Dasawara,
};
pub use pawukon::Wuku;
pub use sasih::{Sasih, SasihDayInfo};
pub use paringkelan::{
    Jejepan, Ingkel, WatekMadya, WatekAlit,
    Lintang, PancaSuda, Pararasan, Rakam,
};
pub use rahinan::Rahinan;

// ── Version ───────────────────────────────────────────────────────────────────

/// Library version, mirrors `Cargo.toml`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
