use bit_set::BitSet;

use crate::utils::*;

fn pows() -> impl Iterator<Item = usize> {
    successors(Some(1), |prev| Some(prev * 2))
}

fn binparse(slice: &[u8], chk: impl FnMut(&u8) -> bool) -> usize {
    slice
        .iter()
        .rev()
        .map(chk)
        .zip(pows())
        .filter_map(|(v, pow)| v.as_some(pow))
        .sum::<usize>()
}

fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|line| {
        let (l, r) = line.as_bytes()[..10].split_at(7);
        binparse(l, |&c| c == b'B') * 8 + binparse(r, |&c| c == b'R')
        // binparse(r, |&c| c == b'R' || c == b'B')
    })
}

pub fn part1(input: &str) -> usize {
    parse(input).max().unwrap()
}

pub fn part2(input: &str) -> usize {
    let nums: BitSet<usize> = parse(input).collect();

    let (min, max) = (nums.iter().next().unwrap(), nums.iter().last().unwrap());

    (min..max).find(|&e| !nums.contains(e)).unwrap()
}

#[test]
fn test() {
    let input = read_input("input5.txt").unwrap();
    assert_eq!(part1(&input), 864);
    assert_eq!(part2(&input), 739);
}
