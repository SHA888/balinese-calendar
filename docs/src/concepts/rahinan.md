# Rahinan (Holy Days)

Rahinan are the sacred and holy days in the Balinese calendar system. These days are determined by complex intersections of various calendar cycles and hold profound religious and cultural significance for the Balinese people.

## Overview

Rahinan are determined by:

- **Pawukon cycle** intersections
- **Wewaran combinations**
- **Sasih transitions**
- **Astronomical events**
- **Cultural traditions**

## Major Holy Days

### Galungan

The most important holy day in the Balinese calendar:

- **Frequency**: Every 210 days (Pawukon cycle)
- **Timing**: Wednesday Keliwon of Wuku Dunggulan
- **Significance**: Victory of dharma (good) over adharma (evil)
- **Duration**: 3 days (Galungan, Manis Galungan, Umanis Galungan)

#### Galungan Celebrations
- **Penjor**: Decorated bamboo poles in front of houses
- **Offerings**: Special offerings (banten) for ancestors
- **Temple visits**: Community prayers and ceremonies
- **Family gatherings**: Reunions and feasting

```rust
use balinese_calendar::BalineseDate;

// Check for Galungan
let date = BalineseDate::from_ymd(2023, 1, 1)?;
if date.is_galungan() {
    println!("Today is Galungan! Time for celebration.");
}

// Find next Galungan
let next_galungan = date.next_galungan();
println!("Next Galungan: {} days from now", next_galungan);
```

### Kuningan

Comes 10 days after Galungan:

- **Frequency**: Every 210 days
- **Timing**: Saturday Keliwon of Wuku Kuningan
- **Significance**: Ancestor spirits return to heavens
- **Duration**: 1 day

#### Kuningan Observances
- **Yellow offerings**: Symbolizing prosperity
- **Kuningan water**: Purification rituals
- **Temple ceremonies**: Final prayers for ancestors
- **Community cleansing**: Spiritual purification

### Nyepi (Balinese New Year)

The day of silence and reflection:

- **Frequency**: Once per Saka year
- **Timing**: New moon of Sasih Kasa
- **Significance**: New year purification
- **Duration**: 24 hours of silence

#### Nyepi Observances (Catur Brata Penyepian)
1. **Amati Geni**: No fire or light
2. **Amati Karya**: No work or activity
3. **Amati Lelungan**: No travel
4. **Amati Lelanguan**: No entertainment or feasting

#### Nyepi Preparation
- **Melasti**: Purification ceremony (3 days before)
- **Tawur Kesanga**: Exorcism rituals (eve of Nyepi)
- **Ogoh-ogoh**: Demon effigy parades

```rust
use balinese_calendar::BalineseDate;

// Check for Nyepi
let date = BalineseDate::from_ymd(2023, 3, 22)?;
if date.is_nyepi() {
    println!("Today is Nyepi - Day of silence.");
}

// Get Nyepi information
if date.is_nyepi_eve() {
    println!("Tonight is Nyepi eve - Ogoh-ogoh parade!");
}
```

## Minor Holy Days

### Saraswati Day

Day of knowledge and learning:

- **Frequency**: Every 210 days
- **Timing**: Saturday of Wuku Watugunung
- **Significance**: Honoring Goddess Saraswati
- **Activities**: Book blessings, learning ceremonies

### Pagerwesi

Day of spiritual protection:

- **Frequency**: Every 210 days
- **Timing**: Wednesday of Wuku Sinta
- **Significance**: Strengthening spiritual defenses
- **Activities**: Temple prayers, protection rituals

### Tumpek Ceremonies

Six ceremonial days every 35 days:

| Tumpek | Focus | Activities |
|--------|-------|------------|
| Tumpek Landep | Metal objects/tools | Blessing of weapons, vehicles, tools |
| Tumpek Wariga | Plants/vegetation | Blessing of gardens, rice fields |
| Tumpek Kuningan | Ancestors/spirits | Ancestor worship and offerings |
| Tumpek Krulut | Arts/music | Traditional performances and blessings |
| Tumpek Wayang | Wayang puppets | Puppet shows and storytelling |
| Tumpek Uduh | Fruit trees | Tree blessings and harvest prayers |

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;

// Check for Tumpek
if let Some(tumpek_type) = date.tumpek_type() {
    println!("Today is Tumpek {:?}", tumpek_type);
    match tumpek_type {
        balinese_calendar::Tumpek::Landep => {
            println!("Time to bless metal objects and tools!");
        }
        balinese_calendar::Tumpek::Wariga => {
            println!("Time to bless plants and gardens!");
        }
        _ => println!("Other Tumpek ceremony"),
    }
}
```

## Special Combinations

### Kajeng Kliwon

Spiritually powerful days:

- **Frequency**: Every 15 days
- **Condition**: Pancawara Keliwon + specific Saptawara
- **Significance**: Enhanced spiritual energy
- **Activities**: Purification, offerings, meditation

### Buda Cemeng

Auspicious ceremonial days:

- **Frequency**: Every 35 days
- **Condition**: Wednesday (Buda) + Pon (Cemeng)
- **Significance**: Balance and harmony
- **Activities**: Ceremonies, blessings, community events

### Siwa Ratri

Night of Shiva:

- **Frequency**: Once per year
- **Timing**: 7th day of waxing moon in Sasih Kapitu
- **Significance**: Spiritual purification and meditation
- **Activities**: All-night meditation and prayers

## Library Implementation

The Balinese Calendar library provides comprehensive Rahinan support:

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;

// Check for all major holy days
println!("Holy Day Information for {}:", date);
println!("  Is Galungan: {}", date.is_galungan());
println!("  Is Kuningan: {}", date.is_kuningan());
println!("  Is Nyepi: {}", date.is_nyepi());
println!("  Is Saraswati: {}", date.is_saraswati());
println!("  Is Pagerwesi: {}", date.is_pagerwesi());
println!("  Is Kajeng Kliwon: {}", date.is_kajeng_kliwon());
println!("  Is Buda Cemeng: {}", date.is_buda_cemeng());
println!("  Is Tumpek: {}", date.is_tumpek());

// Get specific information
if let Some(holy_day) = date.get_holy_day() {
    println!("Today is: {:?}", holy_day);
    println!("Significance: {}", holy_day.description());
    println!("Recommended activities: {:?}", holy_day.activities());
}
```

## Regional Variations

### North Bali
- **Different emphasis** on certain ceremonies
- **Local deities** and traditions
- **Variations** in timing and practices

### South Bali
- **More elaborate** ceremonies
- **Tourism influence** on celebrations
- **Urban adaptations** of traditional practices

### East Bali
- **Stricter adherence** to traditions
- **Less tourism** influence
- **Community-focused** celebrations

## Modern Observances

### Contemporary Practices

- **Digital calendars** with holy day reminders
- **Social media** announcements and sharing
- **Tourism-friendly** ceremony schedules
- **Educational programs** for visitors

### Challenges

- **Tourism pressure** on traditional practices
- **Modern work schedules** conflicting with ceremonies
- **Urbanization** affecting community participation
- **Globalization** influencing traditional values

## Cultural Significance

Rahinan maintain:

- **Spiritual connection** to ancestors and deities
- **Community cohesion** through shared celebrations
- **Cultural identity** and tradition preservation
- **Moral guidance** through religious teachings
- **Environmental awareness** through nature-based ceremonies

## Future of Rahinan

### Preservation Efforts

- **Documentation** of traditional practices
- **Education programs** for younger generations
- **Cultural centers** and workshops
- **Digital archives** and resources

### Adaptation

- **Flexible scheduling** for modern life
- **Virtual participation** options
- **Interfaith dialogue** and understanding
- **Sustainable practices** for ceremonies
