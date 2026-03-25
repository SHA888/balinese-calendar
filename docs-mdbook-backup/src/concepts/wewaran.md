# Wewaran Cycles

Wewaran cycles are the various week-based systems that operate simultaneously within the Balinese calendar. These cycles create the unique day combinations that determine auspicious and inauspicious times.

## Overview

The Balinese calendar uses multiple week systems that run concurrently:

- **Saptawara** (7-day) - Most commonly used
- **Pancawara** (5-day) - Market and ceremonial days
- **Sadwara** (6-day) - Special ceremonies
- **Astawara** (8-day) - Agricultural timing
- **Sangawara** (9-day) - Rare ceremonial purposes
- **Dasawara** (10-day) - Major temple festivals

## Saptawara (7-day Week)

The most important week system, corresponding to the Western week:

### Day Names

| Balinese | English | Sanskrit Origin |
|----------|---------|----------------|
| Redite | Sunday | Ravi-vara |
| Soma | Monday | Soma-vara |
| Anggara | Tuesday | Angaraka-vara |
| Buda | Wednesday | Budha-vara |
| Wraspati | Thursday | Brihaspati-vara |
| Sukra | Friday | Sukra-vara |
| Saniscara | Saturday | Sani-vara |

### Usage

- **Daily life** and work schedules
- **Market days** in traditional villages
- **Personal activities** and social events

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;
println!("Saptawara: {}", date.saptawara()); // Output: Redite
```

## Pancawara (5-day Week)

The second most important week system:

### Day Names

1. **Umanis** - "Sweet" - Good for positive activities
2. **Paing** - "Less" - Neutral day
3. **Pon** - "Middle" - Balanced day
4. **Wage** - "Unequal" - Challenging day
5. **Keliwon** - "Clear" - Most spiritually significant

### Usage

- **Traditional markets** (pasar) operate on 5-day cycles
- **Ceremonial timing** and offerings
- **Agricultural activities**

### Keliwon Significance

Keliwon days are particularly important:
- **Kajeng Kliwon**: When Keliwon coincides with certain Saptawara days
- **Spiritual cleansing** and purification rituals
- **Temple ceremonies** often scheduled for Keliwon

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;
println!("Pancawara: {}", date.pancawara()); // Output: Umanis

// Check if it's a Keliwon day
if date.pancawara().is_keliwon() {
    println!("Today is Keliwon - spiritually significant!");
}
```

## Sadwara (6-day Week)

Used for specific ceremonial purposes:

### Day Names

1. **Tungleh** - Beginning/creation
2. **Aryang** - Protection
3. **Urukung** - Prosperity
4. **Paniron** - Judgment
5. **Was** - Destruction
6. **Mawu** - Creator

### Usage

- **Life cycle ceremonies** (birth, marriage, death)
- **Temple anniversary celebrations** (Odalan)
- **Healing rituals** and traditional medicine

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;
println!("Sadwara: {}", date.sadwara()); // Output: Tungleh
```

## Astawara (8-day Week)

Less commonly used but still important:

### Day Names

1. **Sri** - Goddess of prosperity
2. **Indra** - King of gods
3. **Guru** - Teacher/planet Jupiter
4. **Yama** - God of death
5. **Rudra** - Fierce aspect of Shiva
6. **Sura** - Divine/angelic
7. **Dadhi** - Nourishment
8. **Rana** - Battle/war

### Usage

- **Agricultural planning** and harvesting
- **Building construction** and foundation laying
- **Business ventures** and investments

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;
println!("Astawara: {}", date.astawara()); // Output: Sri
```

## Sangawara (9-day Week)

Rarely used in modern practice:

### Day Names

1. **Dangu** - Old/ancient
2. **Jangur** - Young/new
3. **Gigis** - Sharp/keen
4. **Nohan** - Hidden
5. **Ogan** - Powerful
6. **Erangan** - Passionate
7. **Urungan** - Elevated
8. **Tulus** - Pure/sincere
9. **Dadi** - Becoming/manifest

### Usage

- **Traditional healing** practices
- **Spiritual consultations**
- **Rare ceremonial occasions**

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;
println!("Sangawara: {}", date.sangawara()); // Output: Dangu
```

## Dasawara (10-day Week)

Used for major temple festivals:

### Day Names

1. **Pandita** - Priest/scholar
2. **Pati** - Death/control
3. **Suka** - Happiness
4. **Duka** - Sorrow
5. **Sri** - Prosperity
6. **Manuh** - Human being
7. **Manusa** - Mankind
8. **Raja** - King/royal
9. **Dewa** - God/deity
10. **Raksasa** - Demon/giant

### Usage

- **Major temple festivals** (odalan agung)
- **Royal ceremonies** and state functions
- **Important religious holidays**

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;
println!("Dasawara: {}", date.dasawara()); // Output: Pandita
```

## Day Combinations

The power of the Wewaran system comes from the unique combinations of different week cycles:

### Example: January 1, 2023

- **Saptawara**: Redite (Sunday)
- **Pancawara**: Umanis
- **Sadwara**: Tungleh
- **Astawara**: Sri
- **Sangawara**: Dangu
- **Dasawara**: Pandita

This combination creates the specific character and auspiciousness of the day.

## Special Combinations

### Kajeng Kliwon

When Pancawara Keliwon coincides with:
- **Saptawara**: Tuesday, Thursday, Saturday
- **Sadwara**: Paniron or Was

These are particularly powerful spiritual days.

### Tumpek Days

Six special days every 35 days when:
- **Pancawara**: Keliwon
- **Saptawara**: Saturday

Each Tumpek has a specific focus:
- **Tumpek Landep**: Metal objects and tools
- **Tumpek Wariga**: Plants and vegetation
- **Tumpek Kuningan**: Ancestors and spirits
- **Tumpek Krulut**: Traditional arts and music
- **Tumpek Wayang**: Wayang puppets and performing arts
- **Tumpek Uduh**: Fruit trees and agriculture

## Cultural Significance

The Wewaran cycles determine:

- **Daily activities** and work schedules
- **Ceremonial timing** and temple festivals
- **Personal auspicious days** for important events
- **Agricultural cycles** and harvesting times
- **Market days** and commercial activities

## Library Implementation

The Balinese Calendar library provides comprehensive access to all Wewaran systems:

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;

// Get all Wewaran information
println!("Complete Wewaran for {}:", date);
println!("  Saptawara (7-day): {}", date.saptawara());
println!("  Pancawara (5-day): {}", date.pancawara());
println!("  Sadwara (6-day): {}", date.sadwara());
println!("  Astawara (8-day): {}", date.astawara());
println!("  Sangawara (9-day): {}", date.sangawara());
println!("  Dasawara (10-day): {}", date.dasawara());

// Check special combinations
if date.is_kajeng_kliwon() {
    println!("Today is Kajeng Kliwon!");
}

if date.is_tumpek() {
    println!("Today is Tumpek!");
}
```

## Modern Usage

While traditionally used for religious and agricultural purposes, Wewaran cycles are now also considered for:

- **Business planning** and scheduling
- **Personal development** activities
- **Cultural tourism** and event planning
- **Digital calendar applications** and reminders
