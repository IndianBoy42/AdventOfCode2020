use std::collections::hash_map::Entry;
use std::{mem, unreachable};

use crate::utils::*;

fn solve(input: &str, n: usize) -> usize {
    let mut map: FMap<usize, (usize, Option<usize>)> = fmap(2020);

    let starting_count = input.split(',').count();

    let mut last = 0;
    input
        .split(',')
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .enumerate()
        .for_each(|(i, num)| {
            map.insert(num, (i, None));
            last = num;
        });

    // let mut last: usize = input
    //     .split(',')
    //     .last()
    //     .and_then(|s| s.parse().ok())
    //     .unwrap();

    for i in starting_count..n {
        match map.entry(last) {
            Entry::Occupied(mut occ) => {
                let (prev, prev2) = occ.get_mut();
                last = if let Some(prev2) = prev2 {
                    *prev - *prev2
                } else {
                    // First time spoken
                    0
                }
            }
            Entry::Vacant(vac) => {
                unreachable!()
            }
        }

        match map.entry(last) {
            Entry::Occupied(mut occ) => match occ.get_mut() {
                (prev, Some(prev2)) => {
                    *prev2 = *prev;
                    *prev = i;
                }
                (prev, prev2) => {
                    *prev2 = Some(*prev);
                    *prev = i;
                }
            },
            Entry::Vacant(vac) => {
                vac.insert((i, None));
            }
        };
    }

    last
}

pub fn part1(input: &str) -> usize {
    solve(input, 2020)
}

pub fn part2(input: &str) -> usize {
    solve(input, 30_000_000)
}

#[test]
fn test() {
    let input = read_input("input15.txt").unwrap();
    assert_eq!(part1(&input), 1522);
    assert_eq!(part2(&input), 18234);
}
