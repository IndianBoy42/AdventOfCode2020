use std::mem::swap;
use std::vec;

use ndarray::{azip, par_azip, s, Array2};

use crate::grid::Grid2D;
use crate::utils::*;

type Grid = Grid2D<u8>;
// type Grid = FMap<(i32, i32), u8>;
// wrap Vec<Vec<>> or NDArray for the Grid (should make a library)

fn parse_grid(input: &str) -> Grid {
    Grid::from_iter_w_shape(
        (
            input.lines().count(),
            input.lines().next().unwrap().as_bytes().len(),
        ),
        input.lines().map(str::bytes).flatten().map(|b| match b {
            b'.' => 0,
            b => b,
        }),
    )

    // let grid: Grid2D<u8> = input
    //     .lines()
    //     .enumerate()
    //     .flat_map(|(i, line)| {
    //         line.bytes()
    //             .enumerate()
    //             .map(move |(j, byte)| ((i as _, j as _), byte))
    //     })
    //     .filter(|&(j, byte)| byte != b'.').collect();
    // grid
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

    map.iter()
        .map(|((x, y), _)| ((x, y), compute(map, x, y)))
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

    map.iter()
        .map(|((x, y), _)| ((x, y), compute(map, x, y)))
        .collect()
}

fn step_part1_mut(map: &Grid, new: &mut Grid) {
    let (X, Y) = {
        let sh = map.arr.shape();
        (sh[0], sh[1])
    };

    let f = |((x, y), &tile): ((i32, i32), _)| {
        let occ = || {
            NEIGHBOURS_STEP
                .iter()
                .map(|&(i, j)| (x + i, y + j))
                .filter_map(|coord| map.get(&coord))
                .filter(|&&tile| tile == OCCUPIED)
                .count()
        };
        let winc = || occ();

        match tile {
            0 => 0,
            OCCUPIED => {
                if occ() >= 4 {
                    EMPTY
                } else {
                    OCCUPIED
                }
            }
            EMPTY => {
                if winc() == 0 {
                    OCCUPIED
                } else {
                    EMPTY
                }
            }
            rest => rest,
            // _ => continue,
        }
    };

    par_azip!((index (x,y), tile in &map.arr, new in &mut new.arr) {
        if *tile != 0 {*new = f(((x as _,y as _), tile));}
    });
}

fn step_part2_mut(map: &Grid, new: &mut Grid, neighbours: &FMap<(i32, i32), Vec<(i32, i32)>>) {
    let f = |((x, y), &tile)| {
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

    par_azip!((index (x,y), tile in &map.arr, new in &mut new.arr) {
        if *tile != 0 {*new = f(((x as _,y as _), tile));}
    });
}

pub fn part1(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let mut grid2 = grid.clone();

    loop {
        step_part1_mut(&grid, &mut grid2);
        step_part1_mut(&grid2, &mut grid);
        // swap(&mut grid, &mut grid2);

        let same = ndarray::Zip::from(&grid.arr)
            .and(&grid2.arr)
            .into_par_iter()
            // .fold(true, |acc, &a, &b| acc && (a == b));
            .all(|(&a, &b)| a == b);
        // .map(|(&a, &b)| (a == b))
        // .reduce(|| true, |a, b| a && b)

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
        step_part2_mut(&grid, &mut grid2, &neighbours);
        step_part2_mut(&grid2, &mut grid, &neighbours);
        // swap(&mut grid, &mut grid2);

        let same = ndarray::Zip::from(&grid.arr)
            .and(&grid2.arr)
            .into_par_iter()
            // .fold(true, |acc, &a, &b| acc && (a == b));
            .all(|(&a, &b)| a == b);
        // .map(|(&a, &b)| (a == b))
        // .reduce(|| true, |a, b| a && b);

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
