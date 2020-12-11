use std::mem::swap;
use std::vec;

use crate::utils::*;

type Grid = FMap<(i32, i32), u8>;
// wrap Vec<Vec<>> or NDArray for the Grid (should make a library)
 
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

fn pre_neighbours2(map: &Grid) -> FMap<(i32, i32), Vec<(i32, i32)>> {
    fn compute(map: &Grid, x: i32, y: i32) -> Vec<(i32, i32)> {
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

fn pre_neighbours1(map: &Grid) -> FMap<(i32, i32), Vec<(i32, i32)>> {
    fn compute(map: &Grid, x: i32, y: i32) -> Vec<(i32, i32)> {
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
    let mut chg = false;

    let mut f = |(&(x, y), &tile), new: &mut u8| {
        let neighbours = NEIGHBOURS_STEP.iter().map(|&(i, j)| (x + i, y + j));

        let occ = neighbours
            .filter_map(|coord| map.get(&coord))
            .filter(|&&tile| tile == OCCUPIED)
            .count();

        let getocc = occ == 0;
        let getempty = occ >= 4;

        *new = match tile {
            OCCUPIED if getempty => {
                chg |= true;
                EMPTY
            }
            EMPTY if getocc => {
                chg |= true;
                OCCUPIED
            }
            rest => rest,
            // _ => continue,
        };
    };

    for e @ (&(x, y), _) in map {
        f(e, new.get_mut(&(x, y)).unwrap());
    }

    chg
}

fn step_part2_mut(
    map: &Grid,
    new: &mut Grid,
    neighbours: &FMap<(i32, i32), Vec<(i32, i32)>>,
) -> bool {
    let mut chg = false;

    let mut f = |(&(x, y), &tile), new: &mut u8| {
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

        *new = match tile {
            OCCUPIED if getempty => {
                chg |= true;
                EMPTY
            }
            EMPTY if getocc => {
                chg |= true;
                OCCUPIED
            }
            rest => rest,
            // _ => continue,
        };
    };

    for e @ (&(x, y), _) in map {
        f(e, new.get_mut(&(x, y)).unwrap());
    }

    chg
}

fn step_part1(map: &Grid) -> (Grid, bool) {
    let mut new = map.clone();

    let chg = step_part1_mut(map, &mut new);

    (new, chg)
}

fn step_part2(map: &Grid, neighbours: &FMap<(i32, i32), Vec<(i32, i32)>>) -> (Grid, bool) {
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
