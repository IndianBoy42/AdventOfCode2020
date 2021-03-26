use std::collections::hash_map::Entry;
use std::{mem, unreachable};

use crate::utils::*;

// const SPLIT: usize = 1_000_000;
const SPLIT: usize = 500_000;
const DENSE_CAPACITY: usize = SPLIT;
const SPARSE_CAPACITY: usize = SPLIT;

use nohash_hasher::IntMap;

macro_rules! entry {
    ($map:tt, $densemap:tt, $u:tt) => {
        // $densemap.entry($u)
        if $u < SPLIT {
            $densemap.entry($u)
        } else {
            $map.entry($u)
        }
    };
}

fn solve(input: &str, n: usize) -> usize {
    let mut map: FMap<usize, (usize, usize)> = fmap((n * 2 + 1).min(SPARSE_CAPACITY));
    let mut densemap: IntMap<usize, (usize, usize)> =
        IntMap::with_capacity_and_hasher((n * 2 + 1).min(DENSE_CAPACITY), Default::default());

    let starting_count = input.split(',').count();

    let mut last = 0;
    input
        .split(',')
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .enumerate()
        .for_each(|(i, num)| {
            // entry!(map, densemap, num).or_insert((i, i));
            map.insert(num, (i, i));
            densemap.insert(num, (i, i));
            last = num;
        });

    // let mut last: usize = input
    //     .split(',')
    //     .last()
    //     .and_then(|s| s.parse().ok())
    //     .unwrap();

    match entry!(map, densemap, last) {
        // match map.entry(last) {
        Entry::Occupied(mut occ) => {
            let (prev, prev2) = occ.get_mut();
            last = *prev - *prev2;
        }
        Entry::Vacant(vac) => {
            unreachable!()
        }
    }

    for i in starting_count..(n - 1) {
        match entry!(map, densemap, last) {
            // match map.entry(last) {
            Entry::Occupied(mut occ) => {
                let (prev, prev2) = occ.get_mut();
                *prev2 = *prev;
                *prev = i;
                last = *prev - *prev2;
            }
            Entry::Vacant(vac) => {
                vac.insert((i, i));
                last = 0;
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
