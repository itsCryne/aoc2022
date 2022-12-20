use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion, SamplingMode};
use aoc2022::{days::*, util::get_puzzle_input};

fn criterion_benchmark(c: &mut Criterion) {
    let mut g =c.benchmark_group("All days");
    g.sampling_mode(SamplingMode::Auto);
    g.warm_up_time(Duration::from_secs(10));
    g.measurement_time(Duration::from_secs(10));

    g.bench_function("Day 01a", |b| b.iter(|| day_01::a(black_box(&get_puzzle_input(1)))));
    g.bench_function("Day 01b", |b| b.iter(|| day_01::b(black_box(&get_puzzle_input(1)))));

    g.bench_function("Day 02a", |b| b.iter(|| day_02::a(black_box(&get_puzzle_input(2)))));
    g.bench_function("Day 02b", |b| b.iter(|| day_02::b(black_box(&get_puzzle_input(2)))));

    g.bench_function("Day 03a", |b| b.iter(|| day_03::a(black_box(&get_puzzle_input(3)))));
    g.bench_function("Day 03b", |b| b.iter(|| day_03::b(black_box(&get_puzzle_input(3)))));

    g.bench_function("Day 04a", |b| b.iter(|| day_04::a(black_box(&get_puzzle_input(4)))));
    g.bench_function("Day 04b", |b| b.iter(|| day_04::b(black_box(&get_puzzle_input(4)))));

    g.bench_function("Day 05a", |b| b.iter(|| day_05::a(black_box(&get_puzzle_input(5)))));
    g.bench_function("Day 05b", |b| b.iter(|| day_05::b(black_box(&get_puzzle_input(5)))));

    g.bench_function("Day 06a", |b| b.iter(|| day_06::a(black_box(&get_puzzle_input(6)))));
    g.bench_function("Day 06b", |b| b.iter(|| day_06::b(black_box(&get_puzzle_input(6)))));

    g.bench_function("Day 07a", |b| b.iter(|| day_07::a(black_box(&get_puzzle_input(7)))));
    g.bench_function("Day 07b", |b| b.iter(|| day_07::b(black_box(&get_puzzle_input(7)))));

    g.bench_function("Day 08a", |b| b.iter(|| day_08::a(black_box(&get_puzzle_input(8)))));
    g.bench_function("Day 08b", |b| b.iter(|| day_08::b(black_box(&get_puzzle_input(8)))));

    g.bench_function("Day 09a", |b| b.iter(|| day_09::a(black_box(&get_puzzle_input(9)))));
    g.bench_function("Day 09b", |b| b.iter(|| day_09::b(black_box(&get_puzzle_input(9)))));
    
    g.bench_function("Day 10a", |b| b.iter(|| day_10::a(black_box(&get_puzzle_input(10)))));
    g.bench_function("Day 10b", |b| b.iter(|| day_10::b(black_box(&get_puzzle_input(10)))));

    g.bench_function("Day 11a", |b| b.iter(|| day_11::a(black_box(&get_puzzle_input(11)))));
    g.bench_function("Day 11b", |b| b.iter(|| day_11::b(black_box(&get_puzzle_input(11)))));

    g.bench_function("Day 12a", |b| b.iter(|| day_12::a(black_box(&get_puzzle_input(12)))));
    g.bench_function("Day 12b", |b| b.iter(|| day_12::b(black_box(&get_puzzle_input(12)))));

    g.bench_function("Day 13a", |b| b.iter(|| day_13::a(black_box(&get_puzzle_input(13)))));
    g.bench_function("Day 13b", |b| b.iter(|| day_13::b(black_box(&get_puzzle_input(13)))));

    g.bench_function("Day 14a", |b| b.iter(|| day_14::a(black_box(&get_puzzle_input(14)))));
    g.bench_function("Day 14b", |b| b.iter(|| day_14::b(black_box(&get_puzzle_input(14)))));

    g.bench_function("Day 15a", |b| b.iter(|| day_15::a(black_box(&get_puzzle_input(15)))));
    g.bench_function("Day 15b", |b| b.iter(|| day_15::b(black_box(&get_puzzle_input(15)))));

    g.bench_function("Day 16a", |b| b.iter(|| day_16::a(black_box(&get_puzzle_input(16)))));
    g.bench_function("Day 16b", |b| b.iter(|| day_16::b(black_box(&get_puzzle_input(16)))));

    g.bench_function("Day 17a", |b| b.iter(|| day_17::a(black_box(&get_puzzle_input(17)))));
    g.bench_function("Day 17b", |b| b.iter(|| day_17::b(black_box(&get_puzzle_input(17)))));

    g.bench_function("Day 18a", |b| b.iter(|| day_18::a(black_box(&get_puzzle_input(18)))));
    g.bench_function("Day 18b", |b| b.iter(|| day_18::b(black_box(&get_puzzle_input(18)))));

    g.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);