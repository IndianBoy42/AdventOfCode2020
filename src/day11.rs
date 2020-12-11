use std::vec;

use crate::utils::*;

type Grid = FMap<(i32, i32), u8>;

fn parse_grid(input: &str) -> Grid {
    let it = input.lines().enumerate().flat_map(|(i, line)| {
        line.bytes()
            .enumerate()
            .filter(|&(j, byte)| byte != b'.')
            .map(move |(j, byte)| ((i as _, j as _), byte))
    });

    // let mut map = fmap(30_000);
    // map.extend(it);
    // map
    it.collect()
}

const OCCUPIED: u8 = b'#';
const EMPTY: u8 = b'L';

const NEIGHBOURS_STEP: [(i32, i32); 8] = [
    (1, 1),
    (0, 1),
    (-1, 1),
    (1, 0),
    (-1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn print_grid(map: &Grid) {}

const DONT_PARALLELIZE: bool = false;

fn pre_neighbours2(map: &Grid) -> FMap<(i32, i32), Vec<(i32, i32)>> {
    fn compute(map: &Grid, x: i32, y: i32) -> Vec<(i32, i32)> {
        NEIGHBOURS_STEP
            .iter()
            .map(|&(xs, ys)| {
                std::iter::successors(Some((x, y)), |&(x, y)| Some((x + xs, y + ys)))
                    .skip(1)
                    .take(100)
                    .find(|coord| map.contains_key(coord))
                    .unwrap_or_else(|| (x + xs, y + ys))
            })
            .collect()
    }

    map.par_iter()
        .map(|(&(x, y), _)| ((x, y), compute(map, x, y)))
        .collect()
}

fn step_part1_mut(map: &Grid, new: &mut Grid) {
    for (&(x, y), &tile) in map {
        let neighbours = NEIGHBOURS_STEP.iter().map(|&(i, j)| (x + i, y + j));

        let occ = neighbours
            .filter(|coord| {
                map.get(coord)
                    .map(|&tile| tile == OCCUPIED)
                    .unwrap_or(false)
            })
            .count();

        let getocc = occ == 0;
        let getempty = occ >= 4;

        match tile {
            OCCUPIED if getempty => *new.get_mut(&(x, y)).unwrap() = EMPTY,
            EMPTY if getocc => *new.get_mut(&(x, y)).unwrap() = OCCUPIED,
            _ => {}
        };
    }
}

fn step_part1(map: &Grid) -> Grid {
    if DONT_PARALLELIZE {
        let mut new = map.clone();

        step_part1_mut(map, &mut new);

        new
    } else {
        map.par_iter()
            .map(|(&(x, y), &tile)| {
                let neighbours = NEIGHBOURS_STEP.iter().map(|&(i, j)| (x + i, y + j));

                let occ = neighbours
                    .filter(|coord| {
                        map.get(coord)
                            .map(|&tile| tile == OCCUPIED)
                            .unwrap_or(false)
                    })
                    .count();

                let getocc = occ == 0;
                let getempty = occ >= 4;

                let newtile = match tile {
                    OCCUPIED if getempty => EMPTY,
                    EMPTY if getocc => OCCUPIED,
                    rest => rest,
                };

                ((x, y), newtile)
            })
            .collect()
    }
}

fn step_part2_mut(map: &Grid, new: &mut Grid, neighbours: &FMap<(i32, i32), Vec<(i32, i32)>>) {
    for (&(x, y), &tile) in map {
        let neighbours = neighbours.get(&(x, y)).unwrap().iter();

        let occ = neighbours
            .filter(|coord| {
                map.get(coord)
                    .map(|&tile| tile == OCCUPIED)
                    .unwrap_or(false)
            })
            .count();

        let getocc = occ == 0;
        let getempty = occ >= 5;

        match tile {
            OCCUPIED if getempty => *new.get_mut(&(x, y)).unwrap() = EMPTY,
            EMPTY if getocc => *new.get_mut(&(x, y)).unwrap() = OCCUPIED,
            _ => {}
        };
    }
}

fn step_part2(map: &Grid, neighbours: &FMap<(i32, i32), Vec<(i32, i32)>>) -> Grid {
    if DONT_PARALLELIZE {
        let mut new = map.clone();

        step_part2_mut(map, &mut new, neighbours);

        new
    } else {
        map.par_iter()
            .map(|(&(x, y), &tile)| {
                let neighbours = neighbours.get(&(x, y)).unwrap().iter();

                let occ = neighbours
                    .filter(|coord| {
                        map.get(coord)
                            .map(|&tile| tile == OCCUPIED)
                            .unwrap_or(false)
                    })
                    .count();

                let getocc = occ == 0;
                let getempty = occ >= 5;

                let newtile = match tile {
                    OCCUPIED if getempty => EMPTY,
                    EMPTY if getocc => OCCUPIED,
                    rest => rest,
                };

                ((x, y), newtile)
            })
            .collect()
    }
}

pub fn part1(input: &str) -> usize {
    let mut grid = parse_grid(input);

    loop {
        let new = step_part1(&grid);

        let same = izip!(&new, grid).all(|(l, r)| *l.1 == r.1);

        grid = new;

        if same {
            break;
        }
    }

    grid.values().filter(|&&t| t == OCCUPIED).count()
}

pub fn part2(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let neighbours = pre_neighbours2(&grid);

    loop {
        let new = step_part2(&grid, &neighbours);

        let same = izip!(&new, grid).all(|(l, r)| *l.1 == r.1);

        grid = new;

        if same {
            break;
        }
    }

    grid.values().filter(|&&t| t == OCCUPIED).count()
}

#[test]
fn test() {
    let input = read_input("input11.txt").unwrap();
    assert_eq!(part1(&input), 2494);
    assert_eq!(part2(&input), 2306);
}
