use crate::utils::*;

fn pows() -> impl Iterator<Item = u16> {
    successors(Some(1), |prev| Some(prev * 2))
}

fn parse(input: &str) -> impl Iterator<Item = u16> + '_ {
    input
    .lines()
    .map(|line| {
        let (l, r) = line.as_bytes().split_at(7);
        let row: ArrayVec<[u16; 7]> = l
            .iter()
            .map(|&c| match c {
                b'F' => 0,
                b'B' => 1,
                _ => unreachable!(),
            })
            .collect();
        let col: ArrayVec<[u16; 3]> = r
            .iter()
            .map(|&c| match c {
                b'L' => 0,
                b'R' => 1,
                _ => unreachable!(),
            })
            .collect();

        let row = row
            .into_iter()
            .rev()
            .zip(pows())
            .map(|(v, pow)| v * pow)
            .sum::<u16>();
        let col = col
            .into_iter()
            .rev()
            .zip(pows())
            .map(|(v, pow)| v * pow)
            .sum::<u16>();

        row * 8 + col
    })
}

pub fn part1(input: &str) -> u16 {
    parse(input)
        // .inspect(|x| println!("{:?}", x))
        .max()
        .unwrap()
}
pub fn part2(input: &str) -> u16 {
    let nums = parse(input)
        // .inspect(|x| println!("{:?}", x))
        .collect_vec();
    let (min, max) = (nums.iter().min().copied().unwrap(), nums.iter().max().copied().unwrap());
    let set: FSet<_> = FSet::from_iter(nums);

    *FSet::from_iter(min..=max).difference(&set).next().unwrap()
}

#[test]
fn test5() {
    let input = read_input("input5.txt").unwrap();
    assert_eq!(part1(&input), 864);
    assert_eq!(part2(&input), 739);
}
