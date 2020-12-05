use aoc20::day1;
use aoc20::day2;
use aoc20::day3;
use aoc20::day4;
use aoc20::day5;
use aoc20::utils;

fn main() {
    println!("Hello, world!");

    let input = utils::read_input("input1.txt").unwrap();
    dbg!(day1::part1(&input));
    dbg!(day1::part2(&input));

    let input = utils::read_input("input2.txt").unwrap();
    dbg!(day2::part1(&input));
    dbg!(day2::part2(&input));

    let input = utils::read_input("input3.txt").unwrap();
    dbg!(day3::part1(&input));
    dbg!(day3::part2(&input));

    let input = utils::read_input("input4.txt").unwrap();
    dbg!(day4::part1(&input));
    dbg!(day4::part2(&input));

    let input = utils::read_input("input5.txt").unwrap();
    dbg!(day5::part1(&input));
    dbg!(day5::part2(&input));
}
