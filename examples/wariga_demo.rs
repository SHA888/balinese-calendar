// examples/wariga_demo.rs
//
// Demonstration of Wariga computation layer features.

use balinese_calendar::{
    BalineseDate, DauhQuality, PawiwahanQuality, WarigaBelog, name_compatibility, next_otonan,
    otonan_dates, pawiwahan_compatibility, wariga_belog,
};
use chrono::NaiveDate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Balinese Calendar Wariga Computation Layer Demo ===\n");

    // Create some test dates
    let birth_date = BalineseDate::from_ymd(1990, 6, 15)?;
    let query_date = BalineseDate::from_ymd(2024, 3, 15)?;
    let partner_date = BalineseDate::from_ymd(1992, 8, 22)?;

    println!("Birth Date: {}", birth_date.to_balinese_string());
    println!("Query Date: {}", query_date.to_balinese_string());
    println!("Partner Date: {}\n", partner_date.to_balinese_string());

    // 1. Wariga BELOG - Personalized day quality
    println!("1. Wariga BELOG (Personalized Day Quality)");
    println!("-------------------------------------------");
    let belog_result = wariga_belog(&birth_date, &query_date);
    println!("Result: {:?}", belog_result);
    println!("Description: {}", belog_result.description());
    println!(
        "Interpretation: {} {}\n",
        match belog_result {
            WarigaBelog::Pati => "⚠️  Avoid major activities",
            WarigaBelog::Guru => "📚 Good for learning/spiritual practice",
            WarigaBelog::Ratu => "👑 Good for leadership/official matters",
            WarigaBelog::Lara => "🚫 Avoid important undertakings",
        },
        if belog_result.description().contains("danger")
            || belog_result.description().contains("suffering")
        {
            "(inauspicious)"
        } else {
            "(auspicious)"
        }
    );

    // 2. Tri-Pramana - Composite urip values
    println!("2. Tri-Pramana (Composite Urip Values)");
    println!("--------------------------------------");
    let tri_pramana_result = query_date.tri_pramana();

    match tri_pramana_result {
        Some(tri_pramana) => {
            println!("Pawukon Day: {}", query_date.pawukon_day);
            println!("Tri-Pramana Urip: {}", tri_pramana.urip);
            println!("Quality: {:?}", tri_pramana.quality);
            println!("Description: {}", tri_pramana.quality.description());
            println!(
                "Interpretation: {}",
                match tri_pramana.quality {
                    balinese_calendar::PramanaQuality::LungguhSakti =>
                        "🔨 Auspicious for crafting, practical work",
                    balinese_calendar::PramanaQuality::UtamaAsih =>
                        "✨ Excellent for all good works",
                    balinese_calendar::PramanaQuality::PugeranBakti =>
                        "🙏 Favourable for worship, devotion",
                    balinese_calendar::PramanaQuality::MuktiPapa =>
                        "⚠️  Inauspicious, risk of danger",
                }
            );
        }
        None => {
            println!("Error: Unable to calculate Tri-Pramana (invalid Pawukon day)");
        }
    }

    // 3. Pawiwahan - Marriage compatibility
    println!("3. Pawiwahan (Marriage Compatibility)");
    println!("------------------------------------");
    let pawiwahan_result = pawiwahan_compatibility(&birth_date, &partner_date);
    println!("Combined Urip: {}", pawiwahan_result.combined_urip);
    println!("Remainder: {}", pawiwahan_result.remainder);
    println!("Quality: {:?}", pawiwahan_result.quality);
    println!("Description: {}", pawiwahan_result.quality.description());
    println!(
        "Auspicious: {}",
        if pawiwahan_result.quality.is_auspicious() { "✅ Yes" } else { "❌ No" }
    );
    let level_str = match pawiwahan_result.quality {
        PawiwahanQuality::BecikPisanSudhaNulus => "5 (Excellent)",
        PawiwahanQuality::KawonPisanBayaPati => "9 (Worst)",
        PawiwahanQuality::BecikNyamaBrayaAsih => "16 (Excellent)",
        _ => {
            let fallback = format!("{:?}", pawiwahan_result.quality as u8);
            Box::leak(fallback.into_boxed_str())
        }
    };
    println!("Level: {}/16\n", level_str);

    // 4. Dauh Sukaranti - Time-slot quality
    println!("4. Dauh Sukaranti (Time-Slot Quality)");
    println!("-------------------------------------");
    let combined_urip = query_date.saptawara.urip() + query_date.pancawara.urip();
    println!("Combined Urip (Sapta + Panca): {}", combined_urip);
    let dauh_qualities = balinese_calendar::dauh_sukaranti(combined_urip);

    let periods = [
        "I (05:30–07:55)",
        "II (07:55–10:25)",
        "III (10:20–12:45)",
        "IV (12:45–15:10)",
        "V (15:10–17:30)",
    ];
    for (i, quality) in dauh_qualities.iter().enumerate() {
        let symbol = match quality {
            DauhQuality::Krta => "✨",
            DauhQuality::Sume => "✅",
            DauhQuality::Peta => "⚪",
            DauhQuality::Pali => "⚠️",
            DauhQuality::Kelara => "❌",
        };
        println!("Dauh {}: {:?} - {} {}", periods[i], quality, symbol, quality.description());
    }
    println!();

    // 5. Name Compatibility
    println!("5. Tenung Patemuan Adan (Name Compatibility)");
    println!("--------------------------------------------");
    let name_a = "Made";
    let name_b = "Kadek";
    let name_result = name_compatibility(name_a, name_b);
    println!("Name A: \"{}\"", name_a);
    println!("Name B: \"{}\"", name_b);
    println!("Combined Urip: {}", name_result.combined_urip);
    println!("Remainder: {}", name_result.remainder);
    println!("Compatible: {}\n", if name_result.is_compatible { "✅ Yes" } else { "❌ No" });

    // 6. Otonan Calculator
    println!("6. Otonan Calculator (210-Day Birthday Cycle)");
    println!("---------------------------------------------");
    let birth_naive = NaiveDate::from_ymd_opt(1990, 6, 15).unwrap();
    let next_otonan = next_otonan(birth_naive);
    let today = chrono::Utc::now().date_naive();

    println!("Birth Date: {}", birth_naive);
    println!("Today: {}", today);
    println!("Next Otonan: {}", next_otonan);
    println!("Days until next Otonan: {}", (next_otonan - today).num_days());

    println!("\nNext 3 Otonan dates:");
    let future_otonan = otonan_dates(birth_naive, 3);
    for (i, date) in future_otonan.iter().enumerate() {
        println!("  Otonan {}: {} (in {} days)", i + 1, date, (*date - today).num_days());
    }

    println!("\n=== Demo Complete ===");
    Ok(())
}
