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
        let nums: BitSet<usize> = parse(input).collect();
        // let (min, max) = minmax(&nums).unwrap();
        let (min, max) = (
            nums.iter().next().unwrap(),
            nums.iter().last().unwrap(),
        );
        (min..max).find(|&e| !nums.contains(e)).unwrap()
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
