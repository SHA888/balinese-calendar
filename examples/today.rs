// examples/today.rs
//
// Display today's Balinese calendar date with full details.
//
// Run with: cargo run --example today

use balinese_calendar::{BalineseDate, DayBoundary};

fn main() -> anyhow::Result<()> {
    let today = BalineseDate::today()?;
    println!("Today (sunrise-adjusted): {}", today.to_balinese_string());

    let midnight = BalineseDate::today_with_boundary(&DayBoundary::Midnight)?;
    println!(
        "Today (midnight):         {}",
        midnight.to_balinese_string()
    );
    println!();

    let d = today;

    println!("═══════════════════════════════════════════════════════");
    println!(
        "  KALENDER BALI — {}/{:02}/{:02}",
        d.gregorian_year, d.gregorian_month, d.gregorian_day
    );
    println!("═══════════════════════════════════════════════════════");
    println!();
    println!("  {}", d.to_balinese_string());
    println!();
    println!("── Pawukon ─────────────────────────────────────────────");
    println!(
        "  Wuku        : {} (#{}, day {}/7)",
        d.wuku.name(),
        d.wuku as u8 + 1,
        d.wuku_day + 1
    );
    println!("  Pawukon day : {}/210", d.pawukon_day + 1);
    println!("  Ecology tag : {}", d.wuku.ecology_tag());
    println!();
    println!("── Wewaran ─────────────────────────────────────────────");
    println!(
        "  Saptawara   : {} ({})",
        d.saptawara.name(),
        d.saptawara.name_indonesian()
    );
    println!(
        "  Pancawara   : {} (urip {})",
        d.pancawara.name(),
        d.pancawara.urip()
    );
    println!("  Triwara     : {}", d.triwara.name());
    println!("  Dwiwara     : {}", d.dwiwara.name());
    println!("  Caturwara   : {}", d.caturwara.name());
    println!("  Sadwara     : {}", d.sadwara.name());
    println!("  Astawara    : {}", d.astawara.name());
    println!("  Sangawara   : {}", d.sangawara.name());
    println!("  Dasawara    : {}", d.dasawara.name());
    println!();
    println!("── Paringkelan ─────────────────────────────────────────");
    println!("  Jejepan     : {}", d.jejepan.name());
    println!("  Ingkel      : {}", d.ingkel.name());
    println!("  Watek Madya : {}", d.watek_madya.name());
    println!("  Watek Alit  : {}", d.watek_alit.name());
    println!("  Lintang     : {}", d.lintang.name());
    println!("  PancaSuda   : {}", d.panca_suda.name());
    println!("  Pararasan   : {}", d.pararasan.name());
    println!("  Rakam       : {}", d.rakam.name());
    println!();
    println!("── Sasih / Saka ────────────────────────────────────────");
    println!("  Saka year   : {}", d.saka_year);
    println!("  Sasih       : {}", d.sasih.name());
    println!("  Season      : {}", d.sasih.season_tag());
    println!("  Pancaroba   : {}", d.sasih.is_pancaroba());
    println!("  Planting    : {}", d.sasih.is_planting_signal());
    println!("  Purnama     : {}", d.is_purnama);
    println!("  Tilem       : {}", d.is_tilem);
    println!("  Nampih      : {}", d.is_nampih);
    println!();

    if !d.rahinan.is_empty() {
        println!("── Rahinan (Holy Days) ─────────────────────────────────");
        for r in &d.rahinan {
            println!("  ★ {r:?}");
        }
        println!();
    }

    println!("═══════════════════════════════════════════════════════");

    Ok(())
}
