use aoc20::*;

fn main() {
    let input = utils::read_input("input24.txt").unwrap();
    for _ in 0..100 {
        dbg!(day24::part1(&input));
        dbg!(day24::part2(&input));
    }
}
