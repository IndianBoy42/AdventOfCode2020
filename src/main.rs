use aoc20::{day11, day111, utils::read_input};

fn main() {
    let input = read_input("input11.txt").unwrap();
    dbg!(day111::part1(&input));
    dbg!(day111::part2(&input));
    // assert_eq!(day10::part20(&input), 193434623148032);
    // assert_eq!(day10::part2big(&input), 193434623148032u64.into());

    // let input = read_input("10.in").unwrap();
    // println!("{}", day10::part2big(&input));
}
