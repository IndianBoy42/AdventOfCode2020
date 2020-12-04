use aoc20::day3;
use aoc20::utils;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = utils::read_input("input3.txt").unwrap();

    c.bench_function("Day 3 Part 1", |b| {
        b.iter(|| day3::part1(black_box(&input)))
    });
    c.bench_function("Day 3 Part 2", |b| {
        b.iter(|| day3::part2(black_box(&input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
