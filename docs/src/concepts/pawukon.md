# Pawukon Cycle

The Pawukon cycle is the heart of the Balinese calendar system. It's a unique 210-day cycle that determines the timing of most religious ceremonies and cultural events in Bali.

## Overview

The Pawukon cycle is:

- **Duration**: 210 days exactly
- **Structure**: 30 weeks of 7 days each
- **Origin**: Ancient Javanese/Balinese astronomical calculations
- **Purpose**: Determines auspicious days for ceremonies

## Mathematical Structure

The Pawukon cycle is based on the least common multiple (LCM) of several week systems:

```
LCM(7, 5, 6, 8, 9, 10) = 1260
```

However, the actual cycle used is 210 days, which is:

```
LCM(7, 5, 6) = 210
```

## Component Cycles

The Pawukon consists of several overlapping cycles:

### Primary Cycles

| Cycle | Length | Day Names | Purpose |
|-------|--------|-----------|---------|
| Saptawara | 7 days | Redite, Soma, Anggara, Buda, Wraspati, Sukra, Saniscara | Daily life |
| Pancawara | 5 days | Umanis, Paing, Pon, Wage, Keliwon | Market days |
| Sadwara | 6 days | Tungleh, Aryang, Urukung, Paniron, Was, Mawu | Ceremonial |

### Secondary Cycles

| Cycle | Length | Purpose |
|-------|--------|---------|
| Astawara | 8 days | Special ceremonies |
| Sangawara | 9 days | Agricultural timing |
| Dasawara | 10 days | Major temple festivals |

## Day Combinations

Each day in the Pawukon cycle has a unique combination of names from different cycles:

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;

println!("Day combinations:");
println!("  Saptawara: {}", date.saptawara());
println!("  Pancawara: {}", date.pancawara());
println!("  Sadwara: {}", date.sadwara());
println!("  Astawara: {}", date.astawara());
println!("  Sangawara: {}", date.sangawara());
println!("  Dasawara: {}", date.dasawara());
```

## Pawukon Weeks (Wuku)

The 210-day cycle is divided into 30 weeks, each called a **Wuku**:

### Wuku Names and Numbers

1. **Sinta** - Days 1-7
2. **Landep** - Days 8-14
3. **Ukir** - Days 15-21
4. **Kulantir** - Days 22-28
5. **Tolu** - Days 29-35
6. **Gumbreg** - Days 36-42
7. **Wariga** - Days 43-49
8. **Warigadian** - Days 50-56
9. **Julungwangi** - Days 57-63
10. **Sungsang** - Days 64-70
11. **Dunggulan** - Days 71-77
12. **Kuningan** - Days 78-84
13. **Langkir** - Days 85-91
14. **Medangsia** - Days 92-98
15. **Pujut** - Days 99-105
16. **Pahang** - Days 106-112
17. **Krulut** - Days 113-119
18. **Merakih** - Days 120-126
19. **Tambir** - Days 127-133
20. **Medangkungan** - Days 134-140
21. **Matal** - Days 141-147
22. **Uye** - Days 148-154
23. **Menail** - Days 155-161
24. **Parangbakat** - Days 162-168
25. **Bala** - Days 169-175
26. **Ugu** - Days 176-182
27. **Wayang** - Days 183-189
28. **Kelawu** - Days 190-196
29. **Dukut** - Days 197-203
30. **Watugunung** - Days 204-210

## Special Days in the Pawukon

### Galungan and Kuningan

The most important days in the Pawukon cycle:

- **Galungan**: Day 71 (Wuku Dunggulan, Wednesday Keliwon)
- **Kuningan**: Day 81 (Wuku Kuningan, Saturday Keliwon)

```rust
use balinese_calendar::BalineseDate;

// Find the next Galungan
let date = BalineseDate::from_ymd(2023, 1, 1)?;
if date.is_galungan() {
    println!("Today is Galungan!");
}

// Find the next Kuningan
if date.is_kuningan() {
    println!("Today is Kuningan!");
}
```

### Other Important Days

- **Tumpek**: Six ceremonial days every 35 days
- **Kajeng Kliwon**: Every 15 days when Pancawara and Saptawara align
- **Buda Cemeng**: Every 35 days (Wednesday Pon)

## Calculating Pawukon Days

The library provides efficient algorithms to calculate Pawukon positions:

```rust
use balinese_calendar::{BalineseDate, pawukon};

// Get the Pawukon day number (1-210)
let date = BalineseDate::from_ymd(2023, 1, 1)?;
let pawukon_day = date.pawukon_day();
println!("Pawukon day: {}", pawukon_day);

// Get the Wuku (week) number
let wuku = date.wuku();
println!("Wuku: {}", wuku);

// Get the Wuku name
let wuku_name = date.wuku_name();
println!("Wuku name: {}", wuku_name);
```

## Pawukon and Gregorian Calendar

The Pawukon cycle doesn't align with the Gregorian calendar, so the same Pawukon date will fall on different Gregorian dates each year.

### Example: Pawukon Day 1

- 2023: January 1, 2023
- 2024: August 18, 2024
- 2025: April 6, 2025

## Cultural Significance

The Pawukon cycle governs:

- **Temple ceremonies** (Odalan)
- **Life cycle rituals** (Manusa Yadnya)
- **Agricultural activities**
- **Traditional healing practices**
- **Artistic performances**

## Library Implementation

The Balinese Calendar library implements:

- Accurate Pawukon day calculations
- Wuku determination
- Special day detection (Galungan, Kuningan, etc.)
- Efficient algorithms for date conversion

```rust
use balinese_calendar::BalineseDate;

// Comprehensive Pawukon information
let date = BalineseDate::from_ymd(2023, 1, 1)?;

println!("Pawukon Information:");
println!("  Day number: {}", date.pawukon_day());
println!("  Week (Wuku): {} - {}", date.wuku(), date.wuku_name());
println!("  Is Galungan: {}", date.is_galungan());
println!("  Is Kuningan: {}", date.is_kuningan());
println!("  Is Tumpek: {}", date.is_tumpek());
```

## Historical Notes

The Pawukon cycle predates the arrival of Hinduism in Bali and has roots in ancient indigenous timekeeping systems. It was later integrated with Hindu calendar elements to create the complex system used today.

## Modern Adaptations

While traditionally used for religious purposes, the Pawukon cycle is now also used for:

- **Tourism planning** (avoiding ceremony days)
- **Business scheduling**
- **Government administrative planning**
- **Cultural preservation efforts**
