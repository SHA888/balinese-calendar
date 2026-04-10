<div align="center">

# Balinese Calendar 🌺

**The first native Rust implementation of the Balinese Saka Calendar**

[![Crates.io](https://img.shields.io/crates/v/balinese-calendar)](https://crates.io/crates/balinese-calendar)
[![docs.rs](https://img.shields.io/docsrs/balinese-calendar)](https://docs.rs/balinese-calendar)
[![License](https://img.shields.io/crates/l/balinese-calendar)](https://github.com/SHA888/balinese-calendar/blob/main/LICENSE-MIT)
[![MSRV](https://img.shields.io/badge/MSRV-1.86.0-blue)](https://github.com/SHA888/balinese-calendar)
[![Downloads](https://img.shields.io/crates/d/balinese-calendar)](https://crates.io/crates/balinese-calendar)

</div>

Computes Pawukon (210-day cycle), Wewaran (multi-cycle day names), Sasih
(lunar months), Saka year, Rahinan (ceremony dates), and traditional Wariga
classification systems — all from a single Gregorian date input.

## 📚 Documentation

Complete documentation is available at **[sha888.github.io/balinese-calendar](https://sha888.github.io/balinese-calendar/)**

- [📖 Getting Started Guide](https://sha888.github.io/balinese-calendar/guide/getting-started)
- [🔧 API Reference](https://sha888.github.io/balinese-calendar/api/balinese-date)
- [📚 Concepts](https://sha888.github.io/balinese-calendar/concepts/calendar-systems)
- [💡 Examples](https://sha888.github.io/balinese-calendar/guide/basic-usage)

## Quick Start

```rust
use balinese_calendar::BalineseDate;

let today = BalineseDate::from_ymd(2026, 3, 22)?;

println!("{}", today.to_balinese_string());
// Redite Pon Dukut, Sasih Kadasa, Saka 1948

println!("Wuku: {} (urip {})", today.wuku.name(), today.wuku.urip());
println!("Weton: {} {}", today.saptawara.name(), today.pancawara.name());
println!("Combined urip: {}", today.saptawara.urip() + today.pancawara.urip());

for r in &today.rahinan {
    println!("Ceremony: {r}");
}
```

## Features

- **Pawukon (30 Wuku × 210-day cycle)**
- **10 Wewaran cycles** (Eka through Dasa)
- **Sasih (lunar month)** with Nampih detection
- **Saka year** (incl. Nyepi boundary)
- **Rahinan detection** (Galungan, Kuningan, Saraswati, etc.)
- **Urip computation** (Sapta + Panca Wara)
- **Ingkel, Jejepan, Watek, Lintang**
- **PancaSuda / Pararasan / Rakam**
- **Ngunaratri** (intercalary tithi)
- **DayBoundary** (sunrise-aware date)
- **WebAssembly (WASM) support** for browser environments
- **Astronomical sunrise** using real astronomical calculations

---

## Data Sources

This crate's accuracy is grounded in physical Balinese calendar sources, not inference from other software.

**Primary Source:** I Made Bidja Alm. / I Md Agus Putra Wijaya, *Kalender Bali 2026*, published by IBI Cabang Kab. Badung. Compiled from 50+ lontar Wariga manuscripts and 13 Kawi/Sanskrit/Balinese dictionaries.

**Cross-validated against:**
- [kalenderbali.org](https://www.kalenderbali.org) (I Wayan Nuarsa, Universitas Udayana)
- [basabali.org](https://dictionary.basabali.org) (BASAbali Wiki)
- [BPNB Bali](https://kebudayaan.kemdikbud.go.id/bpnbbali) (Balai Pelestarian Nilai Budaya)
- [edysantosa/sakacalendar](https://github.com/edysantosa/sakacalendar) (Java, LGPL-2.1)
- [peradnya/balinese-date-java-lib](https://github.com/peradnya/balinese-date-java-lib) (Java, Apache-2.0)

**Validation Results (v0.1.2)**
- 365/365 day-of-week matches against kalenderbali.org
- 30/30 Wuku names verified
- 12/12 Sasih transitions verified
- ✅ **Academically validated pancaroba implementation** (Sasih Kalima–Kanem, dry→wet transition)
- 210/210 Gebogan Urip Tri-Pramana entries extracted
- Zero mismatches in Pawukon cycle integrity (tested across 1969–2027)

---

## Dewasa Ayu Research

Determining "good days" (*dewasa ayu*) is the primary reason people consult a Balinese calendar. Our approach is evidence-based.

**Candana et al. (2021)** compared three fuzzy inference methods for Dewasa Pawiwahan (wedding day selection) against a Wariga expert's 16-date ground truth over 731 days (2020–2021):

| Method | Precision | Recall | F-1 Score |
|---|---|---|---|
| Tsukamoto | 3.70% | 6.25% | 4.65% |
| Mamdani | 4.76% | 6.25% | 5.41% |
| **Sugeno** | **92.31%** | **75.00%** | **82.76%** |

Sugeno correctly identified 12 of 16 expert-chosen days with only 1 false positive. This crate will implement Sugeno inference for v0.3.0.

The core Wariga rule governing variable priority (*Alahaning Dewasa*):
```
Wewaran < Wuku < Penanggal < Sasih < Dauh
```

Key finding: the expert exclusively selects Buddha (Wednesday) and Sukra (Friday) for score-80 days, never Redite (Sunday) or Saniscara (Saturday) — despite Saniscara having the highest sapta urip (9). Day quality is tradition-assigned, not urip-derived.

See `references/EXTRACTED_ALGORITHMS.md` for the complete 77-date validation dataset and `references/BIBLIOGRAPHY.md` for 99 scientific references.

## Naming Conventions

The Balinese calendar has multiple manuscript traditions. Where authoritative
sources disagree on names, this crate documents both:

| This crate (default) | Wariga Sundari Bungkah (Bidja) | Source |
|---|---|---|
| Sumur Sinaba | Sumer Sinuhe | PancaSuda #4 |
| Lebu Katiup Angin | Lelu Kalung Angis | PancaSuda #7 |
| Satria Wibhawa | Satria Wibawa | PancaSuda #3 |

Default names follow I.B. Putra Manik Aryana (*Dasar Wariga*). The Bidja
variants are available via `PancaSuda::name_sundari_bungkah()`.

## Installation

Add to `Cargo.toml`:
```toml
[dependencies]
balinese-calendar = "0.1.3"
```

**Optional Features:**
```toml
balinese-calendar = { version = "0.1.3", features = ["serde"] }
balinese-calendar = { version = "0.1.3", features = ["astronomical"] }
balinese-calendar = { version = "0.1.3", features = ["wasm"] }
balinese-calendar = { version = "0.1.3", features = ["serde", "astronomical", "wasm"] }
```

Available features:
- `serde` - Enable serialization/deserialization support
- `astronomical` - Enable astronomical sunrise calculations
- `wasm` - Enable WebAssembly support for browser environments

## API Overview

```rust
use balinese_calendar::{BalineseDate, Rahinan, Sasih, Wuku};

// From Gregorian date
let d = BalineseDate::from_ymd(2026, 6, 17)?;

// Pawukon
assert_eq!(d.wuku, Wuku::Dungulan);
assert_eq!(d.saptawara, Saptawara::Buda);
assert_eq!(d.pancawara, Pancawara::Kliwon);

// Sasih
assert_eq!(d.sasih, Sasih::Kasa);
assert_eq!(d.saka_year, 1948);

// Rahinan (ceremony detection)
assert!(d.rahinan.iter().any(|r| matches!(r, Rahinan::Galungan)));

// Urip
assert_eq!(d.saptawara.urip() + d.pancawara.urip(), 15);

// 210-day cycle
let d2 = BalineseDate::from_jdn(d.jdn + 210);
assert_eq!(d.wuku, d2.wuku);
assert_eq!(d.pancawara, d2.pancawara);
```

## WebAssembly (WASM) Support

Enable client-side Balinese calendar in any web frontend:

```javascript
import init, { from_ymd, today } from 'balinese-calendar';

// Initialize WASM module
await init();

// Create Balinese date
const date = from_ymd(2026, 3, 26);
console.log(date.toBalineseString());
// "Redite Pon Dukut, Sasih Kadasa, Saka 1948"

// Get today's Balinese date
const today = today();
console.log(today.rahinanList());
// ["Galungan", "Kuningan", ...]
```

**Custom day boundaries:**
```javascript
import { from_ymd_fixed_sunrise, from_ymd_astronomical } from 'balinese-calendar';

// Fixed sunrise at 6 AM UTC
const fixed = from_ymd_fixed_sunrise(2026, 3, 26, 6);

// Astronomical sunrise for Bali (default coordinates)
const astro = from_ymd_astronomical(2026, 3, 26, -8.3405, 115.0920);
```

## Astronomical Sunrise

Calculate actual astronomical sunrise times for any location:

```rust
use balinese_calendar::{BalineseDate, DayBoundary};

// Bali centroid coordinates
let boundary = DayBoundary::Astronomical {
    lat: -8.3405,
    lon: 115.0920
};

let date = BalineseDate::from_ymd_with_boundary(2026, 3, 26, &boundary)?;
println!("Balinese date with astronomical sunrise: {}", date.to_balinese_string());

// Custom coordinates for other Hindu communities
let custom_boundary = DayBoundary::Astronomical {
    lat: -6.2088,  // Jakarta
    lon: 106.8456
};
```

**Features:**
- Real astronomical calculations using the `sunrise` crate
- Custom coordinates for any location worldwide
- Handles polar day/night conditions gracefully
- Tested against BMKG (Indonesian meteorological agency) reference data

## Supported Date Range

1900–2100 (validated 1969–2027, extrapolated outside this range). Historical dates before 1900 are planned for v1.0.0.

## Architecture

All computation is deterministic and `O(1)` per date — no database lookups, no network calls, no floating-point for core calendar operations.

The Sasih walk-forward algorithm starts from peradnya-calibrated pivot points and walks to the target JDN, handling Nampih Sasih (intercalary months) and Ngunaratri (intercalary tithis) along the way.

The Dewasa Ayu engine (v0.3.0) is the only component requiring `f64` and will be feature-gated behind `dewasa-ayu`.

## License

[MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE)

## Acknowledgements

- **I Made Bidja Alm.** — compiler of the 2026 reference calendar
- **I Ketut Suwintana** (Politeknik Negeri Bali) — kalenderbali.info
- **I Gusti Agung Mahendra Putra** (peradnya) — balinese-date-java-lib
- **Edy Santosa Putra** — sakacalendar Java library
- **E.W. Hary Candana, I.G.A. Gunadi, D.G.H. Divayana** (Universitas Pendidikan Ganesha) — Sugeno comparison study
- **N. Karjanto & F. Beauducel** — ethnoarithmetic Zeller's congruence

---

<div align="center">

**If you find this crate useful, please consider giving it a star on [GitHub](https://github.com/SHA888/balinese-calendar)!**

</div>
