use std::collections::VecDeque;

use crate::utils::*;

pub fn part1(input: &str) -> u32 {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let f = |p: &str| -> VecDeque<_> {
        p.lines()
            .skip(1)
            .map(u32::from_str)
            .map(Result::unwrap)
            .collect()
    };

    let (mut p1, mut p2) = (f(p1), f(p2));
    while !p1.is_empty() && !p2.is_empty() {
        // dbg!((&p1,&p2));
        let (c1, c2) = p1.pop_front().zip(p2.pop_front()).unwrap();
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    let win = if p1.is_empty() { p2 } else { p1 };
    win.into_iter().rev().zip(1..).map(|(a, b)| a * b).sum()
}

fn play(mut p1: VecDeque<u32>, mut p2: VecDeque<u32>) -> (bool, VecDeque<u32>) {
    let mut set = fset(0);

    while !p1.is_empty() && !p2.is_empty() {
        if !set.insert((p1.clone(), p2.clone())) {
            return (true, p1);
        }

        let (c1, c2) = p1.pop_front().zip(p2.pop_front()).unwrap();

        let p1win = if p1.len() >= (c1 as usize) && p2.len() >= (c2 as usize) {
            play(
                p1.iter().copied().take(c1 as usize).collect(),
                p2.iter().copied().take(c2 as usize).collect(),
            )
            .0
        } else {
            c1 > c2
        };

        if p1win {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    if p1.is_empty() {
        (false, p2)
    } else {
        (true, p1)
    }
}

pub fn part2(input: &str) -> u32 {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let f = |p: &str| -> VecDeque<_> {
        p.lines()
            .skip(1)
            .map(u32::from_str)
            .map(Result::unwrap)
            .collect()
    };

    let (p1, p2) = (f(p1), f(p2));
    let (_, hand) = play(p1, p2);

    hand.into_iter().rev().zip(1..).map(|(a, b)| a * b).sum()
}

#[test]
fn test() {
    let input = read_input("input22.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part1(&input), 33694);
    assert_eq!(part2(&input), 31835);
}
