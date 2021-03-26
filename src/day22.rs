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

type Hands = (ArrayVec<[u8; 8]>, ArrayVec<[u8; 8]>);
fn play(mut p1: VecDeque<u8>, mut p2: VecDeque<u8>) -> (bool, VecDeque<u8>) {
    fn play_util(
        p1: &mut VecDeque<u8>,
        p2: &mut VecDeque<u8>,
        previously_played: &mut FMap<Hands, bool>,
    ) -> bool {
        // if let Some(&b) = previously_played.get(&(p1.clone(), p2.clone())) {
        //     return b;
        // }
        let mut set: FSet<Hands> = fset(0);

        while !p1.is_empty() && !p2.is_empty() {
            let cloned = (p1.iter().copied().collect(), p2.iter().copied().collect());
            // if let Some(&b) = previously_played.get(&cloned) {
            //     // previously_played.extend(set.into_iter().map(|x| (x, b)));
            //     return b;
            // }
            if !set.insert(cloned) {
                // previously_played.extend(set.into_iter().map(|x| (x, true)));
                return true;
            }

            let c1 = p1.pop_front().unwrap();
            let c2 = p2.pop_front().unwrap();

            let p1win = if p1.len() >= (c1 as usize) && p2.len() >= (c2 as usize) {
                let mut p1 = p1.iter().copied().take(c1 as usize).collect();
                let mut p2 = p2.iter().copied().take(c2 as usize).collect();
                play_util(&mut p1, &mut p2, previously_played)
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

        let ret = !p1.is_empty();
        // previously_played.extend(set.into_iter().map(|x| (x, ret)));
        return ret;
    }

    let mut previously_played = fmap(0);

    if play_util(&mut p1, &mut p2, &mut previously_played) {
        (true, p1)
    } else {
        (false, p2)
    }
}

pub fn part2(input: &str) -> usize {
    let (p1, p2) = input.split_once("\n\n").unwrap();
    let f = |p: &str| -> VecDeque<_> {
        p.lines()
            .skip(1)
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .collect()
    };

    let (p1, p2) = (f(p1), f(p2));
    let (_, hand) = play(p1, p2);

    hand.into_iter()
        .rev()
        .zip(1..)
        .map(|(a, b)| (a as usize) * (b as usize))
        .sum()
}

#[test]
fn test() {
    let input = read_input("input22.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part1(&input), 33694);
    assert_eq!(part2(&input), 31835);
}
