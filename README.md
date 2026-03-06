# balinese-calendar 🌺

[![Crates.io](https://img.shields.io/crates/v/balinese-calendar.svg)](https://crates.io/crates/balinese-calendar)
[![docs.rs](https://docs.rs/balinese-calendar/badge.svg)](https://docs.rs/balinese-calendar)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

**The first native Rust implementation of the Balinese Saka Calendar (Kalender Bali).**

Zero-allocation, `no_unsafe`, immutable value types. Converts any Gregorian date
into the full Balinese calendar representation — Pawukon, Wewaran, Sasih, Paringkelan,
Rahinan — in a single function call.

Built for Balinese ecological timekeeping and cultural preservation.

---

## Features

| System | Coverage |
|---|---|
| **Pawukon** | 30 Wuku × 7 days (210-day cycle) with ecology tags |
| **Pawewaran** | All 10 concurrent week cycles: Eka · Dwi · Tri · Catur · Panca · Sad · Sapta · Asta · Sanga · Dasa Wara |
| **Sasih** | 12 lunar months + Nampih Sasih (intercalary), Penanggal/Pangelong/Purnama/Tilem, Ngunaratri |
| **Saka year** | Gregorian → Saka conversion with Nyepi boundary correction |
| **Paringkelan** | Jejepan · Ingkel · Watek (Madya & Alit) · Lintang · PancaSuda · Pararasan · Rakam |
| **Rahinan** | Galungan · Kuningan · Saraswati · Pagerwesi · Tumpek (5 types) · Kajeng Keliwon · Purnama · Tilem · Nyepi · Siwa Ratri · Anggar Kasih |

---

## Quick Start

```toml
[dependencies]
balinese-calendar = "0.1"
```

```rust
use balinese_date::BalineseDate;

// Today's date
let today = BalineseDate::today()?;
println!("{}", today.to_balinese_string());
// → "Sukra Umanis Sungsang, Penanggal 7 Kasanga Saka 1948"

// Any Gregorian date
let nyepi = BalineseDate::from_ymd(2026, 3, 19)?;
println!("Saka year: {}", nyepi.saka_year);   // → 1948
println!("Sasih:     {}", nyepi.sasih.name()); // → "Kadasa"
println!("Wuku:      {}", nyepi.wuku.name());  // → "..."

// Check holy days
for rahinan in &today.rahinan {
    println!("Rahinan: {:?}", rahinan);
}

// Pancaroba (seasonal transition) flag
if today.sasih.is_pancaroba() {
    println!("In pancaroba transition");
}
```

---

## Algorithm Basis

- Ardhana, I.B.S. (2005). *"Pokok-Pokok Wariga"*. Surabaya: Paramita.
- babadbali.com (Yayasan Bali Galang) — wewaran & paringkelan algorithms.
- Pendit, N.S. (2001). *"Nyepi: kebangkitan, toleransi, dan kerukunan"*. Gramedia.
- [peradnya/balinese-calendar-js-lib](https://github.com/peradnya/balinese-calendar-js-lib) (Apache-2.0) — behaviour reference.
- Wikipedia: [Balinese Saka Calendar](https://en.wikipedia.org/wiki/Balinese_saka_calendar).

**Validation target:** [kalenderbali.org](https://kalenderbali.org) (I Wayan Nuarsa, Universitas Udayana).

---

## ⚠ Production Note: Nampih Sasih

Intercalary month (Nampih Sasih) placement is declared annually by
**PHDI** (Parisada Hindu Dharma Indonesia). The built-in `NAMPIH_YEARS`
array in `src/sasih.rs` must be verified and extended each year from the
official PHDI calendar before deploying in production.

---

## Run Tests

```bash
cargo test
cargo run --example today
cargo bench
```

---

## About

Built for Bali to preserve and promote Balinese culture through technology by Bali developers.

**License:** Apache-2.0
