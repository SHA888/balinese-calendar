use balinese_calendar::{BalineseDate, Sasih};

#[test]
fn test_pancaroba_season_tags() {
    // Test that only Kalima and Kanem are marked as pancaroba
    assert_eq!(Sasih::Kalima.season_tag(), "pancaroba");
    assert_eq!(Sasih::Kanem.season_tag(), "pancaroba");

    // Test that all other sasih are NOT marked as pancaroba
    assert_ne!(Sasih::Kasa.season_tag(), "pancaroba");
    assert_ne!(Sasih::Karo.season_tag(), "pancaroba");
    assert_ne!(Sasih::Katiga.season_tag(), "pancaroba");
    assert_ne!(Sasih::Kapat.season_tag(), "pancaroba");
    assert_ne!(Sasih::Kapitu.season_tag(), "pancaroba");
    assert_ne!(Sasih::Kawolu.season_tag(), "pancaroba");
    assert_ne!(Sasih::Kasanga.season_tag(), "pancaroba");
    assert_ne!(Sasih::Kadasa.season_tag(), "pancaroba");
    assert_ne!(Sasih::Desta.season_tag(), "pancaroba");
    assert_ne!(Sasih::Sada.season_tag(), "pancaroba");
}

#[test]
fn test_is_pancaroba() {
    // Test that only Kalima and Kanem return true for is_pancaroba()
    assert!(Sasih::Kalima.is_pancaroba());
    assert!(Sasih::Kanem.is_pancaroba());

    // Test that all other sasih return false for is_pancaroba()
    assert!(!Sasih::Kasa.is_pancaroba());
    assert!(!Sasih::Karo.is_pancaroba());
    assert!(!Sasih::Katiga.is_pancaroba());
    assert!(!Sasih::Kapat.is_pancaroba());
    assert!(!Sasih::Kapitu.is_pancaroba());
    assert!(!Sasih::Kawolu.is_pancaroba());
    assert!(!Sasih::Kasanga.is_pancaroba());
    assert!(!Sasih::Kadasa.is_pancaroba());
    assert!(!Sasih::Desta.is_pancaroba());
    assert!(!Sasih::Sada.is_pancaroba());
}

#[test]
fn test_pancaroba_flag_in_balinese_date() {
    // Test dates in Sasih Kalima (pancaroba)
    // October 2026 should be in Sasih Kalima based on academic sources
    let d = BalineseDate::from_ymd(2026, 10, 15).unwrap();
    assert_eq!(d.sasih, Sasih::Kalima);
    assert!(d.to_flat_record().pancaroba_flag);

    // Test dates in Sasih Kanem (pancaroba)
    // November 2026 should be in Sasih Kanem based on academic sources
    let d = BalineseDate::from_ymd(2026, 11, 15).unwrap();
    assert_eq!(d.sasih, Sasih::Kanem);
    assert!(d.to_flat_record().pancaroba_flag);

    // Test dates NOT in pancaroba (dry season)
    let d = BalineseDate::from_ymd(2026, 7, 15).unwrap();
    assert!(!matches!(d.sasih, Sasih::Kalima | Sasih::Kanem));
    assert!(!d.to_flat_record().pancaroba_flag);

    // Test dates NOT in pancaroba (wet season)
    let d = BalineseDate::from_ymd(2026, 1, 15).unwrap();
    assert!(!matches!(d.sasih, Sasih::Kalima | Sasih::Kanem));
    assert!(!d.to_flat_record().pancaroba_flag);
}

#[test]
fn test_season_mapping_completeness() {
    // Test that all sasih have valid season tags
    let all_seasons = vec![
        (Sasih::Kasa, "dry"),
        (Sasih::Karo, "dry"),
        (Sasih::Katiga, "dry"),
        (Sasih::Kapat, "dry"),
        (Sasih::Kalima, "pancaroba"),
        (Sasih::Kanem, "pancaroba"),
        (Sasih::Kapitu, "wet"),
        (Sasih::Kawolu, "wet"),
        (Sasih::Kasanga, "wet"),
        (Sasih::Kadasa, "wet"),
        (Sasih::Desta, "dry"),
        (Sasih::Sada, "dry"),
    ];

    for (sasih, expected_season) in all_seasons {
        assert_eq!(
            sasih.season_tag(),
            expected_season,
            "Sasih {:?} should have season tag '{}'",
            sasih,
            expected_season
        );
    }
}
