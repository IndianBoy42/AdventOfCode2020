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
    let check = |right, down| {
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

    (check(1, 1) * check(3, 1) * check(5, 1) * check(7, 1) * check(1, 2))
}
