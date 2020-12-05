use bit_set::BitSet;

use crate::utils::*;

fn pows() -> impl Iterator<Item = usize> {
    successors(Some(1), |prev| Some(prev * 2))
}

fn parse(input: &str) -> impl Iterator<Item = usize> + '_ {
    input.lines().map(|line| {
        let (l, r) = line.as_bytes().split_at(7);
        let row = l
            .iter()
            .rev()
            .map(|&c| c == b'B')
            .zip(pows())
            .filter_map(|(v, pow)| v.as_some(pow))
            .sum::<usize>();
        let col = r
            .iter()
            .rev()
            .map(|&c| c == b'R')
            .zip(pows())
            .filter_map(|(v, pow)| v.as_some(pow))
            .sum::<usize>();

        row * 8 + col
    })
}

fn minmax(it: impl IntoIterator<Item = usize>) -> Option<(usize, usize)> {
    match it.into_iter().minmax() {
        itertools::MinMaxResult::NoElements => None,
        itertools::MinMaxResult::OneElement(a) => Some((a, a)),
        itertools::MinMaxResult::MinMax(a, b) => Some((a, b)),
    }
}

pub fn part1(input: &str) -> usize {
    parse(input).max().unwrap()
}
pub fn part2(input: &str) -> usize {
    if false {
        let nums = {
            let mut nums = parse(input).collect_vec();
            nums.sort_unstable();
            nums
        };
        izip!(&nums[1..], &nums)
            .find(|&(&a, &b)| (a - b) == 2)
            .map(|(a, _)| a + 1)
            .unwrap()
    } else {
        let nums = parse(input).collect_vec();
        let (min, max) = minmax(nums.iter().copied()).unwrap();

        let set: BitSet<usize> = nums.into_iter().collect();
        (min..max).find(|&e| !set.contains(e)).unwrap()
        // let set = FSet::from_iter(nums);
        // (min..max).find(|e| !set.contains(e)).unwrap()
    }
}

#[test]
fn test() {
    let input = read_input("input5.txt").unwrap();
    assert_eq!(part1(&input), 864);
    assert_eq!(part2(&input), 739);
}
