use std::ops::Not;
use std::vec;

use crate::u32set::U32Set;
use crate::utils::*;

use smallvec::{smallvec, SmallVec};

const N: usize = 10;

fn reverse_border(s: U32Set) -> U32Set {
    s.revn(10)
}

fn extract_border(s: &BitSet) -> ArrayVec<[U32Set; 4]> {
    let mut vec = ArrayVec::new();

    let top: U32Set = (0..N).map(|n| s.contains(n)).collect();
    vec.push(top);
    // vec.push(top.revn(10));

    let btm: U32Set = (0..N).map(|n| s.contains(N * (N - 1) + n)).collect();
    vec.push(btm);
    // vec.push(btm.revn(10));

    let lft: U32Set = (0..N).map(|n| s.contains(n * N)).collect();
    vec.push(lft);
    // vec.push(lft.revn(10));

    let rgt: U32Set = (0..N).map(|n| s.contains(N - 1 + n * N)).collect();
    vec.push(rgt);
    // vec.push(rgt.revn(10));

    // vec.sort();

    vec
}

pub fn part1(input: &str) -> usize {
    let tiles: FMap<usize, BitSet> = input
        // Split by blank lines
        .split("\n\n")
        // Split each tile to lines
        .filter_map(|tile| tile.trim().is_empty().not().as_some(tile.lines()))
        // Extract first line (id)
        .map(|mut tilelines| (tilelines.next(), tilelines))
        .map(|(header, tile)| {
            (
                // parse the header
                header
                    .and_then(|header| header.split_once(' '))
                    .and_then(|(_, num)| num.strip_suffix(':'))
                    .and_then(|num| num.parse().ok())
                    .unwrap(),
                // Parse the tile into bitmap
                tile.flat_map(|line| line.bytes())
                    .enumerate()
                    .filter_map(|(i, b)| (b == b'#').as_some(i))
                    .collect(),
            )
        })
        .collect();

    // Construct mappings from tile to border and border to tile
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
                    .entry(reverse_border(border))
                    .and_modify(|v: &mut SmallVec<[usize; 2]>| {
                        v.push(k);
                    })
                    .or_insert_with(|| smallvec![k]);
                fromborder
                    .entry(border)
                    .and_modify(|v: &mut SmallVec<[usize; 2]>| {
                        v.push(k);
                    })
                    .or_insert_with(|| smallvec![k]);
            }
            toborder.insert(k, borders);
        }
        (toborder, fromborder)
    };

    let connected_edges: FSet<_> = fromborder
        .iter()
        .filter(|(k, v)| v.len() >= 2)
        .map(|e| dbg!(e))
        .map(|(k, v)| k)
        .collect();
    dbg!(&toborder, &fromborder);

    // This is not true in general but the input given by AoC satisfies this. This greatly simplifies the algorithm
    debug_assert!(fromborder.iter().filter(|(k, v)| v.len() > 2).count() == 0);

    let tilemap: FMap<_, _> = toborder
        .iter()
        .map(|(k, v)| {
            (
                k,
                v.iter()
                    .filter(|&&edge| {
                        connected_edges.contains(&edge)
                            || connected_edges.contains(&reverse_border(edge))
                    })
                    .collect_vec(),
            )
        })
        .collect();
    dbg!(&tilemap);

    // Corner tiles have only 2 connected edges
    tilemap
        .iter()
        .filter(|(k, v)| v.len() == 2)
        .map(|(&k, v)| k)
        .product()
}

pub fn part2(input: &str) -> usize {
    unimplemented!()
}

#[test]
fn test() {
    let input = read_input("input20.txt").unwrap();
    // let input = read_input("test.txt").unwrap();

    assert_eq!(part1(&input), 66020135789767);
    assert_eq!(part2(&input), 0);
}
