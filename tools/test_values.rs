use balinese_calendar::BalineseDate;

fn main() {
    let d = BalineseDate::from_ymd(2026, 2, 14).unwrap();
    println!("Date: 2026-02-14");
    println!("Wuku: {} ({:?})", d.wuku.name(), d.wuku);
    println!("Saptawara: {} ({:?})", d.saptawara.name(), d.saptawara);
    println!("Pancawara: {} ({:?})", d.pancawara.name(), d.pancawara);
    println!("Pawukon day: {}", d.pawukon_day);
}
