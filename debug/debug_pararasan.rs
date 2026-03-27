use balinese_calendar::BalineseDate;

fn main() {
    let d = BalineseDate::from_ymd(2026, 1, 3).unwrap();
    println!("2026-01-03:");
    println!("  arya_name: {}", d.pararasan.name());
    println!("  bidja_name: {}", d.pararasan.name_sundari_bungkah());
    println!(
        "  pancawa: {}, sapta: {}",
        d.pancawara.name(),
        d.saptawara.name()
    );
    println!("  idx: {}", (d.pancawara as u8 + d.saptawara as u8) % 8);
}
