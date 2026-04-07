// tests/wariga_test.rs
//
// Tests for Wariga computation layer — traditional Balinese day quality and compatibility systems.

use balinese_calendar::{
    BalineseDate, DauhQuality, PawiwahanQuality, PramanaQuality, WarigaBelog, dauh_sukaranti,
    name_compatibility, next_otonan, next_otonan_from, otonan_dates, pawiwahan_compatibility,
    tri_pramana_for_day, wariga_belog,
};
use chrono::{NaiveDate, Utc};

#[test]
fn test_wariga_belog_basic() {
    // Test Wariga BELOG calculation with known dates
    let birth = BalineseDate::from_ymd(2000, 1, 1).unwrap();
    let query = BalineseDate::from_ymd(2024, 1, 1).unwrap();

    let result = wariga_belog(&birth, &query);

    // Should return one of the four variants
    match result {
        WarigaBelog::Pati | WarigaBelog::Guru | WarigaBelog::Ratu | WarigaBelog::Lara => {
            // Valid result
        }
    }

    // Test description method
    let desc = result.description();
    assert!(!desc.is_empty());
    assert!(desc.len() > 10); // Should have meaningful description
}

#[test]
fn test_wariga_belog_same_date() {
    // Test with same birth and query date
    let birth = BalineseDate::from_ymd(1990, 6, 15).unwrap();
    let query = BalineseDate::from_ymd(1990, 6, 15).unwrap();

    let result = wariga_belog(&birth, &query);

    // Should be deterministic
    assert_eq!(result, wariga_belog(&birth, &query));
}

#[test]
fn test_tri_pramana_lookup() {
    // Test Tri-Pramana lookup for different Pawukon days
    for day in 0..210u16 {
        let result = tri_pramana_for_day(day);

        // Should return Some result for valid days
        assert!(result.is_some(), "Failed for day {}", day);
        let tri_pramana = result.unwrap();

        // Should have valid urip (1-30)
        assert!(
            tri_pramana.urip >= 1 && tri_pramana.urip <= 30,
            "Invalid urip {} for day {}",
            tri_pramana.urip,
            day
        );

        // Should have valid quality
        match tri_pramana.quality {
            PramanaQuality::LungguhSakti
            | PramanaQuality::UtamaAsih
            | PramanaQuality::PugeranBakti
            | PramanaQuality::MuktiPapa => {
                // Valid quality
            }
        }

        // Test description method
        let desc = tri_pramana.quality.description();
        assert!(!desc.is_empty());
    }

    // Test out of bounds
    assert!(tri_pramana_for_day(210).is_none());
    assert!(tri_pramana_for_day(1000).is_none());
}

#[test]
fn test_tri_pramana_balinese_date_method() {
    // Test the tri_pramana method on BalineseDate
    let date = BalineseDate::from_ymd(2024, 3, 15).unwrap();
    let result = date.tri_pramana();

    // Should return Some result
    assert!(result.is_some());
    let tri_pramana = result.unwrap();

    // Should be consistent with direct lookup
    let expected = tri_pramana_for_day(date.pawukon_day);
    assert!(expected.is_some());
    let expected_tri_pramana = expected.unwrap();
    assert_eq!(tri_pramana.urip, expected_tri_pramana.urip);
    assert_eq!(tri_pramana.quality, expected_tri_pramana.quality);
}

#[test]
fn test_pawiwahan_compatibility() {
    // Test Pawiwahan marriage compatibility
    let date_a = BalineseDate::from_ymd(1990, 5, 10).unwrap();
    let date_b = BalineseDate::from_ymd(1992, 8, 22).unwrap();

    let result = pawiwahan_compatibility(&date_a, &date_b);

    // Should have valid combined urip
    assert!(result.combined_urip > 0);

    // Should have valid remainder (0-15)
    assert!(result.remainder < 16);

    // Should have valid quality
    match result.quality {
        PawiwahanQuality::MadyaSukaDuka
        | PawiwahanQuality::KawonLaraMiskin
        | PawiwahanQuality::KawonLaraWarang
        | PawiwahanQuality::KawonPanakeMati
        | PawiwahanQuality::BecikPisanSudhaNulus
        | PawiwahanQuality::KawonSengsara
        | PawiwahanQuality::MadyaSukaDuka2
        | PawiwahanQuality::KawonLaraKenapali
        | PawiwahanQuality::KawonPisanBayaPati
        | PawiwahanQuality::BecikBikigaRatuna
        | PawiwahanQuality::BecikKapardyaniyah
        | PawiwahanQuality::BecikKedrpingHari
        | PawiwahanQuality::BecikTanKirang
        | PawiwahanQuality::KawonTanPolihKeselamatan
        | PawiwahanQuality::BecikBokung
        | PawiwahanQuality::BecikNyamaBrayaAsih => {
            // Valid quality
        }
    }

    // Test description method
    let desc = result.quality.description();
    assert!(!desc.is_empty());

    // Test is_auspicious method
    let _is_good = result.quality.is_auspicious();
}

#[test]
fn test_pawiwahan_symmetry() {
    // Test that Pawiwahan compatibility is symmetric
    let date_a = BalineseDate::from_ymd(1985, 3, 12).unwrap();
    let date_b = BalineseDate::from_ymd(1988, 9, 30).unwrap();

    let result_ab = pawiwahan_compatibility(&date_a, &date_b);
    let result_ba = pawiwahan_compatibility(&date_b, &date_a);

    assert_eq!(result_ab.combined_urip, result_ba.combined_urip);
    assert_eq!(result_ab.remainder, result_ba.remainder);
    assert_eq!(result_ab.quality, result_ba.quality);
}

#[test]
fn test_dauh_sukaranti() {
    // Test Dauh Sukaranti time-slot qualities
    let urip_values = [1, 5, 10, 15, 20, 25, 30];

    for &urip in &urip_values {
        let qualities = dauh_sukaranti(urip);

        // Should return exactly 5 qualities
        assert_eq!(qualities.len(), 5);

        // All qualities should be valid
        for quality in qualities {
            match quality {
                DauhQuality::Kelara
                | DauhQuality::Pali
                | DauhQuality::Sume
                | DauhQuality::Krta
                | DauhQuality::Peta => {
                    // Valid quality
                }
            }

            // Test description method
            let desc = quality.description();
            assert!(!desc.is_empty());
        }
    }
}

#[test]
fn test_name_compatibility() {
    // Test name compatibility
    let name_a = "Made";
    let name_b = "Kadek";

    let result = name_compatibility(name_a, name_b);

    // Should have valid combined urip
    assert!(result.combined_urip > 0);

    // Should have valid remainder
    assert!(result.remainder < 7);

    // Should have boolean compatibility result
    let _is_compatible = result.is_compatible;
}

#[test]
fn test_name_compatibility_edge_cases() {
    // Test with empty names
    let result = name_compatibility("", "");
    assert!(result.combined_urip > 0);

    // Test with long names
    let long_name = "A".repeat(100);
    let result = name_compatibility(&long_name, "B");
    assert!(result.combined_urip > 0);
}

#[test]
fn test_otonan_calculator() {
    // Test Otonan calculator
    let birth = NaiveDate::from_ymd_opt(1990, 6, 15).unwrap();

    // Test next otonan from specific date (deterministic)
    let after = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let next_from = next_otonan_from(birth, after);
    assert!(next_from > after);

    // Should be a multiple of 210 days after birth
    let days_diff = (next_from - birth).num_days();
    assert!(days_diff % 210 == 0);
    assert!(days_diff > 0);

    // Test otonan dates calculation
    let dates = otonan_dates(birth, 3);
    assert_eq!(dates.len(), 3);

    // First should be birth + 210 days
    assert_eq!(dates[0], birth + chrono::Duration::days(210));

    // Should be sequential
    assert_eq!(dates[1], dates[0] + chrono::Duration::days(210));
    assert_eq!(dates[2], dates[1] + chrono::Duration::days(210));
}

#[test]
fn test_otonan_dates() {
    // Test multiple Otonan dates calculation
    let birth = NaiveDate::from_ymd_opt(1985, 12, 25).unwrap();
    let count = 5;

    let dates = otonan_dates(birth, count);

    // Should return exactly the requested number of dates
    assert_eq!(dates.len(), count);

    // All dates should be after birth date
    for (i, &date) in dates.iter().enumerate() {
        assert!(date > birth);

        // Should be exactly (i+1) * 210 days after birth
        let expected = birth + chrono::Duration::days((i as i64 + 1) * 210);
        assert_eq!(date, expected);
    }

    // Dates should be in chronological order
    for i in 1..dates.len() {
        assert!(dates[i] > dates[i - 1]);
    }
}

#[test]
fn test_otonan_edge_cases() {
    // Test edge case: birth date exactly on the reference date
    let birth = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
    let same_day = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
    let next_from_same = next_otonan_from(birth, same_day);
    assert!(next_from_same > same_day);

    // Should be exactly 210 days after birth
    assert_eq!(next_from_same, birth + chrono::Duration::days(210));

    // Test with birth date today
    let today = Utc::now().date_naive();
    let next_today = next_otonan(today);
    assert!(next_today > today);

    // Test zero count for otonan_dates
    let birth = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let empty_dates = otonan_dates(birth, 0);
    assert!(empty_dates.is_empty());

    // Test with birth date in the near future (should still work)
    let near_future = Utc::now().date_naive() + chrono::Duration::days(30);
    let next_near = next_otonan(near_future);
    // Note: next_otonan returns the first otonan after today, not necessarily after the birth date
    // So we just verify it returns a valid date
    assert!(next_near > today);
}

#[test]
fn test_wariga_descriptions() {
    // Test that all description methods return meaningful text
    let birth = BalineseDate::from_ymd(1995, 4, 10).unwrap();
    let query = BalineseDate::from_ymd(2024, 8, 20).unwrap();

    // Wariga BELOG descriptions
    let belog = wariga_belog(&birth, &query);
    assert!(!belog.description().is_empty());
    assert!(belog.description().len() > 5);

    // Tri-Pramana descriptions
    let tri_pramana_result = tri_pramana_for_day(105); // Middle of cycle
    assert!(tri_pramana_result.is_some());
    let tri_pramana = tri_pramana_result.unwrap();
    assert!(!tri_pramana.quality.description().is_empty());
    assert!(tri_pramana.quality.description().len() > 5);

    // Pawiwahan descriptions
    let pawiwahan = pawiwahan_compatibility(&birth, &query);
    assert!(!pawiwahan.quality.description().is_empty());
    assert!(pawiwahan.quality.description().len() > 5);

    // Dauh Sukaranti descriptions - use direct function call
    let dauh_qualities = balinese_calendar::dauh_sukaranti(10);
    for quality in dauh_qualities {
        assert!(!quality.description().is_empty());
        assert!(quality.description().len() > 3);
    }
}
