# Basic Usage

This guide covers common usage patterns for the Balinese Calendar crate.

## Creating Dates

### From Gregorian Date

```rust
use balinese_calendar::BalineseDate;

// Create from year, month, day
let date = BalineseDate::from_ymd(2026, 3, 25)?;
```

### From Current Date

```rust
use chrono::Local;
use balinese_calendar::BalineseDate;

let now = Local::now();
let today = BalineseDate::from_ymd(
    now.year(),
    now.month(),
    now.day()
)?;
```

## Accessing Calendar Components

### Wewaran (Weekly Cycles)

```rust
let date = BalineseDate::from_ymd(2026, 3, 25)?;

// 7-day week (like Sunday-Saturday)
println!("Saptawara: {}", date.saptawara.name());

// 5-day market week
println!("Pancawara: {}", date.pancawara.name());

// Other cycles
println!("Ekawara: {}", date.ekawara.name());    // 1-day
println!("Dwiwara: {}", date.dwiwara.name());    // 2-day
println!("Triwara: {}", date.triwara.name());    // 3-day
println!("Caturwara: {}", date.caturwara.name()); // 4-day
println!("Sadwara: {}", date.sadwara.name());    // 6-day
println!("Astawara: {}", date.astawara.name());  // 8-day
println!("Sangawara: {}", date.sangawara.name()); // 9-day
println!("Dasawara: {}", date.dasawara.name());  // 10-day
```

### Pawukon System

```rust
// Wuku (30 weeks in 210-day cycle)
println!("Wuku: {}", date.wuku.name());
println!("Wuku day: {}/7", date.wuku_day);
```

### Paringkelan

```rust
// Various paringkelan components
println!("Jejepan: {}", date.jejepan.name());
println!("Ingkel: {}", date.ingkel.name());
println!("Watek Alit: {}", date.watek_alit.name());
println!("Watek Madya: {}", date.watek_madya.name());
println!("Lintang: {}", date.lintang.name());
println!("Panca Suda: {}", date.panca_suda.name());
println!("Pararasan: {}", date.pararasan.name());
println!("Rakam: {}", date.rakam.name());
```

### Sasih (Lunar Calendar)

```rust
// Lunar month and year
println!("Sasih: {}", date.sasih.name());
println!("Saka Year: {}", date.saka_year);
println!("Sasih Day: {}", date.sasih_day.day());

// Check for intercalary month
if date.is_nampih {
    println!("This is a Nampih (intercalary) month");
}
```

## Working with Holy Days

### Checking Moon Phases

```rust
if date.is_purnama {
    println!("Today is Purnama (full moon)");
}

if date.is_tilem {
    println!("Today is Tilem (new moon)");
}
```

### Finding Rahinan

```rust
// Check if there are any holy days
if !date.rahinan.is_empty() {
    println!("Holy days today:");
    for rahinan in &date.rahinan {
        println!("  - {}", rahinan.name());
    }
}
```

### Common Holy Day Combinations

```rust
// Kajeng Kliwon (auspicious day)
if date.triwara.name() == "Kajeng" && date.pancawara.name() == "Kliwon" {
    println!("Today is Kajeng Kliwon!");
}

// Buda Cemeng Klawu (inauspicious day)
if date.caturwara.name() == "Menala" && date.pancawara.name() == "Kliwon" {
    println!("Today is Buda Cemeng Klawu");
}
```

## Date Ranges and Iterations

### Finding Specific Days

```rust
use balinese_calendar::BalineseDate;

// Find all Purnama days in 2026
let mut purnama_dates = Vec::new();

for month in 1..=12 {
    for day in 1..=31 {
        if let Ok(date) = BalineseDate::from_ymd(2026, month, day) {
            if date.is_purnama {
                purnama_dates.push((month, day));
            }
        }
    }
}

println!("Purnama dates in 2026: {:?}", purnama_dates);
```

### Calendar Generation

```rust
// Generate a month calendar
fn print_month_calendar(year: i32, month: u32) -> Result<(), Box<dyn std::error::Error>> {
    println!("Calendar for {}-{:02}", year, month);
    println!("{:>3} {:>10} {:>10}", "Day", "Wuku", "Saptawara");

    for day in 1..=31 {
        if let Ok(date) = BalineseDate::from_ymd(year, month, day) {
            println!("{:>3} {:>10} {:>10}",
                     day,
                     date.wuku.name(),
                     date.saptawara.name());
        }
    }

    Ok(())
}
```

## Dual Naming Traditions

Some components support multiple naming traditions:

```rust
// Pararasan has two naming traditions
println!("Aryana: {}", date.pararasan.name());
println!("Sundari Bungkah: {}", date.pararasan.name_sundari_bungkah());
```

## Error Handling

### Handling Invalid Dates

```rust
use balinese_calendar::{BalineseDate, BalineseDateError};

match BalineseDate::from_ymd(2026, 2, 30) {
    Ok(date) => println!("Valid date"),
    Err(BalineseDateError::InvalidDate { year, month, day }) => {
        eprintln!("Invalid date: {}-{}-{}", year, month, day);
    }
    Err(BalineseDateError::OutOfRange) => {
        eprintln!("Date out of range (1800-2200)");
    }
}
```

### Graceful Degradation

```rust
// Process a range of dates, skipping invalid ones
for day in 1..=31 {
    if let Ok(date) = BalineseDate::from_ymd(2026, 2, day) {
        // Process valid date
        println!("Day {}: {}", day, date.wuku.name());
    }
    // Invalid dates (like Feb 30) are silently skipped
}
```

## Performance Considerations

### Efficient Batch Processing

```rust
// Good: Reuse date object
let date = BalineseDate::from_ymd(2026, 3, 25)?;
let wuku = date.wuku.name();
let sapta = date.saptawara.name();
let panca = date.pancawara.name();

// Less efficient: Multiple conversions
let wuku = BalineseDate::from_ymd(2026, 3, 25)?.wuku.name();
let sapta = BalineseDate::from_ymd(2026, 3, 25)?.saptawara.name();
let panca = BalineseDate::from_ymd(2026, 3, 25)?.pancawara.name();
```

## Next Steps

- Explore [Advanced Features](./advanced-features.md) for more complex use cases
- Learn about [Calendar Systems](../concepts/calendar-systems.md) to understand the concepts
- Check the [API Reference](../api/balinese-date.md) for complete documentation
