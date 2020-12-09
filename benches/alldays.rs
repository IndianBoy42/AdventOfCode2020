use aoc20::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input1 = utils::read_input("input1.txt").unwrap();
    c.bench_function("Day 1 Part 1", |b| {
        b.iter(|| day1::part1(black_box(&input1)))
    });
    c.bench_function("Day 1 Part 2", |b| {
        b.iter(|| day1::part2(black_box(&input1)))
    });

    let input2 = utils::read_input("input2.txt").unwrap();
    c.bench_function("Day 2 Part 1", |b| {
        b.iter(|| day2::part1(black_box(&input2)))
    });
    c.bench_function("Day 2 Part 2", |b| {
        b.iter(|| day2::part2(black_box(&input2)))
    });

    let input3 = utils::read_input("input3.txt").unwrap();
    c.bench_function("Day 3 Part 1", |b| {
        b.iter(|| day3::part1(black_box(&input3)))
    });
    c.bench_function("Day 3 Part 2", |b| {
        b.iter(|| day3::part2(black_box(&input3)))
    });

    let input4 = utils::read_input("input4.txt").unwrap();
    c.bench_function("Day 4 Part 1", |b| {
        b.iter(|| day4::part1(black_box(&input4)))
    });
    c.bench_function("Day 4 Part 2", |b| {
        b.iter(|| day4::part2(black_box(&input4)))
    });

    let input5 = utils::read_input("input5.txt").unwrap();
    c.bench_function("Day 5 Part 1", |b| {
        b.iter(|| day5::part1(black_box(&input5)))
    });
    c.bench_function("Day 5 Part 2", |b| {
        b.iter(|| day5::part2(black_box(&input5)))
    });

    let input6 = utils::read_input("input6.txt").unwrap();
    c.bench_function("Day 6 Part 1", |b| {
        b.iter(|| day6::part1(black_box(&input6)))
    });
    c.bench_function("Day 6 Part 2", |b| {
        b.iter(|| day6::part2(black_box(&input6)))
    });

    let input7 = utils::read_input("input7.txt").unwrap();
    c.bench_function("Day 7 Part 1", |b| {
        b.iter(|| day7::part1(black_box(&input7)))
    });
    c.bench_function("Day 7 Part 2", |b| {
        b.iter(|| day7::part2(black_box(&input7)))
    });

    let input8 = utils::read_input("input8.txt").unwrap();
    c.bench_function("Day 8 Part 1", |b| {
        b.iter(|| day8::part1(black_box(&input8)))
    });
    c.bench_function("Day 8 Part 2", |b| {
        b.iter(|| day8::part2(black_box(&input8)))
    });

    let input9 = utils::read_input("input9.txt").unwrap();
    c.bench_function("Day 9 Part 1", |b| {
        b.iter(|| day9::part1(black_box(&input9)))
    });
    c.bench_function("Day 9 Part 2", |b| {
        b.iter(|| day9::part2(black_box(&input9)))
    });

    c.bench_function("All Days", |b| {
        b.iter(|| {
            day1::part1(black_box(&input1));
            day1::part2(black_box(&input1));
            day2::part1(black_box(&input2));
            day2::part2(black_box(&input2));
            day3::part1(black_box(&input3));
            day3::part2(black_box(&input3));
            day4::part1(black_box(&input4));
            day4::part2(black_box(&input4));
            day5::part1(black_box(&input5));
            day5::part2(black_box(&input5));
            day6::part1(black_box(&input6));
            day6::part2(black_box(&input6));
            day7::part1(black_box(&input7));
            day7::part2(black_box(&input7));
            day8::part1(black_box(&input8));
            day8::part2(black_box(&input8));
            day9::part1(black_box(&input9));
            day9::part2(black_box(&input9));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
