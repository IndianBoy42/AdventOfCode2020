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
enum OpCode {
    NOP,
    JMP,
    ACC,
}

pub fn part1(input: &str) -> i32 {
    let program = parse(input);

    solve(&program).1
}

pub fn part2(input: &str) -> i32 {
    let mut program = parse(input);

    if false {
        program
            .clone()
            .par_iter()
            .enumerate()
            .filter(|(_,&(ins, _))| ins != INS_ACC)
            .find_map_any(|(i, &(ins, _))| {
                let mut program = program.clone();

                // let old = ins;
                program[i].0 = match ins {
                    INS_NOP => INS_JMP,
                    INS_JMP => INS_NOP,
                    _ => unreachable!(),
                };

                let (pc, acc) = solve(&program);
                // program[i].0 = old;

                if pc as usize == program.len() {
                    Some(acc)
                } else {
                    None
                }
            }).unwrap()
    } else {
        for (i, (ins, _)) in program.clone().iter_mut().enumerate() {
            if *ins == INS_ACC {
                continue;
            }

            let old = *ins;
            program[i].0 = match old {
                INS_NOP => INS_JMP,
                INS_JMP => INS_NOP,
                _ => unreachable!(),
            };

            let (pc, acc) = solve(&program);
            if pc as usize == program.len() {
                return acc;
            }

            program[i].0 = old;
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
