use crate::utils::*;

fn nums(input: &str) -> Vec<i16> {
    let mut nums: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    // nums.push(0);
    // nums.push(nums.iter().max().unwrap() + 3);
    nums.sort_unstable();
    nums
}

pub fn part1(input: &str) -> usize {
    let nums = nums(input);

    let it = nums.array_windows().map(|[a, b]| b - a);

    let ones = it.filter(|&d| d == 1).count() + 1;
    // let threes = it.clone().filter(|&d| d == 3).count() + 1;
    let threes = nums.len() - ones + 1;

    ones * threes
}

pub fn part10(input: &str) -> usize {
    type S = BitSet;
    // type S = FSet<i32>;
    let nums: S = input.lines().map(|line| line.parse().unwrap()).collect();

    let ones = nums.iter().filter(|&d| nums.contains(d - 1)).count() + 1;
    let threes = nums.len() - ones + 1;

    ones * threes
}

pub fn part20(input: &str) -> i64 {
    let mut nums: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    nums.push(0);
    nums.sort_unstable();
    
    let grps = nums.array_windows().group_by(|[b, a]| a - b);

    fn trib(c: usize) -> i64 {
        // if you wanna be pedantic, you can actually calculate this for however many long you want
        // precomputation is still O(n), although it would be slower so
        match c {
            1 => 1,
            2 => 2,
            3 => 4,
            4 => 7,
            _ => unreachable!()
        }
    }

    let prod = grps
        .into_iter()
        .filter_map(|(d, grp)| (d == 1).as_some(grp.count()).map(trib))
        .product();
    
        prod
}
pub fn part2(input: &str) -> i64 {
    let nums = nums(input);

    let mut table = vec![0; nums.len()];
    let table = &mut table[..];
    for i in 0..3 {
        table[i] = 1;
        for j in 0..i {
            if (nums[i] - nums[j]) <= 3 {
                table[i] += table[j];
            }
        }
    }
    for i in 3..table.len() {
        for j in (i - 3)..i {
            if (nums[i] - nums[j]) <= 3 {
                table[i] += table[j];
            }
        }
    }

    *table.last().unwrap()
}

#[test]
fn test() {
    let input = read_input("input10.txt").unwrap();
    assert_eq!(part1(&input), 3000);
    assert_eq!(part2(&input), 193434623148032);
}
