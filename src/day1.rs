use std::mem::size_of;

use bit_set::BitSet;

use crate::utils::*;

// pub fn find_sums_to<I, It: IntoIterator<Item = I> + Clone>(input: It, tar: I) -> Option<(I, I)>
// where
//     It::IntoIter: Clone,
//     I:  Copy + std::ops::Add<Output = I> + PartialEq,
// {
//     iproduct!(input.clone().into_iter(), input.into_iter()).find(|&(x, y)| (x + y) == tar)
// }

pub fn find_sums_to<I, It: IntoIterator<Item = I> + Clone>(input: &It, tar: I) -> Option<(I, I)>
where
    It::IntoIter: Clone,
    I: Copy + std::hash::Hash + std::cmp::Eq + std::ops::Sub<Output = I>,
{
    let mut set = fset(0);
    for num in input.clone() {
        if set.contains(&(tar - num)) {
            return Some((num, tar - num));
        } else {
            set.insert(num);
        }
    }
    None
}

pub fn part1(input: &str) -> u32 {
    let nums = input
        .lines()
        .map(u32::from_str)
        .map(Result::unwrap)
        .collect_vec();

    iproduct!(&nums, &nums)
        .find(|(&x, &y)| (x + y) == 2020)
        .map(|(&x, &y)| x * y)
        .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let nums = input
        .lines()
        .map(u32::from_str)
        .map(Result::unwrap)
        .collect_vec();

    iproduct!(&nums, &nums, &nums)
        .find(|(&x, &y, &z)| (x + y + z) == 2020)
        .map(|(&x, &y, &z)| x * y * z)
        .unwrap()
}


pub fn part2_2(input: &str) -> i32 {
    let nums = input
        .lines()
        .map(i32::from_str)
        .map(Result::unwrap)
        .collect_vec();

    nums.iter()
        .find_map(|&x| {
            let mut set = fset(nums.len());
            for &num in &nums {
                if set.contains(&(2020 - x - num)) {
                    return Some((x, num, 2020 - x - num));
                } else {
                    set.insert(num);
                }
            }
            None
        })
        .map(|(x, y, z)| x * y * z)
        .unwrap()
}

pub fn part2_3(input: &str) -> i32 {
    let nums = input
        .lines()
        .map(i32::from_str)
        .map(Result::unwrap)
        .collect_vec();

    nums.par_iter()
        .filter_map(|&x| {
            let mut set = fset(nums.len());
            for &num in &nums {
                if set.contains(&(2020 - x - num)) {
                    return Some((x, num, 2020 - x - num));
                } else {
                    set.insert(num);
                }
            }
            None
        })
        .map(|(x, y, z)| x * y * z)
        .collect::<Vec<_>>()
        .first()
        .copied()
        .unwrap()
}

fn round_pow2(n: usize) -> usize {
    1 << (size_of::<usize>() * 8 - n.leading_zeros() as usize)
}
pub fn part2fft(input: &str) -> usize {
    let nums: BitSet = input
        .lines()
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .filter(|&x| x < 2020)
        .collect();
    let N = round_pow2(nums.capacity());

    unimplemented!()
}

#[test]
fn test1() {
    let input = read_input("input1.txt").unwrap();
    assert_eq!(part1(&input), 646779);
    assert_eq!(part2(&input), 246191688);
    assert_eq!(part2_2(&input), 246191688);
    assert_eq!(part2_3(&input), 246191688);
    // assert_eq!(part2fft(&input), 246191688);
}
