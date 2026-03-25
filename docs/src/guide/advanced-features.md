# Advanced Features

This section covers advanced usage patterns and features of the Balinese Calendar library.

## Custom Day Boundaries

The library supports different ways to determine the day boundary (sunrise) for Balinese calendar calculations:

### Fixed Sunrise

```rust
use balinese_calendar::{BalineseDate, DayBoundary};

// Use a fixed sunrise offset (default: 6 AM UTC+2)
let date = BalineseDate::from_ymd(2023, 1, 1)
    .with_day_boundary(DayBoundary::FixedSunrise(6))?;
```

### Midnight Boundary

```rust
// Use Gregorian midnight (legacy behavior)
let date = BalineseDate::from_ymd(2023, 1, 1)
    .with_day_boundary(DayBoundary::Midnight)?;
```

## Astronomical Calculations

When the `astronomical` feature is enabled, you can use real astronomical sunrise calculations:

```rust
#[cfg(feature = "astronomical")]
use balinese_calendar::{BalineseDate, DayBoundary};

#[cfg(feature = "astronomical")]
let date = BalineseDate::from_ymd(2023, 1, 1)
    .with_day_boundary(DayBoundary::Astronomical {
        lat: -8.3405,  // Bali latitude
        lon: 115.0920, // Bali longitude
    })?;
```

> **Note**: Astronomical calculations are currently not implemented and will return an error.

## Performance Optimization

### Batch Processing

For processing multiple dates efficiently:

```rust
use balinese_calendar::BalineseDate;

// Create multiple dates
let dates: Vec<BalineseDate> = (1..31)
    .map(|day| BalineseDate::from_ymd(2023, 1, day))
    .collect::<Result<_, _>>()?;

// Process in batch
for date in dates {
    println!("{}: {}", date, date.pawukon_day());
}
```

### Memory Efficiency

The library uses zero-copy operations where possible and has minimal memory overhead:

- `BalineseDate` struct is only 16 bytes
- No dynamic allocations for basic operations
- Efficient string representations

## Error Handling

Comprehensive error handling for edge cases:

```rust
use balinese_calendar::{BalineseDate, BalineseDateError};

match BalineseDate::from_ymd(2023, 13, 1) {
    Ok(date) => println!("Date: {}", date),
    Err(BalineseDateError::InvalidMonth(month)) => {
        eprintln!("Invalid month: {}", month);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Feature Flags

The library supports several feature flags for customization:

- `default`: Basic functionality with fixed sunrise
- `astronomical`: Enable astronomical sunrise calculations
- `serde`: Enable serialization/deserialization support

```toml
[dependencies]
balinese-calendar = { version = "0.1", features = ["serde"] }
```

## Integration Examples

### With Chrono

```rust
use chrono::{NaiveDate, Utc};
use balinese_calendar::BalineseDate;

let chrono_date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
let balinese_date = BalineseDate::from_ymd(2023, 1, 1)?;

// Convert back to chrono
let back_to_chrono = balinese_date.to_gregorian();
```

### With Serde

```rust
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
struct Event {
    name: String,
    date: BalineseDate,
}
```

## Best Practices

1. **Use appropriate day boundaries** for your use case
2. **Handle errors gracefully** - don't unwrap() in production code
3. **Batch operations** when processing multiple dates
4. **Use feature flags** to minimize binary size
5. **Cache calculations** if you need the same date repeatedly
