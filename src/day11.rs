use std::vec;

use crate::utils::*;

type Grid = FMap<(i32, i32), u8>;

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(j, byte)| byte != b'.')
                .map(move |(j, byte)| ((i as _, j as _), byte))
        })
        .collect::<FMap<_, _>>()
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

fn step(map: &Grid) -> Grid {
    let mut new = map.clone();

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

    new
}

pub fn part1(input: &str) -> usize {
    let mut grid = parse_grid(input);
    // dbg!(&grid);

    loop {
        let new = step(&grid);

        // dbg!(&grid);
        let same = izip!(&new, grid).all(|(l, r)| *l.1 == r.1);
        // dbg!(same);

        grid = new;

        if same {
            break;
        }
    }

    grid.values().filter(|&&t| t == OCCUPIED).count()
}

fn pre_neighbours2(map: &Grid) -> FMap<(i32, i32), Vec<(i32, i32)>> {
    fn compute(map: &Grid, x: i32, y: i32) -> Vec<(i32, i32)> {
        let mut dirs = vec![
            (1, 1),
            (0, 1),
            (-1, 1),
            (1, 0),
            (-1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];

        for dir in &mut dirs {
            let (xs, ys) = *dir;

            *dir = std::iter::successors(Some((x, y)), |&(x, y)| Some((x + xs, y + ys)))
                .skip(1)
                .take(100)
                .find(|coord| map.contains_key(coord))
                .unwrap_or_else(|| (x + xs, y + ys));
        }

        dirs
    }

    map.iter()
        .map(|(&(x, y), _)| ((x, y), compute(map, x, y)))
        .collect()
}

fn neighbours2(n: &FMap<(i32, i32), Vec<(i32, i32)>>, x: i32, y: i32) -> &[(i32, i32)] {
    &n.get(&(x, y)).unwrap()
}

fn step2(map: &Grid, neighbours: &FMap<(i32, i32), Vec<(i32, i32)>>) -> Grid {
    let mut new = map.clone();

    for (&(x, y), &tile) in map {
        let neighbours = neighbours2(neighbours, x, y);

        let occ = neighbours
            .into_iter()
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

    new
}

pub fn part2(input: &str) -> usize {
    let mut grid = parse_grid(input);
    let neighbours = pre_neighbours2(&grid);
    // dbg!(&grid);

    loop {
        let new = step2(&grid, &neighbours);

        // dbg!(&grid);
        let same = izip!(&new, grid).all(|(l, r)| *l.1 == r.1);
        // dbg!(same);
        println!(".");

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
    // assert_eq!(part1(&input), 2494);
    assert_eq!(part2(&input), 10934875012);
}
