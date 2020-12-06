use crate::{u32set, utils::*};

use u32set::U32Set;

fn answers(it: impl IntoIterator<Item = u8>) -> U32Set {
    it.into_iter()
        .filter_map(|c| c.is_ascii_lowercase().as_some_from(|| (c - b'a') as usize))
        .collect()
}
fn intersect(mut acc: U32Set, v: U32Set) -> U32Set {
    acc.intersect_with(v);
    acc
}

pub fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        // .par_bridge()
        .map(|line| answers(line.bytes()))
        .map(U32Set::len)
        .sum()
}
pub fn part2(input: &str) -> usize {
    let full = (0..26).collect::<U32Set>();

    input
        .split("\n\n")
        .par_bridge()
        .map(|group| {
            group
                .lines()
                .map(|line| answers(line.bytes()))
                // .fold1(U32Set::intersect)
                // .unwrap()
                .fold(full, U32Set::intersect)
        })
        .map(U32Set::len)
        .sum()
}

#[test]
fn test() {
    let input = read_input("input6.txt").unwrap();
    assert_eq!(part1(&input), 6763);
    assert_eq!(part2(&input), 3512);
}
