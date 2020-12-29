use crate::utils::*;

const PUBK: usize = 7;
const REM: usize = 20201227;

fn step(x: usize, subj: usize) -> usize {
    (x * subj) % REM
}
fn transform(loopsize: usize, sub: usize) -> usize {
    (0..loopsize).fold(1, |acc, _| step(acc, sub))
}
fn find_loopsize(pubkey: usize) -> usize {
    (1..)
        .scan(1, |st, i| {
            *st = step(*st, PUBK);
            Some((i, *st ))
        })
        .find(|&(i, num)| num == pubkey)
        .unwrap().0
}

pub fn part1(input: &str) -> usize {
    let (a, b) = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_tuple()
        .unwrap();

    let aloop = find_loopsize(a);

    transform(aloop, b)
}

pub fn part2(input: &str) -> usize {
    input.lines().map(|line| line);
    unimplemented!()
}

#[test]
fn test() {
    let a = 5764801;
    let b = 17807724;
    assert_eq!(find_loopsize(a), 8);
    assert_eq!(transform(8, PUBK), a);
    assert_eq!(transform(8, b), 14897079);
    assert_eq!(find_loopsize(b), 11);
    assert_eq!(transform(11, PUBK), b);
    assert_eq!(transform(11, a), 14897079);

    let input = read_input("input25.txt").unwrap();
    assert_eq!(part1(&input), 6408263);
    assert_eq!(part2(&input), 0);
}
