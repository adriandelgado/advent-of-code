use aoc::FILES;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("01A", |b| b.iter(|| aoc::day1::a(black_box(FILES[0]))));
    c.bench_function("01B", |b| b.iter(|| aoc::day1::b(black_box(FILES[0]))));
    c.bench_function("02A", |b| b.iter(|| aoc::day2::a(black_box(FILES[1]))));
    c.bench_function("02B", |b| b.iter(|| aoc::day2::b(black_box(FILES[1]))));
    c.bench_function("03A", |b| b.iter(|| aoc::day3::a(black_box(FILES[2]))));
    c.bench_function("03B", |b| b.iter(|| aoc::day3::b(black_box(FILES[2]))));
    c.bench_function("04A", |b| b.iter(|| aoc::day4::a(black_box(FILES[3]))));
    c.bench_function("04B", |b| b.iter(|| aoc::day4::b(black_box(FILES[3]))));
    c.bench_function("05A", |b| b.iter(|| aoc::day5::a(black_box(FILES[4]))));
    c.bench_function("05B", |b| b.iter(|| aoc::day5::b(black_box(FILES[4]))));
    c.bench_function("06A", |b| b.iter(|| aoc::day6::a(black_box(FILES[5]))));
    c.bench_function("06B", |b| b.iter(|| aoc::day6::b(black_box(FILES[5]))));
    c.bench_function("07A", |b| b.iter(|| aoc::day7::a(black_box(FILES[6]))));
    c.bench_function("07B", |b| b.iter(|| aoc::day7::b(black_box(FILES[6]))));
    c.bench_function("08A", |b| b.iter(|| aoc::day8::a(black_box(FILES[7]))));
    c.bench_function("08B", |b| b.iter(|| aoc::day8::b(black_box(FILES[7]))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
