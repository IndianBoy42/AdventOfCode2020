use crate::utils::*;

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
