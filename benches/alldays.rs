use aoc20::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let input1 = utils::read_input("input1.txt").unwrap();
    let input2 = utils::read_input("input2.txt").unwrap();
    let input3 = utils::read_input("input3.txt").unwrap();
    let input4 = utils::read_input("input4.txt").unwrap();

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
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
