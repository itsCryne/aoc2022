use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc2022::{days::*, util::get_puzzle_input};

fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("Day 01a", |b| b.iter(|| day_01::a(black_box(&get_puzzle_input(1)))));
    c.bench_function("Day 01b", |b| b.iter(|| day_01::b(black_box(&get_puzzle_input(1)))));

    c.bench_function("Day 02a", |b| b.iter(|| day_02::a(black_box(&get_puzzle_input(2)))));
    c.bench_function("Day 02b", |b| b.iter(|| day_02::b(black_box(&get_puzzle_input(2)))));

    c.bench_function("Day 03a", |b| b.iter(|| day_03::a(black_box(&get_puzzle_input(3)))));
    c.bench_function("Day 03b", |b| b.iter(|| day_03::b(black_box(&get_puzzle_input(3)))));

    c.bench_function("Day 04a", |b| b.iter(|| day_04::a(black_box(&get_puzzle_input(4)))));
    c.bench_function("Day 04b", |b| b.iter(|| day_04::b(black_box(&get_puzzle_input(4)))));

    c.bench_function("Day 05a", |b| b.iter(|| day_05::a(black_box(&get_puzzle_input(5)))));
    c.bench_function("Day 05b", |b| b.iter(|| day_05::b(black_box(&get_puzzle_input(5)))));

    c.bench_function("Day 06a", |b| b.iter(|| day_06::a(black_box(&get_puzzle_input(6)))));
    c.bench_function("Day 06b", |b| b.iter(|| day_06::b(black_box(&get_puzzle_input(6)))));

    c.bench_function("Day 06a", |b| b.iter(|| day_06::a(black_box(&get_puzzle_input(6)))));
    c.bench_function("Day 06b", |b| b.iter(|| day_06::b(black_box(&get_puzzle_input(6)))));

    c.bench_function("Day 07a", |b| b.iter(|| day_07::a(black_box(&get_puzzle_input(7)))));
    c.bench_function("Day 07b", |b| b.iter(|| day_07::b(black_box(&get_puzzle_input(7)))));

    c.bench_function("Day 08a", |b| b.iter(|| day_08::a(black_box(&get_puzzle_input(8)))));
    c.bench_function("Day 08b", |b| b.iter(|| day_08::b(black_box(&get_puzzle_input(8)))));

    c.bench_function("Day 09a", |b| b.iter(|| day_09::a(black_box(&get_puzzle_input(9)))));
    c.bench_function("Day 09b", |b| b.iter(|| day_09::b(black_box(&get_puzzle_input(9)))));
    
    c.bench_function("Day 10a", |b| b.iter(|| day_10::a(black_box(&get_puzzle_input(10)))));
    c.bench_function("Day 10b", |b| b.iter(|| day_10::b(black_box(&get_puzzle_input(10)))));

    c.bench_function("Day 11a", |b| b.iter(|| day_11::a(black_box(&get_puzzle_input(11)))));
    c.bench_function("Day 11b", |b| b.iter(|| day_11::b(black_box(&get_puzzle_input(11)))));

    c.bench_function("Day 12a", |b| b.iter(|| day_12::a(black_box(&get_puzzle_input(12)))));
    c.bench_function("Day 12b", |b| b.iter(|| day_12::b(black_box(&get_puzzle_input(12)))));

    c.bench_function("Day 13a", |b| b.iter(|| day_13::a(black_box(&get_puzzle_input(13)))));
    c.bench_function("Day 13b", |b| b.iter(|| day_13::b(black_box(&get_puzzle_input(13)))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);