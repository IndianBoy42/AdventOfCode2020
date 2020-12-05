use crate::utils::*;

fn parse<I: std::str::FromStr>(input: &str) -> impl Iterator<Item = (I, I, u8, &str)>
where
    <I as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| line.split(' ').collect_tuple().unwrap())
        .map(|(range, letter, pwd)| {
            (
                range.split('-').collect_tuple().unwrap(),
                letter.as_bytes()[0],
                // letter.chars().next().unwrap(),
                pwd,
            )
        })
        .map(|((lower, upper), letter, pwd)| {
            (lower.parse().unwrap(), upper.parse().unwrap(), letter, pwd)
        })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .filter(|&(lower, upper, letter, pwd)| {
            (lower..(upper+1)).contains(&pwd.bytes().filter(move |&c| c == letter).count())
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .filter(|&(lower, upper, letter, pwd): &(usize, usize, _, _)| {
            let a = pwd.as_bytes()[lower - 1];
            let b = pwd.as_bytes()[upper - 1];
            (a == letter) ^ (b == letter)
        })
        .count()
}

#[test]
fn test() {
    let input = read_input("input2.txt").unwrap();
    assert_eq!(part1(&input), 614);
    assert_eq!(part2(&input), 354);
}
