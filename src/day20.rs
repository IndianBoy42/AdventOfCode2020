use std::ops::Not;
use std::vec;

use crate::u32set::U32Set;
use crate::utils::*;

const N: usize = 10;

fn extract_border(s: &BitSet) -> ArrayVec<[U32Set; 8]> {
    let mut vec = ArrayVec::new();

    let top: U32Set = (0..N).map(|n| s.contains(n)).collect();
    vec.push(top);
    vec.push(top.revn(10));

    let btm: U32Set = (0..N).map(|n| s.contains(N * (N - 1) + n)).collect();
    vec.push(btm);
    vec.push(btm.revn(10));

    let lft: U32Set = (0..N).map(|n| s.contains(n * N)).collect();
    vec.push(lft);
    vec.push(lft.revn(10));

    let rgt: U32Set = (0..N).map(|n| s.contains(N + n * N)).collect();
    vec.push(rgt);
    vec.push(rgt.revn(10));

    // vec.sort();

    vec
}

pub fn part1(input: &str) -> usize {
    let tiles: FMap<usize, BitSet> = input
        .split("\n\n")
        .filter_map(|tile| tile.trim().is_empty().not().as_some(tile.lines()))
        .map(|mut tilelines| (tilelines.next(), tilelines))
        .map(|(header, tile)| {
            (
                header
                    .and_then(|header| header.split_once(' '))
                    .and_then(|(_, num)| num.strip_suffix(':'))
                    .and_then(|num| num.parse().ok())
                    .unwrap(),
                tile.flat_map(|line| line.bytes())
                    .enumerate()
                    .filter_map(|(i, b)| (b == b'#').as_some(i))
                    .collect(),
            )
        })
        .collect();

    let borders = tiles
        .iter()
        .map(|(&k, v)| (k, extract_border(v)))
        .collect_vec();
    let (toborder, fromborder) = {
        let mut toborder = fmap(borders.len());
        let mut fromborder = fmap(borders.len());
        for (k, borders) in borders {
            for &border in &borders {
                fromborder
                    .entry(border)
                    .and_modify(|v: &mut Vec<usize>| {
                        v.push(k);
                    })
                    .or_insert_with(|| vec![k]);
            }
            toborder.insert(k, borders);
        }
        (toborder, fromborder)
    };

    unimplemented!()
}

pub fn part2(input: &str) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let input = read_input("input20.txt").unwrap();
    assert_eq!(part1(&input), 0);
    assert_eq!(part2(&input), 0);
}
