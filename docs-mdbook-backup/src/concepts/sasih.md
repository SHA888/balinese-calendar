# Sasih (Lunar Month)

Sasih are the lunar months that form the basis of the Saka calendar system in Bali. Each Sasih represents approximately one lunar cycle and has specific cultural and religious significance.

## Overview

The Saka calendar uses 12 lunar months (Sasih):

- **Duration**: Approximately 354 days (12 × 29.5 days)
- **Cycle**: Lunar phases from new moon to new moon
- **Purpose**: Religious ceremonies and agricultural timing
- **Integration**: Works alongside the Pawukon cycle

## The 12 Sasih

### 1. Kasa (First Month)
- **Approximate dates**: March-April
- **Characteristics**: Beginning of the Saka year
- **Significance**: New year celebrations and renewal
- **Agricultural**: Start of planting season

### 2. Karo (Second Month)
- **Approximate dates**: April-May
- **Characteristics**: Growth and development
- **Significance**: Youth ceremonies and coming-of-age rituals

### 3. Katiga (Third Month)
- **Approximate dates**: May-June
- **Characteristics**: Abundance and prosperity
- **Significance**: Harvest festivals and thanksgiving

### 4. Kapat (Fourth Month)
- **Approximate dates**: June-July
- **Characteristics**: Balance and harmony
- **Significance**: Marriage ceremonies and unions

### 5. Kalima (Fifth Month)
- **Approximate dates**: July-August
- **Characteristics**: Challenge and testing
- **Significance**: Rites of passage and initiation

### 6. Kanem (Sixth Month)
- **Approximate dates**: August-September
- **Characteristics**: Community and cooperation
- **Significance**: Village festivals and communal activities

### 7. Kapitu (Seventh Month)
- **Approximate dates**: September-October
- **Characteristics**: Spiritual peak
- **Significance**: Major temple ceremonies and purification

### 8. Kaulu (Eighth Month)
- **Approximate dates**: October-November
- **Characteristics**: Transition and change
- **Significance**: Ancestor worship and remembrance

### 9. Kasanga (Ninth Month)
- **Approximate dates**: November-December
- **Characteristics**: Completion and fulfillment
- **Significance**: Year-end preparations and reflection

### 10. Kadasa (Tenth Month)
- **Approximate dates**: December-January
- **Characteristics**: Wisdom and knowledge
- **Significance**: Educational ceremonies and learning

### 11. Jestha (Eleventh Month)
- **Approximate dates**: January-February
- **Characteristics**: Power and authority
- **Significance**: Royal ceremonies and leadership rituals

### 12. Sadha (Twelfth Month)
- **Approximate dates**: February-March
- **Characteristics**: Completion and renewal
- **Significance**: Year-end ceremonies and preparation for new year

## Lunar Phase System

Each Sasih is divided into two phases based on the moon:

### Suklapaksa (Waxing Moon)
- **Duration**: First 15 days of the Sasih
- **Characteristics**: Growing, increasing energy
- **Activities**: Beginning new projects, planting, construction

### Krsnapaksa (Waning Moon)
- **Duration**: Last 14-15 days of the Sasih
- **Characteristics**: Decreasing, reflective energy
- **Activities**: Completion, harvesting, spiritual practices

## Sasih and Pawukon Integration

The Sasih system works alongside the Pawukon cycle:

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;

// Get both Sasih and Pawukon information
println!("Sasih: {} ({})", date.sasih(), date.sasih_name());
println!("Pawukon day: {}", date.pawukon_day());
println!("Wuku: {}", date.wuku_name());

// Check if it's a significant day
if date.is_sasih_transition() {
    println!("Today marks a Sasih transition!");
}
```

## Important Sasih-based Ceremonies

### Nyepi (Balinese New Year)
- **Sasih**: Kasa (first month)
- **Timing**: New moon of Kasa
- **Significance**: Day of silence and reflection

### Saraswati Day
- **Sasih**: Kadasa (tenth month)
- **Timing**: Saturday of Wuku Watugunung
- **Significance**: Knowledge and learning

### Pagerwesi
- **Sasih**: Kanem (sixth month)
- **Timing**: Wednesday of Wuku Sinta
- **Significance**: Spiritual protection and strength

## Agricultural Calendar

The Sasih system traditionally guides agricultural activities:

| Sasih | Activity | Reason |
|-------|----------|--------|
| Kasa-Karo | Planting rice | Beginning of growing season |
| Katiga-Kapat | Field maintenance | Growth period |
| Kalima-Kanem | First harvest | Early crops ready |
| Kapitu-Kaulu | Main harvest | Peak harvesting time |
| Kasanga-Sadha | Land preparation | Preparing for next cycle |

## Library Implementation

The Balinese Calendar library provides comprehensive Sasih support:

```rust
use balinese_calendar::{BalineseDate, Sasih};

let date = BalineseDate::from_ymd(2023, 1, 1)?;

// Get Sasih information
let sasih = date.sasih();
let sasih_name = date.sasih_name();
let lunar_phase = date.lunar_phase();

println!("Sasih Information:");
println!("  Number: {}", sasih);
println!("  Name: {}", sasih_name);
println!("  Lunar phase: {:?}", lunar_phase);
println!("  Day in Sasih: {}", date.day_in_sasih());

// Check Sasih characteristics
match sasih {
    Sasih::Kasa => println!("Beginning of Saka year - renewal time"),
    Sasih::Kapitu => println!("Spiritual peak - important ceremonies"),
    Sasih::Sadha => println!("Year completion - preparation time"),
    _ => println!("Regular Sasih"),
}
```

## Sasih Calculations

The library uses accurate astronomical calculations:

```rust
use balinese_calendar::BalineseDate;

// Calculate Saka year from Gregorian date
let date = BalineseDate::from_ymd(2023, 1, 1)?;
let saka_year = date.saka_year();
println!("Saka year: {}", saka_year); // Output: 1945

// Get days since Sasih beginning
let day_in_sasih = date.day_in_sasih();
println!("Day {} in {}", day_in_sasih, date.sasih_name());

// Check for Sasih transition
if date.is_sasih_transition() {
    println!("Today is a Sasih transition day (new moon)");
}
```

## Cultural Significance

The Sasih system governs:

- **Religious ceremonies** and temple festivals
- **Agricultural cycles** and planting/harvesting times
- **Personal life events** (birth, marriage, death ceremonies)
- **Economic activities** and market cycles
- **Spiritual practices** and purification rituals

## Modern Adaptations

While maintaining traditional significance, the Sasih system is now used for:

- **Cultural preservation** and education
- **Tourism planning** around major ceremonies
- **Agricultural modernization** with traditional wisdom
- **Digital calendar applications** and reminders
- **Academic research** and cultural studies
