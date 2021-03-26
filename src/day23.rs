use std::collections::VecDeque;

use crate::utils::*;

fn print_nodes(curr: u32, nodes: &[u32]) {
    // dbg!(&nodes[1..10]);
    let mut curr = curr;
    for _ in 0..10 {
        curr = nodes[curr as usize];
        print!("{},", curr);
    }
    println!("");
}
fn game(input: &str, max: u32, moves: usize) -> Vec<u32> {
    let input = input.as_bytes();

    let min = 1;
    let mut nodes = {
        let mut nodes = (1..(max + 2)).collect_vec();
        // let mut nodes = (0..=max).map(|x| (x + 1)).collect_vec();
        for &[node, next] in input.array_windows() {
            // use puzzle input for first few nodes
            nodes[(node - b'0') as usize] = (next - b'0') as u32;
        }
        // Make circular
        if max >= 10 {
            *nodes.last_mut().unwrap() = (input.first().unwrap() - b'0') as _;
            nodes[(input.last().unwrap() - b'0') as usize] = 10;
        } else {
            nodes[(input.last().unwrap() - b'0') as usize] = (input.first().unwrap() - b'0') as _;
        }

        nodes
    };
    let mut curr = (input.first().unwrap() - b'0') as _;

    // print_nodes(curr, &nodes);
    for i in 0..moves {
        // let a = nodes[curr as usize];
        // let b = nodes[a as usize];
        // let c = nodes[b as usize];
        // let d = nodes[c as usize];
        let (a, b, c, d) = unsafe {
            let a = *nodes.get_unchecked(curr as usize);
            let b = *nodes.get_unchecked(a as usize);
            let c = *nodes.get_unchecked(b as usize);
            let d = *nodes.get_unchecked(c as usize);
            (a, b, c, d)
        };

        // remove a,b,c (connect curr to d)
        // nodes[curr as usize] = d;
        unsafe {
            *nodes.get_unchecked_mut(curr as usize) = d;
        }

        // Find destination
        let searchin = |rng: std::ops::Range<u32>| {
            rng.rev()
                .filter(|&j| j != a)
                .filter(|&j| j != b)
                .filter(|&j| j != c)
                .next()
        };
        let search = (min..curr).rev();
        let dest = searchin(min..curr).or_else(|| searchin(curr..max)).unwrap();

        // dest -> (a -> b -> c) -> next(dest)
        unsafe {
            *nodes.get_unchecked_mut(c as usize) =
                std::mem::replace(nodes.get_unchecked_mut(dest as usize), a);
        }
        // nodes[c as usize] = std::mem::replace(&mut nodes[dest as usize], a)

        // new curr
        // curr = nodes[curr as usize];
        unsafe {
            curr = *nodes.get_unchecked(curr as usize);
        }

        // print_nodes(curr, &nodes);
    }

    nodes
}

pub fn part1(input: &str) -> u32 {
    let nodes = game(input, 9, 100);

    let mut out = 0;
    let mut curr = 1;
    for _ in 0..8 {
        curr = nodes[curr as usize];
        out = out * 10 + curr;
    }

    out
}
pub fn part1_deq(input: &str) -> String {
    let mut cups: VecDeque<_> = input.bytes().map(|x| x as usize).collect();
    let (&min, &max) = minmax(&cups).unwrap();

    // let out = |cups: &VecDeque<_>| cups.into_iter().map(|&x| x as u8 as char).join("");

    cups.rotate_left(1);
    for _ in 0..100 {
        let curr = *cups.back().unwrap();
        let (a, b, c) = (
            cups.pop_front().unwrap(),
            cups.pop_front().unwrap(),
            cups.pop_front().unwrap(),
        );

        let search = (min..curr).rev().chain((curr..=max).rev());
        let destination = mov(search)
            .find_map(|i| cups.iter().position(|&cup| cup == i))
            .unwrap();
        // dbg!(out( &cups ), destination, cups[destination] as u8 as char);
        cups.rotate_left(destination + 1);
        cups.push_front(c);
        cups.push_front(b);
        cups.push_front(a);
        cups.rotate_right(destination);

        // dbg!(out(&cups));
    }

    cups.iter()
        .cycle()
        .skip_while(|&&cup| cup as u8 != b'1')
        .skip(1)
        .take(cups.len() - 1)
        .map(|&x| x as u8 as char)
        .join("")
}

pub fn part2(input: &str) -> u64 {
    let nodes = game(input, 1_000_000, 10_000_000);

    let a = nodes[1];
    let b = nodes[a as usize];
    (a as u64) * (b as u64)
}

#[test]
fn test() {
    let input = "389125467".to_owned();
    assert_eq!(part1(&input), 67384529);
    assert_eq!(part2(&input), 149245887792);

    let input = read_input("input23.txt").unwrap();
    assert_eq!(part1(&input), 49576328);
    assert_eq!(part2(&input), 511780369955);
}
