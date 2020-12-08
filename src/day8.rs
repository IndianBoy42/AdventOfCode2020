use core::sync::atomic::{self, AtomicBool};
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

pub fn part1(input: &str) -> i32 {
    let program = parse(input);

    solve(&program).1
}

use crossbeam::channel;

pub fn part2(input: &str) -> i32 {
    let mut program = parse(input);

    if false {
        // let start = Instant::now();
        let finished = AtomicBool::default();

        let res = crossbeam::scope(|scope| {
            const NTHREADS: usize = 4;

            let program = &program;
            let threads = (0..NTHREADS)
                .map(|thread_idx| {
                    // For sending the result back
                    let (s, r) = channel::bounded(1);
                    let s2 = s.clone();
                    let finished = &finished;

                    let th = scope.spawn(move |_| {
                        let s = s;
                        let mut exe = program.clone();
                        let iter = program
                            .iter()
                            .enumerate()
                            .skip(thread_idx)
                            .step_by(NTHREADS);
                        for (i, &(ins, _)) in iter {
                            // Check for cancellation
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
                                (s.send(acc)).unwrap();
                                return;
                            }

                            exe[i].0 = ins;
                        }
                        //not found, hopefully another thread finds
                    });
                    (th, r, s2)
                })
                .collect_vec();

            // Select over the result recv channels of all threads
            let mut sel = channel::Select::new();
            for (_, r, _) in &threads {
                sel.recv(r);
            }
            let sel = (sel.select());
            // This thread found it
            let selindex = sel.index();
            // Complete the recv operation
            let res = (sel.recv(&threads[selindex].1));
            // Stop all the other threads
            // threads
            //     .iter()
            //     .enumerate()
            //     .filter(|&(i, _)| i != selindex)
            //     .for_each(|(_, (_, _, cs, _, _))| {
            //         (cs.send(true));
            //     });
            finished.store(true, atomic::Ordering::Release);

            return res.unwrap();
        });

        // let dur = Instant::now() - start;
        // println!("{:?}", dur);

        res.unwrap()
    } else {
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
}

#[test]
fn test() {
    let input = read_input("input8.txt").unwrap();
    assert_eq!(part1(&input), 1766);
    assert_eq!(part2(&input), 1639);
}
