use std::mem::swap;

use crate::utils::*;

type Coord = (i16, i16, i16);

pub fn part1(input: &str) -> usize {
    let mut map: FSet<(i16, i16, i16)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, b)| b != b'.')
                .map(move |(j, _)| (i as _, j as _, 0))
        })
        .collect();

    let neighbours = iproduct!(-1..=1, -1..=1, -1..=1)
        .filter(|&(x, y, z)| x != 0 || y != 0 || z != 0)
        .collect_vec();

    let it_neighbours = |(xi, yi, zi)| {
        neighbours
            .iter()
            .map(move |&(dx, dy, dz)| (xi + dx, yi + dy, zi + dz))
    };

    let mut newmap = fset(20001);
    let mut countmap = fmap(20001);
    map.reserve(20001);
    for _ in 0..6 {
        newmap.clear();
        countmap.clear();

        for &i in &map {
            let active_neighbours = it_neighbours(i)
                .inspect(|&n| {
                    countmap.entry(n).and_modify(|v| *v += 1).or_insert(1);
                })
                .filter(|n| map.contains(n))
                .count();
            if active_neighbours == 2 || active_neighbours == 3 {
                newmap.insert(i);
            }
        }
        let get_active = countmap.iter().filter(|&(&c, &v)| v == 3).map(|(&c, _)| c);
        newmap.extend(get_active);

        swap(&mut map, &mut newmap);
        // dbg!(map.len());
    }

    map.len()
}

pub fn part2(input: &str) -> usize {
    let mut map: FSet<(i16, i16, i16, i16)> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_, b)| b != b'.')
                .map(move |(j, _)| (i as _, j as _, 0, 0))
        })
        .collect();

    let neighbours = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
        .filter(|&(x, y, z, w)| x != 0 || y != 0 || z != 0 || w != 0)
        .collect_vec();

    let it_neighbours = |(xi, yi, zi, wi)| {
        neighbours
            .iter()
            .map(move |&(dx, dy, dz, dw)| (xi + dx, yi + dy, zi + dz, wi + dw))
    };

    let mut newmap = fset(20001);
    let mut countmap = fmap(20001);
    map.reserve(20001);
    for _ in 0..6 {
        newmap.clear();
        countmap.clear();

        for &i in &map {
            let active_neighbours = it_neighbours(i)
                .inspect(|&n| {
                    countmap.entry(n).and_modify(|v| *v += 1).or_insert(1);
                })
                .filter(|n| map.contains(n))
                .count();
            if active_neighbours == 2 || active_neighbours == 3 {
                newmap.insert(i);
            }
        }
        let get_active = countmap.iter().filter(|&(&c, &v)| v == 3).map(|(&c, _)| c);
        newmap.extend(get_active);

        swap(&mut map, &mut newmap);
        // dbg!(map.len());
    }

    map.len()
}

#[test]
fn test() {
    let input = read_input("input17.txt").unwrap();
    assert_eq!(part1(&input), 336);
    assert_eq!(part2(&input), 2620);
}
