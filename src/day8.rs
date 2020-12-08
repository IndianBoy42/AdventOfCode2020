use crate::utils::*;

fn parse(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(ins, arg)| (ins, arg.parse().unwrap()))
        .collect_vec()
}

fn solve(program: &[(&str, i32)]) -> (i32, i32) {
    let mut acc = 0;
    let mut visited = BitSet::with_capacity(program.len());
    visited.insert(program.len());
    let mut pc = 0i32;
    loop {
        if !visited.insert(pc as usize) {
            break;
        }

        let (ins, arg): (_, i32) = (program[pc as usize]);

        match ins {
            "acc" => acc += arg,
            "jmp" => pc += arg - 1,
            "nop" => {}
            _ => {}
        }
        pc += 1;
    }

    (pc, acc)
}
enum OpCode {
    NOP, JMP, ACC
}

pub fn part1(input: &str) -> i32 {
    let program: Vec<(&str, i32)> = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(ins, arg)| (ins, arg.parse().unwrap()))
        .collect_vec();

    solve(&program).1
}

pub fn part2(input: &str) -> i32 {
    let mut program = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(ins, arg)| (ins, arg.parse().unwrap()))
        .collect_vec();

    for (i, (ins, _)) in program.clone().iter_mut().enumerate() {
        if *ins == "acc" {
            continue;
        }

        let old = *ins;
        program[i].0 = match old {
            "nop" => "jmp",
            "jmp" => "nop",
            _ => unreachable!(),
        };

        let (pc, acc) = solve(&program);
        if pc as usize == program.len() {
            return acc;
        }

        program[i].0 = old;
    }

    unimplemented!()
}

#[test]
fn test() {
    let input = read_input("input8.txt").unwrap();
    assert_eq!(part1(&input), 1766);
    assert_eq!(part2(&input), 1639);
}
