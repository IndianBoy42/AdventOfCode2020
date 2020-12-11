use std::mem::swap;
use std::vec;

use crate::grid::Grid2D;
use crate::utils::*;

// type Grid = Grid2D<u8>;
type Grid = FMap<(i8, i8), u8>;
// wrap Vec<Vec<>> or NDArray for the Grid (should make a library)

fn parse_grid(input: &str) -> Grid {
    let it = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.bytes()
                .enumerate()
                .map(move |(j, byte)| ((i as _, j as _), byte))
        })
        .filter(|&(j, byte)| byte != b'.');

    // let mut map = fmap(30_000);
    // map.extend(it);
    // map
    it.collect()
}

const OCCUPIED: u8 = b'#';
const EMPTY: u8 = b'L';

const NEIGHBOURS_STEP: [(i8, i8); 8] = [
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

fn pre_neighbours2(map: &Grid) -> FMap<(i8, i8), Vec<(i8, i8)>> {
    fn compute(map: &Grid, x: i8, y: i8) -> Vec<(i8, i8)> {
        NEIGHBOURS_STEP
            .iter()
            .filter_map(|&(xs, ys)| {
                std::iter::successors(Some((x + xs, y + ys)), |&(x, y)| Some((x + xs, y + ys)))
                    // .take(100)
                    .take_while(|(x, y)| (0..100).contains(x) && (0..100).contains(y))
                    .find(|coord| map.contains_key(coord))
                // .unwrap_or_else(|| (x + xs, y + ys))
            })
            .fold(Vec::with_capacity(8), |mut acc, v| {
                acc.push(v);
                acc
            })
        // .collect()
    }

    map.par_iter()
        .map(|(&(x, y), _)| ((x, y), compute(map, x, y)))
        .collect()
}

fn pre_neighbours1(map: &Grid) -> FMap<(i8, i8), Vec<(i8, i8)>> {
    fn compute(map: &Grid, x: i8, y: i8) -> Vec<(i8, i8)> {
        NEIGHBOURS_STEP
            .iter()
            .filter_map(|&(xs, ys)| {
                std::iter::successors(Some((x + xs, y + ys)), |&(x, y)| Some((x + xs, y + ys)))
                    // .take(100)
                    .take_while(|(x, y)| (0..100).contains(x) && (0..100).contains(y))
                    .find(|coord| map.contains_key(coord))
                // .unwrap_or_else(|| (x + xs, y + ys))
            })
            .fold(Vec::with_capacity(8), |mut acc, v| {
                acc.push(v);
                acc
            })
        // .collect()
    }

    map.par_iter()
        .map(|(&(x, y), _)| ((x, y), compute(map, x, y)))
        .collect()
}

fn step_part1_mut(map: &Grid, new: &mut Grid) -> bool {
    let f = |(&(x, y), &tile)| {
        let occ = || {
            NEIGHBOURS_STEP
                .iter()
                .map(|&(i, j)| (x + i, y + j))
                .filter_map(|coord| map.get(&coord))
                .filter(|&&tile| tile == OCCUPIED)
                .count()
        };

        match tile {
            OCCUPIED => {
                if occ() >= 4 {
                    EMPTY
                } else {
                    OCCUPIED
                }
            }
            EMPTY => {
                if occ() == 0 {
                    OCCUPIED
                } else {
                    EMPTY
                }
            }
            rest => rest,
            // _ => continue,
        }
    };

    for e @ (&(x, y), _) in map {
        *new.get_mut(&(x, y)).unwrap() = f(e);
    }

    izip!(map, new)
        .filter(|((_, &old), (_, &mut new))| old != new)
        .count()
        != 0
}

fn step_part2_mut(map: &Grid, new: &mut Grid, neighbours: &FMap<(i8, i8), Vec<(i8, i8)>>) -> bool {
    let f = |(&(x, y), &tile)| {
        let occ = || {
            neighbours
                .get(&(x, y))
                .unwrap()
                .iter()
                .filter(|coord| {
                    map.get(coord)
                        .map(|&tile| tile == OCCUPIED)
                        .unwrap_or(false)
                })
                .count()
        };

        match tile {
            OCCUPIED => {
                if occ() >= 5 {
                    EMPTY
                } else {
                    OCCUPIED
                }
            }
            EMPTY => {
                if occ() == 0 {
                    OCCUPIED
                } else {
                    EMPTY
                }
            }
            rest => rest,
            // _ => continue,
        }
    };

    for e @ (&(x, y), _) in map {
        *new.get_mut(&(x, y)).unwrap() = f(e);
    }

    izip!(map, new)
        .filter(|((_, &old), (_, &mut new))| old != new)
        .count()
        != 0
}

fn step_part1(map: &Grid) -> (Grid, bool) {
    let mut new = map.clone();

    let chg = step_part1_mut(map, &mut new);

    (new, chg)
}

fn step_part2(map: &Grid, neighbours: &FMap<(i8, i8), Vec<(i8, i8)>>) -> (Grid, bool) {
    let mut new = map.clone();

    let chg = step_part2_mut(map, &mut new, neighbours);

    (new, chg)
}

pub fn part1(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let mut grid2 = grid.clone();

    loop {
        // let mut (grid2, chgd) = step_part1(&grid);
        let chgd = step_part1_mut(&grid, &mut grid2);

        // let same = izip!(&grid, &grid2).all(|(l, r)| l.1 == r.1);
        let same = !chgd;

        swap(&mut grid, &mut grid2);

        if same {
            break;
        }
    }

    grid.values().filter(|&&t| t == OCCUPIED).count()
}

pub fn part2(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let mut grid2 = grid.clone();

    let neighbours = pre_neighbours2(&grid);

    loop {
        // let mut (grid2, chgd) = step_part2(&grid, &neighbours);
        let chgd = step_part2_mut(&grid, &mut grid2, &neighbours);

        // let same = izip!(&grid2, &grid).all(|(l, r)| l.1 == r.1);
        let same = !chgd;

        swap(&mut grid, &mut grid2);

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
