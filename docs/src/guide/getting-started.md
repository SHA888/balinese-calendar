# Getting Started

This guide will help you get started with the Balinese Calendar crate in your Rust project.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
balinese-calendar = "0.1.2"
```

For the latest development version:

```toml
[dependencies]
balinese-calendar = { git = "https://github.com/SHA888/balinese-calendar" }
```

## Optional Features

The crate provides optional features you can enable:

```toml
[dependencies]
balinese-calendar = { version = "0.1.2", features = ["astronomical", "serde"] }
```

Available features:
- `astronomical` - Enables astronomical calculations for sunrise/sunset times
- `serde` - Enables serialization/deserialization support

## Your First Program

Create a simple program to display today's Balinese date:

```rust
use balinese_calendar::BalineseDate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get today's date
    let today = BalineseDate::from_ymd(2026, 3, 25)?;

    // Display basic information
    println!("Gregorian: {}-{:02}-{:02}",
             today.gregorian_year,
             today.gregorian_month,
             today.gregorian_day);

    println!("Wuku: {}", today.wuku.name());
    println!("Saptawara: {}", today.saptawara.name());
    println!("Pancawara: {}", today.pancawara.name());
    println!("Sasih: {}", today.sasih.name());
    println!("Saka Year: {}", today.saka_year);

    Ok(())
}
```

## Understanding the Output

The program above will output something like:

```
Gregorian: 2026-03-25
Wuku: Sinta
Saptawara: Redite
Pancawara: Umanis
Sasih: Kasanga
Saka Year: 1948
```

Each component represents a different aspect of the Balinese calendar system:

- **Wuku**: One of 30 weeks in the 210-day Pawukon cycle
- **Saptawara**: Day of the 7-day week (similar to Sunday-Saturday)
- **Pancawara**: Day of the 5-day market week
- **Sasih**: Lunar month in the Saka calendar
- **Saka Year**: Year in the Balinese Saka calendar

## Next Steps

- Learn about [Basic Usage](./basic-usage.md) for common operations
- Explore [Calendar Systems](../concepts/calendar-systems.md) to understand the concepts
- Check the [API Reference](../api/balinese-date.md) for detailed documentation

## Common Patterns

### Converting from Current Date

```rust
use chrono::Local;
use balinese_calendar::BalineseDate;

let now = Local::now();
let balinese = BalineseDate::from_ymd(
    now.year(),
    now.month(),
    now.day()
)?;
```

### Checking for Holy Days

```rust
if !balinese.rahinan.is_empty() {
    println!("Today is a holy day!");
    for rahinan in &balinese.rahinan {
        println!("  - {}", rahinan.name());
    }
}
```

### Working with Multiple Dates

```rust
use balinese_calendar::BalineseDate;

let dates = vec![
    BalineseDate::from_ymd(2026, 1, 1)?,
    BalineseDate::from_ymd(2026, 6, 15)?,
    BalineseDate::from_ymd(2026, 12, 31)?,
];

for date in dates {
    println!("{}: Wuku {}",
             date.gregorian_day,
             date.wuku.name());
}
```

## Troubleshooting

### Date Out of Range

The crate supports dates from 1800 to 2200. If you need dates outside this range, please open an issue.

```rust
// This will return an error
let invalid = BalineseDate::from_ymd(1799, 1, 1);
assert!(invalid.is_err());
```

### Invalid Dates

The crate validates dates and rejects impossible dates:

```rust
// February 30 doesn't exist
let invalid = BalineseDate::from_ymd(2026, 2, 30);
assert!(invalid.is_err());
```

## Getting Help

- Check the [FAQ](../reference/faq.md) for common questions
- Read the [API Reference](../api/balinese-date.md) for detailed documentation
- Open an issue on [GitHub](https://github.com/SHA888/balinese-calendar/issues) for bugs or questions
