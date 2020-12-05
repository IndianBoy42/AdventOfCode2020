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

pub fn find_sums_to<It: IntoIterator<Item = usize> + Clone>(
    inputsize: usize,
    input: &It,
    tar: usize,
) -> Option<(usize, usize)>
where
    It::IntoIter: Clone,
{
    let mut set = BitSet::with_capacity(inputsize);
    for num in input.clone() {
        if set.contains(tar - num) {
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
        .map(FromStr::from_str)
        .map(Result::unwrap)
        // .sorted().take_while(|&x| x < 2020)
        .filter(|&x| x < 2020)
        .collect::<BitSet>();

    nums.iter()
        .find_map(|num| nums.contains(2020 - num).as_some((num, 2020 - num)))
        .map(|(x, y)| x * y)
        .unwrap()
        .try_into()
        .unwrap()
    // find_sums_to(nums.len(), &nums, 2020)
    // .map(|(x, y)| x * y)
    // .unwrap()
    // .try_into()
    // .unwrap()
}

pub fn part2(input: &str) -> u32 {
    let nums = input
        .lines()
        .map(usize::from_str)
        .map(Result::unwrap)
        // .sorted().take_while(|&x| x < 2020)
        .filter(|&x| x < 2020)
        .sorted()
        .collect::<BitSet>();

    nums.iter()
        .find_map(|x| {
            nums.iter().find_map(|num| {
                nums.contains(2020 - x - num)
                    .as_some((x, num, 2020 - x - num))
            })
        })
        .map(|(x, y, z)| x * y * z)
        .unwrap()
        .try_into()
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
    let _n = round_pow2(nums.capacity());

    unimplemented!()
}

#[test]
fn test() {
    let input = read_input("input1.txt").unwrap();
    assert_eq!(part1(&input), 646779);
    assert_eq!(part2(&input), 246191688);
    // assert_eq!(part2fft(&input), 246191688);
}
