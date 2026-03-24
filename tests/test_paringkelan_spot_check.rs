// Paringkelan spot-check tests against Aryana (edysantosa) canonical names
// Source: I.B. Putra Manik Aryana, Dasar Wariga + Tenung Wariga; I.B. Supartha Ardana, Pokok-Pokok Wariga (2005)
// Expected names:
// - Watek Alit (4): Uler, Gajah, Lembu, Lintah
// - Watek Madya (5): Gajah, Watu, Buta, Suku, Wong
// - Lintang (35): Gajah through Pucang (full 35-cycle)

use balinese_calendar::BalineseDate;
use chrono::Datelike;

#[test]
fn watek_alit_cycle() {
    // Verify the 4-cycle produces the expected canonical names
    // Using correct epoch PAWUKON_EPOCH_JDN = 2440976
    let d = BalineseDate::from_ymd(2026, 1, 1).unwrap(); // pawukon_day 116 -> %4=0 -> Lintah
    assert_eq!(d.watek_alit.name(), "Lintah");
    let d = BalineseDate::from_ymd(2026, 1, 2).unwrap(); // pawukon_day 117 -> %4=1 -> Uler
    assert_eq!(d.watek_alit.name(), "Uler");
    let d = BalineseDate::from_ymd(2026, 1, 3).unwrap(); // pawukon_day 118 -> %4=2 -> Gajah
    assert_eq!(d.watek_alit.name(), "Gajah");
    let d = BalineseDate::from_ymd(2026, 1, 4).unwrap(); // pawukon_day 119 -> %4=3 -> Lembu
    assert_eq!(d.watek_alit.name(), "Lembu");
    // Verify cycle repeats
    let d = BalineseDate::from_ymd(2026, 1, 5).unwrap(); // pawukon_day 120 -> %4=0 -> Lintah
    assert_eq!(d.watek_alit.name(), "Lintah");
}

#[test]
fn watek_madya_cycle() {
    // Verify the 5-cycle produces the expected canonical names
    // Using correct epoch PAWUKON_EPOCH_JDN = 2440976
    let d = BalineseDate::from_ymd(2026, 1, 1).unwrap(); // pawukon_day 116 -> %5=1 -> Watu
    assert_eq!(d.watek_madya.name(), "Watu");
    let d = BalineseDate::from_ymd(2026, 1, 2).unwrap(); // pawukon_day 117 -> %5=2 -> Buta
    assert_eq!(d.watek_madya.name(), "Buta");
    let d = BalineseDate::from_ymd(2026, 1, 3).unwrap(); // pawukon_day 118 -> %5=3 -> Suku
    assert_eq!(d.watek_madya.name(), "Suku");
    let d = BalineseDate::from_ymd(2026, 1, 4).unwrap(); // pawukon_day 119 -> %5=4 -> Wong
    assert_eq!(d.watek_madya.name(), "Wong");
    let d = BalineseDate::from_ymd(2026, 1, 5).unwrap(); // pawukon_day 120 -> %5=0 -> Gajah
    assert_eq!(d.watek_madya.name(), "Gajah");
    // Verify cycle repeats
    let d = BalineseDate::from_ymd(2026, 1, 6).unwrap(); // pawukon_day 121 -> %5=1 -> Watu
    assert_eq!(d.watek_madya.name(), "Watu");
}

#[test]
fn lintang_spot_checks() {
    // Spot-check known Lintang values at specific dates
    // Lintang = (Pancawara * 7 + Saptawara) mod 35 using enum discriminants
    let checks = [
        (2026, 1, 2, "Sungenge"),  // Sukra(5) + Wage(3) -> idx 26
        (2026, 2, 4, "Mendanu"),   // Buda(3) + Paing(1) -> idx 10
        (2026, 3, 10, "Lumbung"),  // Anggara(2) + Umanis(0) -> idx 2
        (2026, 3, 13, "Sungenge"), // Sukra(5) + Wage(3) -> idx 26
    ];
    for (y, m, d, expected) in checks {
        let balinese = BalineseDate::from_ymd(y, m, d).unwrap();
        assert_eq!(balinese.lintang.name(), expected);
    }
}

#[test]
fn lintang_cycle_completeness() {
    // Verify we can generate all 35 Lintang names
    let mut seen = std::collections::HashSet::new();
    let mut current_date = chrono::NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    for _ in 0..210 {
        // 210 = lcm(5,7,4,35) ensures full cycle coverage
        let balinese = BalineseDate::from_ymd(
            current_date.year(),
            current_date.month(),
            current_date.day(),
        )
        .unwrap();
        seen.insert(balinese.lintang.name());
        current_date = current_date.succ_opt().unwrap();
    }
    assert_eq!(seen.len(), 35, "Should see all 35 Lintang names");
    // Verify a few known entries exist
    assert!(seen.contains("Gajah"));
    assert!(seen.contains("Kiriman"));
    assert!(seen.contains("Pucang"));
    assert!(seen.contains("Sangkatikel"));
    assert!(seen.contains("Pagelaran"));
}
