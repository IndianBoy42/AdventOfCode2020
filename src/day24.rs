use std::mem::swap;

use regex::Regex;

use crate::utils::*;

pub fn parse(input: &str) -> FMap<(i32, i32, i32), bool> {
    let re = Regex::new(r"(e|se|sw|nw|ne|w)").unwrap();
    let lines = input.lines().map(|line| re.captures_iter(line));
    let mut floor: FMap<(i32, i32, i32), bool> = fmap(0);

    for line in lines {
        let coord = line.fold((0, 0, 0), |(x, y, z), step| {
            let step = &step[0];
            match step {
                "se" => (x + 1, y, z - 1),
                "nw" => (x - 1, y, z + 1),
                "ne" => (x, y - 1, z + 1),
                "sw" => (x, y + 1, z - 1),
                "e" => (x + 1, y - 1, z),
                "w" => (x - 1, y + 1, z),
                _ => unreachable!(),
            }
        });

        floor
            .entry(coord)
            .and_modify(|x| *x ^= true)
            .or_insert(true);
    }

    floor
}

pub fn part1(input: &str) -> usize {
    parse(input).values().filter(|&&x| x).count()
}

const DAYS: usize = 100;
const NEIGHBOURS: [(i32, i32, i32); 6] = [
    (-1, 0, 1),
    (1, 0, -1),
    (1, -1, 0),
    (-1, 1, 0),
    (0, 1, -1),
    (0, -1, 1),
];

pub fn part2(input: &str) -> usize {
    let floor = parse(input);
    let mut floor: FSet<_> = floor
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect();
    let mut floor2 = fset(0);
    let mut seen = fset(0);

    for _ in 0..DAYS {
        floor2.clear();
        seen.clear();

        let add = |tile: (i32, i32, i32), (x, y, z)| (tile.0 + x, tile.1 + y, tile.2 + z);
        let getn = |tile| NEIGHBOURS.iter().map(move |&t| add(tile, t));

        for &tile in &floor {
            // tile is black
            let count = getn(tile).fold(0, |count, neighbour| {
                if floor.contains(&neighbour) {
                    count + 1
                } else {
                    // neighbour is white
                    if seen.insert(neighbour) {
                        let count2 = getn(neighbour)
                            // .filter(|&n| n != tile)
                            .filter(|n| floor.contains(n))
                            // .take(2)
                            .count();
                        if count2 == 2 {
                            floor2.insert(neighbour);
                        }
                    }

                    count
                }
            });
            if (1..=2).contains(&count) {
                floor2.insert(tile);
            }
        }

        swap(&mut floor, &mut floor2);
    }

    floor.len()
}

#[test]
fn test() {
    let input = read_input("input24.txt").unwrap();
    assert_eq!(part1(&input), 269);
    assert_eq!(part2(&input), 3667);
}
