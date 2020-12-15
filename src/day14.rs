use crate::utils::*;

enum Instruction {
    Mask(),
}

fn u64_from_bits(it: impl Iterator<Item = bool>) -> u64 {
    it.fold(0, |acc, v| acc << 1 | (v as u64))
}

pub fn part1(input: &str) -> u64 {
    let instrs = input
        .lines()
        .map(|line| line.split_once(" = ").unwrap())
        .group_by(|&(cmd, _)| cmd.starts_with("mask"));

    let mem = instrs
        .into_iter()
        .tuples()
        .map(|((ismask, maskcmd), (isnotmem, memcmd))| {
            debug_assert!(ismask);
            debug_assert!(!isnotmem);
            (maskcmd, memcmd)
        })
        .fold(fmap(1000), |mut map: FMap<u64, u64>, group| {
            let (maskcmd, memcmds) = group;
            let ((maskcmd, mask),) = maskcmd.collect_tuple().unwrap();
            debug_assert_eq!(maskcmd, "mask");
            let set = u64_from_bits(mask.bytes().map(|b| b == b'1'));
            let clr = !u64_from_bits(mask.bytes().map(|b| b == b'0'));

            memcmds.for_each(|(cmd, val)| {
                let addr: u64 = cmd
                    .strip_prefix("mem[")
                    .and_then(|s| s.strip_suffix("]"))
                    .and_then(|s| s.parse().ok())
                    .unwrap();
                let val: u64 = val.parse().unwrap();
                let val = (val | set) & clr;
                map.insert(addr, val);
            });

            map
        });

    mem.values().sum()
}

pub fn part2(input: &str) -> u64 {
    let instrs = input
        .lines()
        .map(|line| line.split_once(" = ").unwrap())
        .group_by(|&(cmd, _)| cmd.starts_with("mask"));

    let mem = instrs
        .into_iter()
        .tuples()
        .map(|((ismask, maskcmd), (isnotmem, memcmd))| {
            debug_assert!(ismask);
            debug_assert!(!isnotmem);
            (maskcmd, memcmd)
        })
        .fold(fmap(1000), |mut map: FMap<u64, u64>, group| {
            let (maskcmd, memcmds) = group;
            let ((maskcmd, mask),) = maskcmd.collect_tuple().unwrap();
            debug_assert_eq!(maskcmd, "mask");
            let set = u64_from_bits(mask.bytes().map(|b| b == b'1'));
            let flt = mask
                .bytes()
                .enumerate()
                .filter_map(|(i, b)| (b == b'X').as_some(35-i))
                .collect::<ArrayVec<[usize; 32]>>();

            for (cmd, val) in memcmds {
                let addr: u64 = cmd
                    .strip_prefix("mem[")
                    .and_then(|s| s.strip_suffix("]"))
                    .and_then(|s| s.parse().ok())
                    .unwrap();
                let val: u64 = val.parse().unwrap();

                let max = 1 << flt.len();
                for index in 0u64..max {
                    // println!("{}", index);
                    let addr = flt.iter().enumerate().fold(addr|set, |addr, (i, &p)| {
                        // println!("\t{} {} {:b}", p, i, addr);
                        let p = 1 << p;
                        let addr = addr & (!p);
                        let idxbit = (index >> i) & 1;
                        let insbit = (!idxbit).wrapping_add(1) & p;
                        // println!("\t>{:b} {} {}", addr, idxbit, insbit);
                        addr | insbit
                    });

                    // println!("{}", addr);
                    map.insert(addr, val);
                }
            };

            map
        });

    mem.values().sum()
}

#[test]
fn test() {
    let input = read_input("input14.txt").unwrap();
    // assert_eq!(part1(&input), 10050490168421);
    assert_eq!(part2(&input), 2173858456958);
}
