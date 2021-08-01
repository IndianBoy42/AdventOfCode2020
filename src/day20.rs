use std::ops::Not;
use std::vec;

use crate::u32set::U32Set;
use crate::utils::*;

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
    let tiles: FMap<usize, BitSet> = parse_tiles(input).collect();

    // Construct mappings from tile to border and border to tile
    let (toborder, fromborder) = parse_borders(tiles);
    let connected_edges = connected_edges(&fromborder);

    // This is not true in general but the input given by AoC satisfies this. This greatly simplifies the algorithm
    debug_assert!(fromborder.iter().filter(|(k, v)| v.len() > 2).count() == 0);

    // Corner tiles have only 2 connected edges
    tile_map(&toborder, &connected_edges)
        .filter(|(k, v)| v.len() == 2)
        .map(|(k, v)| k)
        .product()
}

fn tile_map<'a>(
    toborder: &'a FMap<usize, ArrayVec<[U32Set; 4]>>,
    connected_edges: &'a FSet<U32Set>,
) -> impl Iterator<Item = (usize, ArrayVec<[U32Set; 4]>)> + 'a {
    toborder.iter().map(move |(&k, v)| {
        (
            k,
            v.iter()
                .filter(|&&edge| {
                    connected_edges.contains(&edge)
                        || connected_edges.contains(&reverse_border(edge))
                })
                .copied()
                .collect(),
        )
    })
}

fn connected_edges(fromborder: &FMap<U32Set, ArrayVec<[usize; 2]>>) -> FSet<U32Set> {
    fromborder
        .iter()
        .filter(|(_, v)| v.len() >= 2)
        .map(|(&k, _)| k)
        .collect()
}

fn parse_borders(
    tiles: FMap<usize, BitSet>,
) -> (
    FMap<usize, ArrayVec<[U32Set; 4]>>,
    FMap<U32Set, ArrayVec<[usize; 2]>>,
) {
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
                    .and_modify(|v: &mut ArrayVec<[usize; 2]>| {
                        v.push(k);
                    })
                    .or_insert_with(|| [k].iter().copied().collect());
                fromborder
                    .entry(border)
                    .and_modify(|v: &mut ArrayVec<[usize; 2]>| {
                        v.push(k);
                    })
                    .or_insert_with(|| [k].iter().copied().collect());
            }
            toborder.insert(k, borders);
        }
        (toborder, fromborder)
    };
    (toborder, fromborder)
}

fn parse_tiles(input: &str) -> impl Iterator<Item = (usize, BitSet)> + '_ {
    input
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
}

pub fn part2(input: &str) -> usize {
    let tiles: FMap<usize, BitSet> = parse_tiles(input).collect();

    // Construct mappings from tile to border and border to tile
    let (toborder, fromborder) = parse_borders(tiles);
    let connected_edges = connected_edges(&fromborder);

    // This is not true in general but the input given by AoC satisfies this. This greatly simplifies the algorithm
    debug_assert!(fromborder.iter().filter(|(k, v)| v.len() > 2).count() == 0);

    let map: FMap<_, _> = tile_map(&toborder, &connected_edges).collect();
    let corner = map
        .values()
        .find_position(|&x| x.len() == 2)
        .expect("Must be a corner piece");

    const MONSTER: &str = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";

    unimplemented!()
}

#[test]
fn test() {
    let input = read_input("input20.txt").unwrap();
    // let input = read_input("test.txt").unwrap();

    assert_eq!(part1(&input), 66020135789767);
    assert_eq!(part2(&input), 0);
}
