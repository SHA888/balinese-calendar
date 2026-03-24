// Manual validation helper for Paringkelan (Watek Madya/Alit and Lintang)
// Run with: cargo test --bin paringkelan_manual_validation -- --nocapture
// Then compare output against corpus data manually

use balinese_calendar::BalineseDate;

fn main() {
    let dates = [
        "2026-01-02",
        "2026-02-04",
        "2026-03-10",
        "2026-03-13",
        "2026-04-23",
        "2026-05-01",
        "2026-05-10",
        "2026-05-15",
        "2026-05-16",
        "2026-05-20",
        "2026-05-22",
        "2026-06-21",
        "2026-07-02",
        "2026-07-12",
        "2026-07-18",
        "2026-07-22",
        "2026-07-23",
        "2026-08-07",
        "2026-08-18",
        "2026-08-21",
        "2026-09-04",
        "2026-09-05",
        "2026-09-09",
        "2026-09-23",
        "2026-11-07",
        "2026-11-27",
        "2026-12-08",
        "2026-12-09",
        "2026-12-10",
        "2026-12-16",
    ];
    println!("date,wuku,sapta,panca,watek_madya,watek_alit,lintang");
    for date_str in dates {
        let (y, m, d) = parse_date(date_str);
        let balinese = BalineseDate::from_ymd(y, m, d).unwrap();
        let watek_madya = balinese.watek_madya.name();
        let watek_alit = balinese.watek_alit.name();
        let lintang = balinese.lintang.name();
        println!(
            "{},{},{},{},{},{},{}",
            date_str,
            balinese.wuku.name(),
            balinese.saptawara.name(),
            balinese.pancawara.name(),
            watek_madya,
            watek_alit,
            lintang
        );
    }
}

fn parse_date(s: &str) -> (i32, u32, u32) {
    let parts: Vec<_> = s.split('-').collect();
    (
        parts[0].parse().unwrap(),
        parts[1].parse().unwrap(),
        parts[2].parse().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_manual_validation() {
        main()
    }
}
