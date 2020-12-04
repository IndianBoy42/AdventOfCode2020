use aoc20::day1;
use aoc20::utils;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = utils::read_input("input1.txt").unwrap();

    c.bench_function("Day 1 Part 1", |b| {
        b.iter(|| day1::part1(black_box(&input)))
    });
    c.bench_function("Day 1 Part 2", |b| {
        b.iter(|| day1::part2(black_box(&input)))
    });
    c.bench_function("Day 1 Part 2", |b| {
        b.iter(|| day1::part2_2(black_box(&input)))
    });
    c.bench_function("Day 1 Part 2", |b| {
        b.iter(|| day1::part2_3(black_box(&input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
