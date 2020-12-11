use core::sync::atomic::{self, AtomicBool, AtomicI16};

use crate::utils::*;

type Ins<'a> = &'a str;
const INS_ACC: Ins = "acc";
const INS_JMP: Ins = "jmp";
const INS_NOP: Ins = "nop";
fn parse(input: &str) -> Vec<(Ins, i16)> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(ins, arg)| (ins, arg.parse().unwrap()))
        .collect()
}

// This is slower for some reason
// #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
// enum Ins {
//     ACC,
//     JMP,
//     NOP,
// }
// const INS_ACC: Ins = Ins::ACC;
// const INS_JMP: Ins = Ins::JMP;
// const INS_NOP: Ins = Ins::NOP;
// fn parse(input: &str) -> Vec<(Ins, i16)> {
//     input
//         .lines()
//         .map(|line| line.split_once(' ').unwrap())
//         .map(|(ins, arg)| {
//             (
//                 match ins {
//                     "acc" => INS_ACC,
//                     "jmp" => INS_JMP,
//                     "nop" => INS_NOP,
//                     _ => unreachable!(),
//                 },
//                 arg.parse().unwrap(),
//             )
//         })
//         .collect()
// }

fn solve(program: &[(Ins, i16)]) -> (i16, i16) {
    let mut acc = 0;
    let mut visited = BitSet::with_capacity(program.len());
    visited.insert(program.len());
    let mut pc = 0;
    while visited.insert(pc as usize) {
        let (ins, arg): (_, i16) = program[pc as usize];

        match ins {
            INS_ACC => acc += arg,
            INS_JMP => pc += arg - 1,
            _ => {}
        }
        pc += 1;
    }

    (pc, acc)
}

pub fn part2(input: &str) -> i16 {
    // part2brute(input)
    // part2mt(input)
    part2smart(input)
    // part2onepass(input)
}

fn part2smart(input: &str) -> i16 {
    // Find winning positions
    fn find_winning(program: &[(Ins, i16)]) -> (BitSet, BitSet) {
        fn find_winning_from(
            mut pc: i16,
            program: &[(Ins, i16)],
            winning: &mut BitSet,
            allvisited: &mut BitSet,
        ) {
            let mut visited = allvisited.clone();
            while visited.insert(pc as usize) {
                let (ins, arg): (_, i16) = program[pc as usize];

                match ins {
                    INS_JMP => pc += arg,
                    _ => pc += 1,
                }
            }

            // let newallvisited = visited.clone();
            if winning.contains(pc as usize) {
                visited.difference_with(&allvisited);
                winning.union_with(&visited);
                allvisited.union_with(&visited);
            } else {
                *allvisited = visited;
            }
        }

        let mut winning = BitSet::with_capacity(1024);
        winning.insert(program.len());
        let mut allvisited = BitSet::with_capacity(1024);
        allvisited.insert(program.len());
        for i in 0..program.len() {
            if allvisited.contains(i) {
                continue;
            }
            find_winning_from(i as _, &program, &mut winning, &mut allvisited);
        }
        // let all = (0..program.len()).collect::<BitSet>();
        // while let Some(i) = all.difference(&allvisited).next() {
        // find_winning_from(i as _, &program, &mut winning, &mut allvisited);
        // }

        (winning, allvisited)
    }

    // Solve from a starting state, assume no infinite loops for speeeed
    fn solve_from(mut pc: i16, mut acc: i16, program: &[(Ins, i16)]) -> i16 {
        while pc < program.len() as i16 {
            let (ins, arg): (_, i16) = program[pc as usize];

            match ins {
                INS_ACC => acc += arg,
                INS_JMP => pc += arg - 1,
                _ => {}
            }
            pc += 1;
        }

        acc
    }

    let program = parse(input);

    let (winning, _allvisited) = find_winning(&program);
    // dbg!(winning.union(&death).count());

    let mut acc = 0;
    let mut pc = 0;
    loop {
        let (ins, arg) = program[pc as usize];
        match ins {
            INS_ACC => acc += arg,
            INS_JMP => {
                // If it were nop would be win?
                if winning.contains(pc + 1) {
                    return solve_from(pc as i16 + 1, acc, &program);
                }
                // Just do normal shit
                pc = (pc as i16 + arg - 1) as _;
            }
            INS_NOP => {
                // If it were jmp would be win?
                if winning.contains((pc as i16 + arg) as _) {
                    return solve_from(pc as i16 + arg, acc, &program);
                }
            }
            _ => {}
        }
        pc += 1;
    }
}

pub fn part1(input: &str) -> i16 {
    let program = parse(input);

    solve(&program).1
}

fn part2onepass(input: &str) -> i16 {
    fn find_swap(program: &[(Ins, i16)]) -> i16 {
        // First run, trace instruction flow till infinite loop
        let mut visited = BitSet::with_capacity(program.len());
        let mut pc = 0;
        while visited.insert(pc as usize) {
            let (ins, arg): (_, i16) = program[pc as usize];

            match ins {
                INS_JMP => pc += arg,
                _ => pc += 1,
            }
        }

        let (firstnegjmp, _) = program
            .iter()
            .enumerate()
            .rev()
            .find(|(_i, &(ins, arg))| ins == INS_JMP && arg < 0)
            .unwrap();
        if visited.contains(firstnegjmp) {
            return firstnegjmp as _;
        }
        let mut landingarea = (firstnegjmp..program.len() + 1).collect::<BitSet>();

        loop {
            let iteration = program[..firstnegjmp]
                .iter()
                .enumerate()
                .rev()
                .find_map(|(i, &(ins, arg))| {
                    let visitedi = visited.contains(i);
                    let landsi = landingarea.contains((i as i16 + arg) as usize);

                    // dbg!(i, ins, arg, visitedi, landsi);

                    if ins == INS_NOP && visitedi && landsi {
                        // println!("Found unhit NOP that could land us in the good area, {:?} {:?} {:?} {:?} {:?}", i, ins, arg, visitedi, landsi);
                        return Some(Some(i));
                    } else if ins == INS_JMP && !visitedi && landsi && !landingarea.contains(i) {
                        // println!("Found unhit JMP that could land us in the good area, {:?} {:?} {:?} {:?} {:?}", i, ins, arg, visitedi, landsi);
                        // Look for preceeding JMP
                        let (j, _) = program[..i]
                            .iter()
                            .enumerate()
                            .rev()
                            .find(|(_i, &(ins, _arg))| ins == INS_JMP)
                            .unwrap();
                        if visited.contains(j) {
                            // println!("Found preceeding jump that was hit, {:?} {:?} {:?}", j, ins, arg );
                            return Some(Some(j));
                        } else {
                            // println!("Found preceeding jump that was not hit, {:?} {:?} {:?}", j, ins, arg );
                            // Add new landing area and exit the loop, start search again from the top
                            landingarea.extend((j + 1)..(i + 1));
                            return Some(None);
                        }
                    } else {
                        None
                    }
                })
                .unwrap();

            if let Some(j) = iteration {
                // println!("Finished finding swap {}", j);
                return j as _;
            }

            // println!("Oh shit here we go again");
        }
    }

    let mut program = parse(input);

    let i = find_swap(&program);
    let (ref mut ins, _) = program[i as usize];
    *ins = match *ins {
        INS_NOP => INS_JMP,
        INS_JMP => INS_NOP,
        _ => unreachable!(),
    };

    solve(&program).1
}

fn part2brute(input: &str) -> i16 {
    let mut program = parse(input);

    for (i, &(ins, _)) in program.clone().iter().enumerate() {
        if ins == INS_ACC {
            continue;
        }

        program[i].0 = match ins {
            INS_NOP => INS_JMP,
            INS_JMP => INS_NOP,
            _ => unreachable!(),
        };

        let (pc, acc) = solve(&program);
        if pc as usize == program.len() {
            return acc;
        }

        program[i].0 = ins;
    }
    unreachable!()
}

#[test]
fn test() {
    let input = read_input("input8.txt").unwrap();
    assert_eq!(part2(&input), 1639);
    assert_eq!(part1(&input), 1766);
}
