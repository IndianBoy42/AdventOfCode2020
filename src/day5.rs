use bit_set::BitSet;

use crate::utils::*;

fn pows() -> impl Iterator<Item = usize> {
    successors(Some(1), |prev| Some(prev * 2))
}

fn binparse2(slice: &[u8], chk: impl FnMut(&u8) -> bool) -> usize {
    slice
        .iter()
        .rev()
        .map(chk)
        .zip(pows())
        .map(|(v, pow)| (v as usize) * pow)
        // .map(|(v, pow)| ((-(v as isize)) as usize) & pow)
        // .filter_map(|(v, pow)| v.as_some(pow))
        .sum::<usize>()
}
fn binparse(slice: &[u8], mut chk: impl FnMut(&u8) -> bool) -> usize {
    slice
        .iter()
        .fold(0, |acc, v| (acc << 1) + (chk(v) as usize))
}

fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|line| {
        // let (l, r) = line.as_bytes()[..10].split_at(7);
        // binparse(l, |&c| c == b'B') * 8 + binparse(r, |&c| c == b'R')
        binparse(&line.as_bytes()[..10], |&c| (c & 0b0100) == 0)
    })
}

pub fn part1(input: &str) -> usize {
    parse(input).max().unwrap()
}

// pub fn part2(input: &str) -> usize {
//     let nums: BitSet<usize> = parse(input).collect();
//     let (min, max) = (nums.iter().next().unwrap(), nums.iter().last().unwrap());
//     (min..max).find(|&e| !nums.contains(e)).unwrap()
// }
pub fn part2(input: &str) -> usize {
    let (min, max, sum) = parse(input).fold((usize::MAX, usize::MIN, 0), |(min, max, sum), v| {
        (min.min(v), max.max(v), sum + v)
    });
    ((max - min + 1) * (min + max) / 2) - sum
    // (min..(max + 1)).sum::<usize>() - sum
}

#[test]
fn test() {
    let input = read_input("input5.txt").unwrap();
    assert_eq!(part1(&input), 864);
    assert_eq!(part2(&input), 739);
}
