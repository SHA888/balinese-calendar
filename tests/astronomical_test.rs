//! Test astronomical sunrise functionality

#[cfg(feature = "astronomical")]
mod tests {
    use balinese_calendar::{BalineseDate, DayBoundary};

    #[test]
    fn test_astronomical_sunrise_bali_centroid() {
        // Test with Bali centroid coordinates
        let boundary = DayBoundary::Astronomical { lat: -8.3405, lon: 115.0920 };

        // Test creating a date with astronomical boundary
        let result = BalineseDate::from_ymd_with_boundary(2026, 3, 26, &boundary);

        assert!(
            result.is_ok(),
            "Should successfully create Balinese date with astronomical sunrise: {result:?}"
        );

        let date = result.unwrap();
        println!("Balinese date with astronomical sunrise: {}", date.to_balinese_string());
        println!("Saka year: {}", date.saka_year);
        println!("Sasih: {}", date.sasih.name());
        println!("Wuku: {}", date.wuku.name());
    }

    #[test]
    fn test_astronomical_sunrise_different_coordinates() {
        // Test with different coordinates (e.g., Jakarta)
        let boundary = DayBoundary::Astronomical { lat: -6.2088, lon: 106.8456 };

        let result = BalineseDate::from_ymd_with_boundary(2026, 3, 26, &boundary);

        assert!(
            result.is_ok(),
            "Should successfully create Balinese date with Jakarta coordinates: {result:?}"
        );

        let date = result.unwrap();
        println!("Balinese date (Jakarta coordinates): {}", date.to_balinese_string());
    }

    #[test]
    fn test_astronomical_sunrise_edge_cases() {
        // Test with extreme coordinates
        let boundary = DayBoundary::Astronomical {
            lat: 90.0, // North Pole
            lon: 0.0,
        };

        let result = BalineseDate::from_ymd_with_boundary(2026, 6, 21, &boundary);

        // This might fail due to polar day/night conditions
        match result {
            Ok(date) => println!("North Pole summer: {}", date.to_balinese_string()),
            Err(e) => println!("North Pole summer expected failure: {e}"),
        }
    }

    #[test]
    fn test_astronomical_vs_fixed_sunrise() {
        // Compare astronomical sunrise with fixed sunrise
        let astronomical_boundary = DayBoundary::Astronomical { lat: -8.3405, lon: 115.0920 };
        let fixed_boundary = DayBoundary::FixedSunrise(6);

        let astronomical_date =
            BalineseDate::from_ymd_with_boundary(2026, 3, 26, &astronomical_boundary).unwrap();
        let fixed_date =
            BalineseDate::from_ymd_with_boundary(2026, 3, 26, &fixed_boundary).unwrap();

        println!("Astronomical: {}", astronomical_date.to_balinese_string());
        println!("Fixed (6h): {}", fixed_date.to_balinese_string());

        // They might be different depending on actual sunrise time
        // This test mainly verifies both work without panicking
    }

    #[test]
    fn test_astronomical_sunrise_wasm_compatibility() {
        // Test that astronomical boundary works with WASM functions when both features are enabled
        #[cfg(all(feature = "wasm", feature = "astronomical"))]
        {
            use balinese_calendar::wasm;

            // Test WASM astronomical functions
            let result = wasm::from_ymd_astronomical(2026, 3, 26, -8.3405, 115.0920);
            assert!(result.is_ok(), "WASM astronomical function should work");

            let wasm_date = result.unwrap();
            println!("WASM astronomical date: {}", wasm_date.to_balinese_string());
        }
    }
}
