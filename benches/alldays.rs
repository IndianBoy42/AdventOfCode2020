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

    let input10 = utils::read_input("input10.txt").unwrap();
    c.bench_function("Day 10 Part 1", |b| {
        b.iter(|| day10::part1(black_box(&input10)))
    });
    c.bench_function("Day 10 Part 2", |b| {
        b.iter(|| day10::part2(black_box(&input10)))
    });

    let input11 = utils::read_input("input11.txt").unwrap();
    c.bench_function("Day 11 Part 1", |b| {
        b.iter(|| day11::part1(black_box(&input11)))
    });
    c.bench_function("Day 11 Part 2", |b| {
        b.iter(|| day11::part2(black_box(&input11)))
    });

    let input12 = utils::read_input("input12.txt").unwrap();
    c.bench_function("Day 12 Part 1", |b| {
        b.iter(|| day12::part1(black_box(&input12)))
    });
    c.bench_function("Day 12 Part 2", |b| {
        b.iter(|| day12::part2(black_box(&input12)))
    });

    let input13 = utils::read_input("input13.txt").unwrap();
    c.bench_function("Day 13 Part 1", |b| {
        b.iter(|| day13::part1(black_box(&input13)))
    });
    c.bench_function("Day 13 Part 2", |b| {
        b.iter(|| day13::part2(black_box(&input13)))
    });

    let input14 = utils::read_input("input14.txt").unwrap();
    c.bench_function("Day 14 Part 1", |b| {
        b.iter(|| day14::part1(black_box(&input14)))
    });
    c.bench_function("Day 14 Part 2", |b| {
        b.iter(|| day14::part2(black_box(&input14)))
    });

    let input15 = utils::read_input("input15.txt").unwrap();
    c.bench_function("Day 15 Part 1", |b| {
        b.iter(|| day15::part1(black_box(&input15)))
    });

    let input16 = utils::read_input("input16.txt").unwrap();
    c.bench_function("Day 16 Part 1", |b| {
        b.iter(|| day16::part1(black_box(&input16)))
    });
    c.bench_function("Day 16 Part 2", |b| {
        b.iter(|| day16::part2(black_box(&input16)))
    });

    let input17 = utils::read_input("input17.txt").unwrap();
    c.bench_function("Day 17 Part 1", |b| {
        b.iter(|| day17::part1(black_box(&input17)))
    });
    c.bench_function("Day 17 Part 2", |b| {
        b.iter(|| day17::part2(black_box(&input17)))
    });

    let input18 = utils::read_input("input18.txt").unwrap();
    c.bench_function("Day 18 Part 1", |b| {
        b.iter(|| day18::part1(black_box(&input18)))
    });
    c.bench_function("Day 18 Part 2", |b| {
        b.iter(|| day18::part2(black_box(&input18)))
    });

    let input19 = utils::read_input("input19.txt").unwrap();
    c.bench_function("Day 19 Part 1", |b| {
        b.iter(|| day19::part1(black_box(&input19)))
    });
    c.bench_function("Day 19 Part 2", |b| {
        b.iter(|| day19::part2(black_box(&input19)))
    });

    // let input20 = utils::read_input("input20.txt").unwrap();
    // c.bench_function("Day 20 Part 1", |b| {
    //     b.iter(|| day20::part1(black_box(&input20)))
    // });
    // c.bench_function("Day 20 Part 2", |b| {
    //     b.iter(|| day20::part2(black_box(&input20)))
    // });

    let input21 = utils::read_input("input21.txt").unwrap();
    c.bench_function("Day 21 Part 1", |b| {
        b.iter(|| day21::part1(black_box(&input21)))
    });
    c.bench_function("Day 21 Part 2", |b| {
        b.iter(|| day21::part2(black_box(&input21)))
    });

    let input22 = utils::read_input("input22.txt").unwrap();
    c.bench_function("Day 22 Part 1", |b| {
        b.iter(|| day22::part1(black_box(&input22)))
    });
    c.bench_function("Day 22 Part 2", |b| {
        b.iter(|| day22::part2(black_box(&input22)))
    });

    let input23 = utils::read_input("input23.txt").unwrap();
    c.bench_function("Day 23 Part 1", |b| {
        b.iter(|| day23::part1(black_box(&input23)))
    });
    c.bench_function("Day 23 Part 2", |b| {
        b.iter(|| day23::part2(black_box(&input23)))
    });

    let input24 = utils::read_input("input24.txt").unwrap();
    c.bench_function("Day 24 Part 1", |b| {
        b.iter(|| day24::part1(black_box(&input24)))
    });
    c.bench_function("Day 24 Part 2", |b| {
        b.iter(|| day24::part2(black_box(&input24)))
    });

    let input25 = utils::read_input("input25.txt").unwrap();
    c.bench_function("Day 25 Part 1", |b| {
        b.iter(|| day25::part1(black_box(&input25)))
    });

    let mut longbenches = c.benchmark_group("Long Benchmarks");
    longbenches.sample_size(10);

    longbenches.bench_function("Day 15 Part 2", |b| {
        b.iter(|| day15::part2(black_box(&input15)))
    });

    longbenches.bench_function("All Days", |b| {
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
            day10::part1(black_box(&input10));
            day10::part2(black_box(&input10));
            day11::part1(black_box(&input11));
            day11::part2(black_box(&input11));
            day12::part1(black_box(&input12));
            day12::part2(black_box(&input12));
            day13::part1(black_box(&input13));
            day13::part2(black_box(&input13));
            day14::part1(black_box(&input14));
            day14::part2(black_box(&input14));
            day15::part1(black_box(&input15));
            day15::part2(black_box(&input15));
            day16::part1(black_box(&input16));
            day16::part2(black_box(&input16));
            day17::part1(black_box(&input17));
            day17::part2(black_box(&input17));
            day18::part1(black_box(&input18));
            day18::part2(black_box(&input18));
            day19::part1(black_box(&input19));
            day19::part2(black_box(&input19));
            // day20::part1(black_box(&input20));
            // day20::part2(black_box(&input20));
            day21::part1(black_box(&input21));
            day21::part2(black_box(&input21));
            day22::part1(black_box(&input22));
            day22::part2(black_box(&input22));
            day23::part1(black_box(&input23));
            day23::part2(black_box(&input23));
            day24::part1(black_box(&input24));
            day24::part2(black_box(&input24));
            day25::part1(black_box(&input25));
            // day25::part2(black_box(&input25));
        })
    });

    // let input10big = utils::read_input("10.in").unwrap();
    // longbenches.bench_function("Day 10 Part BIG", |b| {
    //     b.iter(|| day10::part2big(black_box(&input10big)))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
