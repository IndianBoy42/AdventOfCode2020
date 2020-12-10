use std::collections::VecDeque;

use crate::utils::*;

fn nums(input: &str) -> Vec<i16> {
    input // Essentially counting sort
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<BitSet>()
        .into_iter()
        .map(|x| x as _)
        .collect()
    // let mut nums: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    // nums.sort_unstable();
    // nums
}

pub fn part10(input: &str) -> usize {
    let nums = nums(input);

    let it = nums.array_windows().map(|[a, b]| b - a);

    assert_eq!(it.clone().filter(|&d| d > 3).count(), 0);
    // assert_eq!(it.clone().filter(|&d| d == 2).count(), 0);
    // let threes = it.clone().filter(|&d| d == 3).count() + 1;
    let ones = it.filter(|&d| d == 1).count() + 1;
    // assert_eq!(threes + ones - 1, nums.len());
    let threes = nums.len() - ones + 1;

    ones * threes
}

pub fn part1(input: &str) -> usize {
    type S = BitSet;
    let nums: S = input.lines().map(|line| line.parse().unwrap()).collect();

    // let ones = nums.iter().tuple_windows().map(|(a, b)| b - a).filter(|&d| d == 1).count() + 1;
    let ones = nums.iter().filter(|&d| nums.contains(d - 1)).count() + 1;
    let threes = nums.len() - ones + 1;

    ones * threes
}

fn trib(c: usize) -> i64 {
    // if you wanna be pedantic, you can actually calculate this for however many long you want
    // precomputation is still O(n), although it would be slower so
    match c {
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 7,
        _ => 1,
    }
}

pub fn part21(input: &str) -> i64 {
    // let mut nums: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    // nums.push(0);
    // nums.sort_unstable();
    // let grps = nums.array_windows().group_by(|[b, a]| a - b);
    let mut nums = input // Essentially counting sort
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<BitSet>();
    nums.insert(0);

    if false {
        let grps = nums.iter().tuple_windows().group_by(|(b, a)| a - b);

        let prod = grps
            .into_iter()
            .filter_map(|(d, grp)| (d == 1).as_some(grp.count()).map(trib))
            .product();

        prod
    } else {
        let (cons, acc, _) = nums
            .into_iter()
            .skip(1)
            .fold((0, 1, 0), |(cons, acc, prev), next| {
                let diff = next - prev;
                if diff == 3 {
                    (0, acc * trib(cons), next)
                } else {
                    (cons + 1, acc, next)
                }
            });
        acc * trib(cons)
    }
}
pub fn part2(input: &str) -> i64 {
    // let nums = nums(input);
    let nums = input // Essentially counting sort
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<BitSet>();

    let mut table = vec![0; nums.len() * 3];
    let table = &mut table[..];
    table[0] = 1;

    for (i, n) in nums.iter().enumerate().take(3) {
        table[n] = table[(n.max(3) - 3)..n].iter().sum();
    }
    let mut last = 0;
    for (i, n) in nums.into_iter().enumerate().skip(3) {
        last = table[(n - 3)..n].iter().sum();
        table[n] = last;
    }

    last
}

use num_bigint::BigUint;
use num_traits::{One, Zero};
pub fn part2big(input: &str) -> BigUint {
    let nums = nums(input);

    let mut table: VecDeque<BigUint> = std::iter::once(Zero::zero()).collect();
    let mut ti = 0;
    for i in 0..3 {
        table.push_back(
            ((0..i)
                .filter(|&j| (nums[i] - nums[j]) <= 3)
                .map(|j| &table[i - j])
                .sum::<BigUint>())
                + BigUint::one(),
        );
    }
    for i in 3..nums.len() {
        table.push_back(
            ((i - 3)..i)
                .filter(|&j| (nums[i] - nums[j]) <= 3)
                .map(|j| &table[4 - (i - j)])
                .sum(),
        );
        table.pop_front();
        ti += 1;
    }

    let mut num = table.pop_back().unwrap();
    while &num % 10u32 == Zero::zero() {
        num /= 10u32;
    };
    num % 1_000_000_000_000_000u64
}

pub fn part20(input: &str) -> i64 {
    let nums = nums(input);

    let mut table = vec![0; nums.len()];
    let table = &mut table[..];
    for i in 0..3 {
        table[i] = 1;
        for j in 0..i {
            if (nums[i] - nums[j]) <= 3 {
                table[i] += table[j];
            }
        }
    }
    for i in 3..nums.len() {
        for j in (i - 3)..i {
            if (nums[i] - nums[j]) <= 3 {
                table[i] += table[j];
            }
        }
    }

    *table.last().unwrap()
}

#[test]
fn test() {
    let input = read_input("input10.txt").unwrap();
    assert_eq!(part1(&input), 3000);
    assert_eq!(part2(&input), 193434623148032);
    // assert_eq!(day10::part2big(&input), part2(&input).try_into().unwrap());
}
