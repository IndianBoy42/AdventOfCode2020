use core::sync::atomic::{self, AtomicBool, AtomicI32};
use crossbeam::channel;
use std::time::Instant;

use crate::utils::*;

type Ins<'a> = &'a str;
const INS_ACC: Ins = "acc";
const INS_JMP: Ins = "jmp";
const INS_NOP: Ins = "nop";
fn parse(input: &str) -> Vec<(Ins, i32)> {
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
// fn parse(input: &str) -> Vec<(Ins, i32)> {
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

fn solve(program: &[(Ins, i32)]) -> (i32, i32) {
    let mut acc = 0;
    let mut visited = BitSet::with_capacity(program.len());
    visited.insert(program.len());
    let mut pc = 0;
    loop {
        if !visited.insert(pc as usize) {
            break;
        }

        let (ins, arg): (_, i32) = program[pc as usize];

        match ins {
            INS_ACC => acc += arg,
            INS_JMP => pc += arg - 1,
            _ => {}
        }
        pc += 1;
    }

    (pc, acc)
}

pub fn part2(input: &str) -> i32 {
    // part2brute(input)
    // part2mt(input)
    part2smart(input)
}

fn part2smart(input: &str) -> i32 {
    // Find winning positions
    fn find_winning(program: &[(Ins, i32)]) -> (BitSet, BitSet) {
        fn find_winning_from(
            mut pc: i32,
            program: &[(Ins, i32)],
            winning: &mut BitSet,
            death: &mut BitSet,
        ) {
            let mut visited = BitSet::with_capacity(program.len());
            visited.insert(program.len());
            loop {
                if death.contains(pc as usize) {
                    break;
                }
                if !visited.insert(pc as usize) {
                    break;
                }
                if winning.contains(pc as usize) {
                    pc = program.len() as _;
                    break;
                }

                let (ins, arg): (_, i32) = program[pc as usize];

                match ins {
                    INS_JMP => pc += arg - 1,
                    _ => {}
                }
                pc += 1;
            }

            if pc == program.len() as i32 {
                winning.union_with(&visited);
            } else {
                death.union_with(&visited);
            }
        }

        let mut winning = BitSet::with_capacity(program.len());
        let mut death = BitSet::with_capacity(program.len());
        for i in 0..program.len() {
            if winning.contains(i) || death.contains(i) {
                continue;
            }
            find_winning_from(i as _, &program, &mut winning, &mut death);
        }
        death.remove(program.len());

        (winning, death)
    }

    // Solve from a starting state, assume no infinite loops for speeeed
    fn solve_from(mut pc: i32, mut acc: i32, program: &[(Ins, i32)]) -> i32 {
        loop {
            if pc >= program.len() as i32 {
                break;
            }

            let (ins, arg): (_, i32) = program[pc as usize];

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

    let (winning, death) = find_winning(&program);
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
                    return solve_from(pc as i32 + 1, acc, &program);
                }
                // Just do normal shit
                pc = (pc as i32 + arg - 1) as _;
            }
            INS_NOP => {
                // If it were jmp would be win?
                if winning.contains((pc as i32 + arg) as _) {
                    return solve_from(pc as i32 + arg, acc, &program);
                }
            }
            _ => {}
        }
        pc += 1;
    }

    unreachable!()
}

pub fn part1(input: &str) -> i32 {
    let program = parse(input);

    solve(&program).1
}

fn part2mt(input: &str) -> i32 {
    let mut program = parse(input);

    // let start = Instant::now();
    let finished = AtomicBool::default();
    let result = AtomicI32::default();

    let res = crossbeam::scope(|scope| {
        const NTHREADS: usize = 8;

        let program = &program;
        let threads = (0..NTHREADS)
            .map(|thread_idx| {
                let finished = &finished;
                let result = &result;

                scope.spawn(move |_| {
                    let mut exe = program.clone();
                    let iter = program
                        .iter()
                        .enumerate()
                        .skip(thread_idx)
                        .step_by(NTHREADS);
                    for (i, &(ins, _)) in iter {
                        // Check for cancellation ocassionally
                        if i % 16 == 0 && finished.load(atomic::Ordering::Acquire) {
                            return;
                        }
                        // Process as normal
                        if ins == INS_ACC {
                            continue;
                        }

                        exe[i].0 = match ins {
                            INS_NOP => INS_JMP,
                            INS_JMP => INS_NOP,
                            _ => unreachable!(),
                        };

                        let (pc, acc) = solve(&exe);
                        if pc as usize == exe.len() {
                            result.store(acc, atomic::Ordering::Release);
                            finished.store(true, atomic::Ordering::Release);

                            return;
                        }

                        exe[i].0 = ins;
                    }
                    //not found, hopefully another thread finds
                })
            })
            .collect_vec();
    });

    // let dur = Instant::now() - start;
    // println!("{:?}", dur);

    result.load(atomic::Ordering::Acquire)
}

fn part2brute(input: &str) -> i32 {
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
