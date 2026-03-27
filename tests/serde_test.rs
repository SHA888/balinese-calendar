//! Test serde serialization/deserialization functionality

#[cfg(feature = "serde")]
use balinese_calendar::{BalineseDate, DayBoundary, FlatRecord, Rahinan, Sasih, Wuku};

#[cfg(feature = "serde")]
#[test]
fn test_balinese_date_serde() {
    let date = BalineseDate::from_ymd(2026, 3, 26).unwrap();

    // Test serialization to JSON
    let json = serde_json::to_string(&date).expect("Failed to serialize BalineseDate");
    println!("Serialized BalineseDate: {}", json);

    // Test deserialization
    let deserialized: BalineseDate =
        serde_json::from_str(&json).expect("Failed to deserialize BalineseDate");
    assert_eq!(date, deserialized);
}

#[cfg(feature = "serde")]
#[test]
fn test_flat_record_serde() {
    let date = BalineseDate::from_ymd(2026, 3, 26).unwrap();
    let flat = date.to_flat_record();

    // Test serialization to JSON
    let json = serde_json::to_string(&flat).expect("Failed to serialize FlatRecord");
    println!("Serialized FlatRecord: {}", json);

    // Note: FlatRecord contains &'static str fields, so deserialization requires custom handling
    // For now, we just test that serialization works
    assert!(json.contains("gregorian_year"));
    assert!(json.contains("sasih_name"));
}

#[cfg(feature = "serde")]
#[test]
fn test_enums_serde() {
    // Test enum serialization
    let sasih = Sasih::Kasanga;
    let json = serde_json::to_string(&sasih).expect("Failed to serialize Sasih");
    let deserialized: Sasih = serde_json::from_str(&json).expect("Failed to deserialize Sasih");
    assert_eq!(sasih, deserialized);

    let wuku = Wuku::Sinta;
    let json = serde_json::to_string(&wuku).expect("Failed to serialize Wuku");
    let deserialized: Wuku = serde_json::from_str(&json).expect("Failed to deserialize Wuku");
    assert_eq!(wuku, deserialized);

    let rahinan = Rahinan::Galungan;
    let json = serde_json::to_string(&rahinan).expect("Failed to serialize Rahinan");
    let deserialized: Rahinan = serde_json::from_str(&json).expect("Failed to deserialize Rahinan");
    assert_eq!(rahinan, deserialized);

    let boundary = DayBoundary::FixedSunrise(6);
    let json = serde_json::to_string(&boundary).expect("Failed to serialize DayBoundary");
    let deserialized: DayBoundary =
        serde_json::from_str(&json).expect("Failed to deserialize DayBoundary");
    assert_eq!(boundary, deserialized);
}
