use crate::utils::*;

fn parse<I: std::str::FromStr>(input: &str) -> impl Iterator<Item = (I, I, char, &str)>
where
    <I as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| line.split(' ').collect_tuple().unwrap())
        .map(|(range, letter, pwd)| {
            (
                range.split('-').collect_tuple().unwrap(),
                letter.as_bytes()[0] as char,
                // letter.chars().next().unwrap(),
                pwd,
            )
        })
        // .take(10)
        .map(|((lower, upper), letter, pwd)| {
            (lower.parse().unwrap(), upper.parse().unwrap(), letter, pwd)
        })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .filter(|&(lower, upper, letter, pwd)| {
            (lower..=upper).contains(&pwd.chars().filter(move |&c| c == letter).count())
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .filter(|&(lower, upper, letter, pwd): &(usize, usize, _, _)| {
            let a = pwd.as_bytes()[lower - 1] as char;
            let b = pwd.as_bytes()[upper - 1] as char;
            (a == letter) ^ (b == letter)
        })
        .count()
}
