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

    let neighbours = iproduct!(-1..=1, -1..=1, -1..=1).collect_vec();

    let it_neighbours = |(xi, yi, zi)| {
        neighbours
            .iter()
            .map(move |&(dx, dy, dz)| (xi + dx, yi + dy, zi + dz))
    };

    for _ in 0..6 {
        let mut newmap = fset(map.len() * 2);

        let coords = map.iter().flat_map(|&i| it_neighbours(i));

        for i in coords {
            let active_neighbours = it_neighbours(i).filter(|n| map.contains(n)).count();
            let curr_active = map.contains(&i);
            let stay_active = || curr_active && (3..=4).contains(&active_neighbours);
            let get_active = || !curr_active && active_neighbours == 3;
            if stay_active() || get_active() {
                newmap.insert(i);
            }
        }

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

    let mut coordset = fset(20001);
    let mut newmap = fset(20001);
    // let mut countmap = fmap(20001);
    map.reserve(20001);
    for _ in 0..6 {
        newmap.clear();
        // newmap.reserve(map.len() * 10);

        let coords = map.iter().flat_map(|&i| it_neighbours(i));
        coordset.clear();
        coordset.extend(coords);
        let coords = &coordset;
        // let coords = coords.collect::<FSet<_>>();

        for &i in &map {
            let active_neighbours = it_neighbours(i).filter(|n| map.contains(n)).take(4).count();
            if active_neighbours == 2 || active_neighbours == 3 {
                newmap.insert(i);
            }
        }
        for &i in coords {
            let active_neighbours = it_neighbours(i).filter(|n| map.contains(n)).take(4).count();
            if active_neighbours==3 {
                newmap.insert(i);
            }
        }

        swap(&mut map, &mut newmap);
        // dbg!(map.len());
    }

    map.len()
}

#[test]
fn test() {
    let input = read_input("input17.txt").unwrap();
    // assert_eq!(part1(&input), 336);
    assert_eq!(part2(&input), 2620);
}
