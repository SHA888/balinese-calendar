// benches/bench_conversion.rs
//
// Criterion benchmarks for BalineseDate construction throughput.
// Run with: cargo bench

use balinese_calendar::BalineseDate;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_single_date(c: &mut Criterion) {
    c.bench_function("BalineseDate::from_ymd (single)", |b| {
        b.iter(|| BalineseDate::from_ymd(black_box(2026), black_box(3), black_box(6)).unwrap())
    });
}

fn bench_year_range(c: &mut Criterion) {
    let start_jdn = balinese_calendar::utils::gregorian_to_jdn(2024, 1, 1).unwrap();
    c.bench_function("BalineseDate: full year (365 dates)", |b| {
        b.iter(|| {
            for jdn in start_jdn..start_jdn + 365 {
                black_box(BalineseDate::from_jdn(black_box(jdn)));
            }
        })
    });
}

fn bench_flat_record(c: &mut Criterion) {
    let d = BalineseDate::from_ymd(2026, 3, 6).unwrap();
    c.bench_function("to_flat_record()", |b| {
        b.iter(|| black_box(d.to_flat_record()))
    });
}

criterion_group!(
    benches,
    bench_single_date,
    bench_year_range,
    bench_flat_record
);
criterion_main!(benches);
