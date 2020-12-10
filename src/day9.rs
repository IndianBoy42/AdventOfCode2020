use crate::utils::*;
use core::ops::Add;
use std::ops::{AddAssign, Sub, SubAssign};

fn parse<T: FromStr>(input: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(T::from_str)
        .map(Result::unwrap)
        .collect_vec()
}

fn weakness<T, const W: usize>(nums: &[T]) -> T
where
    T: Copy + Add<T> + Send + Sync,
    <T as Add>::Output: PartialEq<T>,
{
    // nums.par_windows(26)
    // .find_map_any(|window| {
    nums.array_windows::<26>()
        // .find_map(|&[ref rest @ .., last]| {
        .find_map(|window| {
            let (&last, rest) = window.split_last().unwrap();
            let iter = rest.iter().copied();
            let comb = iter.tuple_combinations();
            // let comb = iproduct!(iter.clone(), iter);
            // let comb = iter
            // .clone()
            // .enumerate()
            // .flat_map(|(i, x)| iter.clone().skip(i + 1).map(move |y| (x, y)));
            comb.map(|(x, y)| x + y).all(|x| x != last).as_some(last)
        })
        .unwrap()
}

pub fn part1(input: &str) -> usize {
    let nums = parse::<usize>(input);

    weakness::<_, 25>(&nums)
}

fn find_range2<T: Copy + Ord + Eq + Sub<Output = T>>(acc: &[T], tar: T) -> (usize, usize) {
    let iter = acc.iter().copied().enumerate();
    // let mut comb = iter.tuple_combinations();
    // let mut comb = iproduct!(iter.clone(), iter).filter(|((i, x), (j, y))| (i < j));
    let mut comb = iter
        .clone()
        .flat_map(|(i, x)| iter.clone().skip(i + 1).map(move |(j, y)| ((i, x), (j, y))));

    // (comb.find_map(|((i, x), (j, y))| ((y.max(x) - y.min(x)) == tar).as_some((i + 1, j)))).unwrap()
    (comb.find_map(|((i, x), (j, y))| ((y - x) == tar).as_some((i + 1, j)))).unwrap()
}

#[allow(clippy::comparison_chain)]
fn find_range<T: Copy + Ord + Eq + Sub<Output = T>>(acc: &[T], tar: T) -> (usize, usize) {
    let mut it = acc.iter().copied().enumerate().peekable();
    let mut jt = it.clone();

    loop {
        let &(i, x) = it.peek().unwrap();
        let &(j, y) = jt.peek().unwrap();

        let sum = y - x;
        if sum < tar {
            jt.next();
        } else if sum > tar {
            it.next();
        } else {
            return (i + 1, j);
        }
    }
}

fn find_range_direct<T>(nums: &[T], tar: T) -> (usize, usize)
where
    T: Copy + Ord + Eq + Default + AddAssign + SubAssign,
{
    let mut it = nums.iter().copied().enumerate().peekable();
    let mut jt = it.clone();

    let mut sum = T::default();
    while sum != tar {
        if sum < tar {
            let (j, y) = jt.next().unwrap();
            sum += y;
        } else {
            let (i, x) = it.next().unwrap();
            sum -= x;
        }
    }

    let &(i, x) = it.peek().unwrap();
    let &(j, y) = jt.peek().unwrap();

    (i, j)
}

fn partial_sums<T: Copy + AddAssign<T> + Default>(nums: &[T]) -> Vec<T> {
    nums.iter()
        .copied()
        .scan(Default::default(), |acc, v| {
            *acc += v;
            Some(*acc)
        })
        .collect_vec()
}

pub fn part2(input: &str) -> usize {
    let nums = parse::<usize>(input);

    let weakness = weakness::<_, 25>(&nums);

    // let acc = nums
    //     .iter()
    //     .copied()
    //     .scan(0, |acc, v| {
    //         *acc += v;
    //         Some(*acc)
    //     })
    //     .collect_vec();
    // let (i, j) = find_range(&acc, weakness);
    let (i, j) = find_range_direct(&nums, weakness);

    let (min, max) = minmax(nums[i..j].iter().copied()).unwrap();
    min + max
}

#[test]
fn test() {
    let input = read_input("input9.txt").unwrap();
    assert_eq!(part1(&input), 22_477_624);
    assert_eq!(part2(&input), 2_980_044);
}
