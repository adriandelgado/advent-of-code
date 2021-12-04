use aoc::FILES;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("01A", |b| b.iter(|| aoc::day1::a(black_box(FILES[0]))));
    c.bench_function("01B", |b| b.iter(|| aoc::day1::b(black_box(FILES[0]))));
    c.bench_function("02A", |b| b.iter(|| aoc::day2::a(black_box(FILES[1]))));
    c.bench_function("02B", |b| b.iter(|| aoc::day2::b(black_box(FILES[1]))));
    c.bench_function("03A", |b| b.iter(|| aoc::day3::a(black_box(FILES[2]))));
    c.bench_function("03B", |b| b.iter(|| aoc::day3::b(black_box(FILES[2]))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
