use aoc20::{day10, utils::read_input};

fn main() {
    let input = read_input("10.in").unwrap();
    // let input = read_input("input10.txt").unwrap();

    dbg!(day10::part1(&input));
    dbg!(day10::part2(&input));
}
