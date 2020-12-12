use crate::utils::*;

fn step((x, y): &mut (i32, i32), stepdir: u8, stepmag: i32) {
    match stepdir {
        b'N' => *x += stepmag,
        b'S' => *x -= stepmag,
        b'E' => *y += stepmag,
        b'W' => *y -= stepmag,
        _ => unreachable!(),
    }
}
fn rot(dir: u8, deg: i32, rotdir: u8) -> u8 {
    const DIRS: [u8; 4] = [b'N', b'W', b'S', b'E'];
    let idx = DIRS.iter().copied().position(|x| x == dir).unwrap() as i32;
    let rotsteps = deg / 90;
    match rotdir {
        b'L' => DIRS[(idx + rotsteps) as usize % 4],
        b'R' => DIRS[(idx + 4 - rotsteps) as usize % 4],
        _ => unreachable!(),
    }
}

pub fn part1(input: &str) -> i32 {
    let (dir, ship) = input
        .lines()
        .map(|line| line.split_at(1))
        .map(|(l, r)| (l.as_bytes()[0], r.parse().unwrap()))
        .fold((b'E', (0, 0)), |(mut dir, mut pos), (stepdir, stepmag)| {
            match stepdir {
                b'N' | b'S' | b'E' | b'W' => step(&mut pos, stepdir, stepmag),
                b'L' | b'R' => dir = rot(dir, stepmag, stepdir),
                b'F' => {
                    let stepdir = dir;
                    step(&mut pos, stepdir, stepmag)
                }
                _ => unreachable!(),
            };
            (dir, pos)
        });
    ship.0.abs() + ship.1.abs()
}

fn rotwp((n, e): (i32, i32), deg: i32, dir: u8) -> (i32, i32) {
    let deg = deg / 90;
    let deg = if dir == b'R' { -deg } else { deg };
    let deg = (4 + deg) % 4;
    match deg {
        0 => (n, e),
        1 => (e, -n),
        2 => (-n, -e),
        3 => (-e, n),
        _ => unreachable!(),
    }
}

pub fn part2(input: &str) -> i32 {
    let (wp, ship) = input
        .lines()
        .map(|line| line.split_at(1))
        .map(|(l, r)| (l.as_bytes()[0], r.parse().unwrap()))
        .fold(((1, 10), (0, 0)), |(mut wp, mut ship), (dir, mag)| {
            match dir {
                b'N' | b'S' | b'E' | b'W' => step(&mut wp, dir, mag),
                b'L' | b'R' => wp = rotwp(wp, mag, dir),
                b'F' => ship = (ship.0 + wp.0 * mag, ship.1 + wp.1 * mag),
                _ => unreachable!(),
            };
            (wp, ship)
        });
    ship.0.abs() + ship.1.abs()
}

#[test]
fn test() {
    let input = read_input("input12.txt").unwrap();
    assert_eq!(part1(&input), 562);
    assert_eq!(part2(&input), 101860);
}
