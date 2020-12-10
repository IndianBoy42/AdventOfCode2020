use crate::utils::*;

fn parse<I: std::str::FromStr>(input: &str) -> impl ParallelIterator<Item = (I, I, u8, &str)>
where
    <I as std::str::FromStr>::Err: std::fmt::Debug,
    I: Send,
{
    input
        .par_lines()
        .map(|line| line.splitn(3, ' ').collect_tuple().unwrap())
        .map(|(range, letter, pwd)| {
            (
                range.split_once('-').unwrap(),
                // range.split('-').collect_tuple().unwrap(),
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
            (lower..=upper).contains(&pwd.bytes().filter(move |&c| c == letter).count())
        })
        .count()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .filter(|&(lower, upper, letter, pwd): &(usize, usize, _, _)| {
            debug_assert!(pwd.len() >= upper);
            debug_assert!(pwd.len() > lower);
            let &a = unsafe { pwd.as_bytes().get_unchecked(lower - 1) };
            let &b = unsafe { pwd.as_bytes().get_unchecked(upper - 1) };
            (a == letter) ^ (b == letter)
        })
        .count()
    // black_box(parse::<usize>(input).for_each(drop));
    // black_box(354)
}

#[test]
fn test() {
    let input = read_input("input2.txt").unwrap();
    assert_eq!(part1(&input), 614);
    assert_eq!(part2(&input), 354);
}
