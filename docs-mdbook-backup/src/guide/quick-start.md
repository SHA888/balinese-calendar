# Quick Start

Get up and running with the Balinese Calendar in minutes!

## Basic Example

```rust
use balinese_calendar::BalineseDate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Balinese date from Gregorian date
    let date = BalineseDate::from_ymd(2026, 3, 25)?;

    println!("Wuku: {}", date.wuku.name());
    println!("Day: {}", date.saptawara.name());

    Ok(())
}
```

## Common Use Cases

### 1. Get Today's Balinese Date

```rust
use chrono::Local;
use balinese_calendar::BalineseDate;

let now = Local::now();
let today = BalineseDate::from_ymd(
    now.year(),
    now.month(),
    now.day()
)?;

println!("Today is {} in the Balinese calendar", today.wuku.name());
```

### 2. Check for Holy Days

```rust
let date = BalineseDate::from_ymd(2026, 3, 25)?;

if date.is_purnama {
    println!("Today is Purnama (full moon)!");
}

if date.is_tilem {
    println!("Today is Tilem (new moon)!");
}

for rahinan in &date.rahinan {
    println!("Holy day: {}", rahinan.name());
}
```

### 3. Access All Calendar Components

```rust
let date = BalineseDate::from_ymd(2026, 3, 25)?;

// Wewaran (weekly cycles)
println!("Saptawara: {}", date.saptawara.name());
println!("Pancawara: {}", date.pancawara.name());
println!("Triwara: {}", date.triwara.name());

// Pawukon
println!("Wuku: {}", date.wuku.name());
println!("Wuku day: {}", date.wuku_day);

// Paringkelan
println!("Ingkel: {}", date.ingkel.name());
println!("Jejepan: {}", date.jejepan.name());

// Sasih (lunar month)
println!("Sasih: {}", date.sasih.name());
println!("Saka year: {}", date.saka_year);
```

### 4. Work with Date Ranges

```rust
use balinese_calendar::BalineseDate;

// Find all Kajeng Kliwon days in March 2026
let mut kajeng_kliwon_dates = Vec::new();

for day in 1..=31 {
    if let Ok(date) = BalineseDate::from_ymd(2026, 3, day) {
        if date.triwara.name() == "Kajeng" && date.pancawara.name() == "Kliwon" {
            kajeng_kliwon_dates.push(date);
        }
    }
}

println!("Found {} Kajeng Kliwon days", kajeng_kliwon_dates.len());
```

### 5. Dual Naming Traditions

```rust
let date = BalineseDate::from_ymd(2026, 3, 25)?;

// Aryana tradition (default)
println!("Aryana: {}", date.pararasan.name());

// Sundari Bungkah tradition
println!("Sundari Bungkah: {}", date.pararasan.name_sundari_bungkah());
```

## Error Handling

The crate uses `Result` types for operations that can fail:

```rust
use balinese_calendar::{BalineseDate, BalineseDateError};

match BalineseDate::from_ymd(2026, 2, 30) {
    Ok(date) => println!("Valid date: {}", date.wuku.name()),
    Err(BalineseDateError::InvalidDate { year, month, day }) => {
        eprintln!("Invalid date: {}-{}-{}", year, month, day);
    }
    Err(BalineseDateError::OutOfRange) => {
        eprintln!("Date out of supported range (1800-2200)");
    }
}
```

## Performance Tips

The crate is optimized for performance, but here are some tips:

1. **Reuse calculations**: If you need multiple components, access them from the same `BalineseDate` instance
2. **Batch processing**: Process multiple dates in a loop rather than creating individual instances
3. **Avoid unnecessary conversions**: Work with `BalineseDate` directly when possible

## Next Steps

- Learn about [Basic Usage](./basic-usage.md) for more detailed examples
- Understand the [Calendar Systems](../concepts/calendar-systems.md)
- Explore [Advanced Features](./advanced-features.md)
- Check the [API Reference](../api/balinese-date.md)
