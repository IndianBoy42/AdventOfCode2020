use crate::utils::*;

pub fn part1(input: &str) -> usize {
    let n = input.lines().next().unwrap().as_bytes().len();

    input
        .lines()
        .enumerate()
        .filter(|(i, line)| line.bytes().nth((i * 3) % n).map(|b| b == b'#').unwrap())
        .count()
}
pub fn part2(input: &str) -> usize {
    let n = input.lines().next().unwrap().as_bytes().len();
    let lines = input.lines().collect_vec();
    let check = |(right, down)| {
        lines
            .iter()
            .step_by(down)
            .enumerate()
            .filter(|(i, line)| {
                line.bytes()
                    .nth((i * right) % n)
                    .map(|b| b == b'#')
                    .unwrap()
            })
            .count()
        // .fold(0, |a, b| a + b as usize)
    };

    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .copied()
        .map(check)
        .product()
}

#[test]
fn test() {
    let input = read_input("input3.txt").unwrap();
    assert_eq!(part1(&input), 265);
    assert_eq!(part2(&input), 3154761400);
}
