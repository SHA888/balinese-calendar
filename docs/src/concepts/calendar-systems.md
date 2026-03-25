# Calendar Systems

The Balinese calendar system is a complex combination of several different calendar systems that have been used in Bali for centuries. Understanding these systems is essential for proper use of the Balinese Calendar library.

## Overview

The Balinese calendar combines:

1. **Saka Calendar** - A lunar calendar system
2. **Pawukon Cycle** - A 210-day ritual cycle
3. **Wewaran Systems** - Various week-based cycles
4. **Gregorian Calendar** - For civil date correspondence

## Saka Calendar (Lunar Calendar)

The Saka calendar is the official lunar calendar used in Bali:

- **Year length**: 354 or 355 days (lunar year)
- **Months**: 12 lunar months (Sasih)
- **New Year**: Usually falls in March
- **Epoch**: Started in 78 CE

### Sasih (Lunar Months)

Each Sasih (lunar month) is approximately 29.5 days:

1. **Kasa** - First month
2. **Karo** - Second month
3. **Katiga** - Third month
4. **Kapat** - Fourth month
5. **Kalima** - Fifth month
6. **Kanem** - Sixth month
7. **Kapitu** - Seventh month
8. **Kaulu** - Eighth month
9. **Kasanga** - Ninth month
10. **Kadasa** - Tenth month
11. **Jestha** - Eleventh month
12. **Sadha** - Twelfth month

## Pawukon Cycle

The Pawukon is a unique 210-day cycle that runs independently of the lunar calendar:

- **Duration**: 210 days (30 weeks of 7 days)
- **Purpose**: Determines ritual days and ceremonies
- **Components**: Multiple overlapping cycles

### Pawukon Structure

The Pawukon cycle consists of:

- **7-day week** (Saptawara)
- **5-day week** (Pancawara)
- **6-day week** (Sadwara)
- **8-day week** (Astawara)
- **9-day week** (Sangawara)
- **10-day week** (Dasawara)

## Wewaran Systems

Wewaran are the various week-based systems that operate simultaneously:

### Primary Wewaran

1. **Saptawara** (7-day) - Most commonly used
2. **Pancawara** (5-day) - Important for market days
3. **Sadwara** (6-day) - Used for certain ceremonies
4. **Astawara** (8-day) - Less common
5. **Sangawara** (9-day) - Rarely used
6. **Dasawara** (10-day) - Special ceremonial purposes

### Wewaran Days

Each Wewaran system has its own set of day names:

#### Saptawara (7-day)
- Redite (Sunday)
- Soma (Monday)
- Anggara (Tuesday)
- Buda (Wednesday)
- Wraspati (Thursday)
- Sukra (Friday)
- Saniscara (Saturday)

#### Pancawara (5-day)
- Umanis
- Paing
- Pon
- Wage
- Keliwon

## Paringkelan

Paringkelan is another cycle system that groups days:

- **Duration**: 35 days
- **Groups**: 5 groups of 7 days each
- **Purpose**: Agricultural and ceremonial timing

## Rahinan (Holy Days)

Rahinan are special holy days determined by the intersection of various calendar systems:

### Major Holy Days

1. **Galungan** - Every 210 days (Pawukon cycle)
2. **Kuningan** - 10 days after Galungan
3. **Nyepi** - Balinese New Year (Saka calendar)
4. **Saraswati** - Knowledge day
5. **Pagerwesi** - Metal day

### Minor Holy Days

- **Tumpek** - Six ceremonial days every 35 days
- **Kajeng Kliwon** - Every 15 days
- **Buda Cemeng** - Every 35 days

## Calendar Interactions

The beauty of the Balinese calendar system is how these different cycles interact:

### Example: Determining a Ceremonial Day

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;

// Check if it's a special day
if date.is_galungan() {
    println!("Today is Galungan!");
}

// Get all day information
println!("Saptawara: {}", date.saptawara());
println!("Pancawara: {}", date.pancawara());
println!("Pawukon day: {}", date.pawukon_day());
```

## Civil Calendar Integration

The Balinese calendar corresponds to the Gregorian calendar:

- **Date conversion**: Automatic Gregorian ↔ Balinese conversion
- **Time zones**: Bali uses UTC+8 (WITA)
- **Day boundaries**: Traditionally at sunrise

## Historical Context

The Balinese calendar system evolved from:

- **Hindu calendar traditions** from India
- **Local Javanese influences**
- **Indigenous Balinese practices**
- **Agricultural cycles** of the region

## Modern Usage

Today, the Balinese calendar is used for:

- **Religious ceremonies** and temple festivals
- **Cultural events** and traditional performances
- **Agricultural planning** and harvesting
- **Personal auspicious days** for important activities

## Library Implementation

The Balinese Calendar library implements:

- Accurate date calculations for all systems
- Efficient conversion between calendar types
- Comprehensive day information retrieval
- Error handling for edge cases

```rust
// Get comprehensive day information
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;

println!("Full date information:");
println!("  Gregorian: {}", date.to_gregorian());
println!("  Saka Year: {}", date.saka_year());
println!("  Sasih: {}", date.sasih());
println!("  Saptawara: {}", date.saptawara());
println!("  Pancawara: {}", date.pancawara());
println!("  Pawukon: {}", date.pawukon_day());
