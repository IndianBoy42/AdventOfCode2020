use aoc20::day4;
use aoc20::utils;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = utils::read_input("input4.txt").unwrap();

    c.bench_function("Part 1", |b| {
        b.iter(|| day4::part1(black_box(&input)))
    });
    c.bench_function("Part 2", |b| {
        b.iter(|| day4::part2(black_box(&input)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
