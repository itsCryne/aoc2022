use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc2022::{days::*, util::get_puzzle_input};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Day 01a", |b| b.iter(|| day_01::a(black_box(&get_puzzle_input(1)))));
    c.bench_function("Day 01b", |b| b.iter(|| day_01::b(black_box(&get_puzzle_input(1)))));

    c.bench_function("Day 02a", |b| b.iter(|| day_02::a(black_box(&get_puzzle_input(2)))));
    c.bench_function("Day 02b", |b| b.iter(|| day_02::b(black_box(&get_puzzle_input(2)))));

    c.bench_function("Day 03a", |b| b.iter(|| day_03::a(black_box(&get_puzzle_input(3)))));
    c.bench_function("Day 03b", |b| b.iter(|| day_03::b(black_box(&get_puzzle_input(3)))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);