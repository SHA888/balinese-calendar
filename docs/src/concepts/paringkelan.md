# Paringkelan

Paringkelan is a unique 35-day cycle system in the Balinese calendar that groups days for agricultural and ceremonial purposes. It represents one of the many overlapping cycles that create the rich tapestry of Balinese timekeeping.

## Overview

The Paringkelan system:

- **Duration**: 35 days (5 weeks of 7 days each)
- **Structure**: 5 groups called "Paringkelan"
- **Purpose**: Agricultural timing and ceremonial planning
- **Integration**: Works with Pawukon and Wewaran systems

## The 5 Paringkelan Groups

### 1. Sri (1-7 days)
- **Characteristics**: Prosperity and abundance
- **Activities**: Planting, beginning new ventures
- **Ceremonies**: Blessing of seeds and tools
- **Color**: Yellow/gold

### 2. Laba (8-14 days)
- **Characteristics**: Growth and increase
- **Activities**: Field maintenance, nurturing
- **Ceremonies**: Growth blessings
- **Color**: Green

### 3. Jaya (15-21 days)
- **Characteristics**: Victory and success
- **Activities**: Harvesting, celebration
- **Ceremonies**: Thanksgiving rituals
- **Color**: Red

### 4. Mandala (22-28 days)
- **Characteristics**: Balance and harmony
- **Activities**: Planning, preparation
- **Ceremonies**: Balance and harmony rituals
- **Color**: Blue

### 5. Uduh (29-35 days)
- **Characteristics**: Completion and renewal
- **Activities**: Land preparation, rest
- **Ceremonies**: Purification and preparation
- **Color**: White

## Agricultural Applications

The Paringkelan cycle traditionally guides agricultural activities:

### Rice Cultivation Cycle

| Paringkelan | Activity | Traditional Practice |
|-------------|----------|----------------------|
| Sri (Days 1-7) | Land preparation | Clear fields, bless land |
| Laba (Days 8-14) | Planting | Sow rice seedlings |
| Jaya (Days 15-21) | Growth period | Field maintenance |
| Mandala (Days 22-28) | Harvesting | Harvest mature rice |
| Uduh (Days 29-35) | Field rest | Prepare for next cycle |

### Fruit Tree Care

- **Sri**: Pruning and fertilizing
- **Laba**: Flowering period care
- **Jaya**: Fruit development monitoring
- **Mandala**: Harvest planning
- **Uduh**: Tree maintenance and rest

## Ceremonial Significance

### Buda Cemeng

Every 35 days, when:
- **Saptawara**: Wednesday (Buda)
- **Pancawara**: Pon (Cemeng)
- **Paringkelan**: Specific alignment

This day is considered auspicious for:
- **Spiritual cleansing** rituals
- **Temple ceremonies** and offerings
- **Personal purification** practices

### Kajeng Kliwon and Paringkelan

When Kajeng Kliwon aligns with specific Paringkelan days:
- **Enhanced spiritual power** for ceremonies
- **Community gatherings** and festivals
- **Traditional healing** practices

## Library Implementation

The Balinese Calendar library provides Paringkelan support:

```rust
use balinese_calendar::BalineseDate;

let date = BalineseDate::from_ymd(2023, 1, 1)?;

// Get Paringkelan information
let paringkelan = date.paringkelan();
let paringkelan_name = date.paringkelan_name();
let day_in_paringkelan = date.day_in_paringkelan();

println!("Paringkelan Information:");
println!("  Group: {}", paringkelan);
println!("  Name: {}", paringkelan_name);
println!("  Day in cycle: {}", day_in_paringkelan);

// Check for special days
if date.is_buda_cemeng() {
    println!("Today is Buda Cemeng - auspicious for ceremonies!");
}
```

## Paringkelan Calculations

```rust
use balinese_calendar::BalineseDate;

// Calculate Paringkelan position
let date = BalineseDate::from_ymd(2023, 1, 1)?;
let cycle_day = date.day_in_paringkelan();
let group = date.paringkelan();

println!("Day {} of Paringkelan cycle", cycle_day);
println!("Current group: {}", date.paringkelan_name());

// Check group characteristics
match group {
    balinese_calendar::paringkelan::Paringkelan::Sri => {
        println!("Time for planting and new beginnings");
    }
    balinese_calendar::paringkelan::Paringkelan::Jaya => {
        println!("Time for harvest and celebration");
    }
    _ => println!("Regular Paringkelan day"),
}
```

## Integration with Other Cycles

The Paringkelan works alongside:

### With Pawukon
- **Pawukon**: 210-day cycle
- **Paringkelan**: 35-day cycle
- **LCM**: 210 days (6 Paringkelan cycles)

### With Wewaran
- **Saptawara**: 7-day week
- **Paringkelan**: 35 days (5 × 7)
- **Alignment**: Every Paringkelan contains complete weeks

## Cultural Context

### Traditional Agriculture

- **Subak system** (traditional irrigation)
- **Rice terrace management**
- **Seasonal crop rotation**
- **Community cooperation** (gotong royong)

### Modern Applications

- **Organic farming** schedules
- **Sustainable agriculture** planning
- **Cultural tourism** activities
- **Educational programs**

## Regional Variations

Different regions in Bali may have variations:

### North Bali
- Emphasis on **dry season agriculture**
- Different **ceremonial timing**
- Local **planting schedules**

### South Bali
- Focus on **wet rice cultivation**
- More elaborate **ceremonies**
- **Tourism-influenced** practices

### East Bali
- **Traditional practices** maintained
- **Less tourism influence**
- **Community-focused** ceremonies

## Spiritual Significance

The Paringkelan cycle represents:

- **Natural cycles** of growth and rest
- **Spiritual purification** and renewal
- **Community harmony** and cooperation
- **Balance** between human activity and nature

## Modern Usage

While traditional agriculture has evolved, Paringkelan is still used for:

- **Ceremonial planning** and scheduling
- **Cultural preservation** efforts
- **Educational purposes** and cultural learning
- **Spiritual practices** and personal development
- **Agricultural consulting** and traditional wisdom
