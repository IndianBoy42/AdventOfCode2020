use crate::utils::*;

pub fn part1(input: &str) -> usize {
    let N = input.lines().next().unwrap().as_bytes().len();
    input
        .lines()
        .enumerate()
        .filter(|(i, line)| line.bytes().map(|b| b == b'#').nth((i * 3) % N).unwrap())
        // .fold(0, |a, b| a + b as usize)
        .count()
}
pub fn part2(input: &str) -> usize {
    let N = input.lines().next().unwrap().as_bytes().len();
    let check = |(right, down)| {
        input
            .lines()
            .step_by(down)
            .enumerate()
            .filter(|(i, line)| {
                line.bytes()
                    .map(|b| b == b'#')
                    .nth((i * right) % N)
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
fn test2() {
    let input = read_input("input3.txt").unwrap();
    assert_eq!(part1(&input), 265);
    assert_eq!(part2(&input), 3154761400);
}
