use aoc20::{*};

fn main() {
    let input = utils::read_input("input11.txt").unwrap();
    dbg!(day11::part1(&input));
    dbg!(day11::part2(&input));
    // assert_eq!(day10::part20(&input), 193434623148032);
    // assert_eq!(day10::part2big(&input), 193434623148032u64.into());

    // let input = read_input("10.in").unwrap();
    // println!("{}", day10::part2big(&input));
}
