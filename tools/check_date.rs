use balinese_calendar::pawukon::Wuku;
use balinese_calendar::utils::gregorian_to_jdn;
use balinese_calendar::wewaran::pawukon_day;

fn main() {
    let jdn = gregorian_to_jdn(2026, 2, 14).unwrap();
    println!("JDN for 2026-02-14: {}", jdn);

    let pawukon_day = pawukon_day(jdn);
    println!("Pawukon day: {}", pawukon_day);

    let wuku_idx = (pawukon_day / 7) as usize;
    println!("Wuku index: {}", wuku_idx);

    let wuku = Wuku::from_index(wuku_idx);
    println!("Wuku: {} ({:?})", wuku.name(), wuku);
}
