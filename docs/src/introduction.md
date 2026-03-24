# Introduction

Welcome to the **Balinese Calendar** documentation! This is the first native Rust implementation of the Balinese Saka Calendar (Kalender Bali), providing comprehensive support for computing Pawukon, Wewaran, Sasih, Paringkelan, Rahinan, and Saka year from any Gregorian date.

## What is the Balinese Calendar?

The Balinese calendar system is a complex and sophisticated timekeeping system used in Bali, Indonesia. It combines multiple overlapping cycles:

- **Pawukon**: A 210-day cycle fundamental to Balinese timekeeping
- **Wewaran**: Nine concurrent weekly cycles (1-10 days)
- **Sasih**: Lunar months based on the Saka calendar
- **Paringkelan**: Additional cyclic systems for divination and ritual timing
- **Rahinan**: Holy days and ceremonial occasions

## Features

- ✅ **Complete Pawukon System** - All 30 wuku with accurate day calculations
- ✅ **Nine Wewaran Cycles** - From Ekawara (1-day) to Dasawara (10-day)
- ✅ **Sasih Lunar Calendar** - Including Nampih (intercalary) months
- ✅ **Paringkelan Systems** - Jejepan, Ingkel, Watek, Lintang, and more
- ✅ **Rahinan Detection** - Automatic detection of holy days
- ✅ **Dual Naming Traditions** - Support for Aryana and Sundari Bungkah traditions
- ✅ **Performance Optimized** - Efficient calculations with minimal overhead
- ✅ **Type Safe** - Leverages Rust's type system for correctness
- ✅ **Well Tested** - Comprehensive test suite with validation corpus

## Quick Example

```rust
use balinese_calendar::BalineseDate;

// Create a Balinese date from Gregorian date
let date = BalineseDate::from_ymd(2026, 3, 25)?;

// Access various calendar components
println!("Wuku: {}", date.wuku.name());
println!("Saptawara: {}", date.saptawara.name());
println!("Pancawara: {}", date.pancawara.name());
println!("Sasih: {}", date.sasih.name());

// Check for holy days
if !date.rahinan.is_empty() {
    println!("Holy days: {:?}", date.rahinan);
}
```

## Why Rust?

This implementation is written in Rust to provide:

- **Memory Safety** - No null pointer errors or buffer overflows
- **Performance** - Zero-cost abstractions and efficient execution
- **Reliability** - Strong type system catches errors at compile time
- **Portability** - Easy to integrate into various platforms and languages

## Getting Started

Ready to use the Balinese Calendar in your project? Head over to the [Getting Started](./guide/getting-started.md) guide to begin!

## Project Status

This project is actively maintained and used in production. We welcome contributions from the community. See the [Contributing](./development/contributing.md) guide to get involved.

## License

This project is licensed under the Apache-2.0 license. See the repository for full license text.
