use std::iter::Peekable;

use itertools::{unfold, PeekingNext};
use regex::CaptureMatches;
use regex::Regex;

use crate::utils::*;

fn num(input: &str) -> i64 {
    input.trim().parse().unwrap()
}

macro_rules! dbg {
    ($t:expr) => {
        $t
    };
}

fn evalone<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> i64 {
    match dbg!(tokens.next().unwrap()) {
        "+" => unreachable!(),
        "*" => unreachable!(),
        "(" => {
            // tokens.next();
            eval(tokens)
        }
        ")" => unreachable!(),
        token => {
            // tokens.next();
            num(token)
        }
    }
}

fn eval<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> i64 {
    let mut acc = 0;
    while let Some(token) = tokens.next() {
        match dbg!(token) {
            "+" => {
                acc += dbg!(evalone(tokens));
                dbg!(acc);
            }
            "*" => {
                acc *= dbg!(evalone(tokens));
                dbg!(acc);
            }
            "(" => {
                acc = eval(tokens);
                dbg!(acc);
            }
            ")" => break,
            token => {
                let num = num(token);
                acc = num;
                dbg!(acc);
            }
        }
    }

    dbg!(acc)
}

fn tokens(input: &str) -> impl Iterator<Item = &str> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r"([0-9]+|\*|\+|\(|\))").unwrap();
    }
    RE.captures_iter(input)
        .map(|cap| cap.get(0).unwrap().as_str())
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| dbg!(tokens(line).collect_vec()))
        // .take(1)
        .map(|line| eval(&mut line.into_iter()))
        .sum()
}

fn evalone2<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a str>>) -> i64 {
    match dbg!(tokens.next().unwrap()) {
        "+" => unreachable!(),
        "*" => unreachable!(),
        "(" => {
            // tokens.next();
            eval2(tokens)
        }
        ")" => unreachable!(),
        token => {
            // tokens.next();
            num(token)
        }
    }
}
fn eval_star<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a str>>) -> i64 {
    let mut acc = 0;
    // let mut tokens = tokens.peeking_take_while(|&token| token != "*");
    while let Some(&token) = tokens.peek() {
        match dbg!(token) {
            "+" => {
                tokens.next();
                acc += dbg!(evalone2(tokens));
                dbg!(acc);
            }
            "*" => break,
            "(" => {
                tokens.next();
                acc = eval2(tokens);
                dbg!(acc);
            }
            ")" => break,
            token => {
                tokens.next();
                let num = num(token);
                acc = num;
                dbg!(acc);
            }
        }
    }

    dbg!(acc)
}
fn eval2<'a>(tokens: &mut Peekable<impl Iterator<Item = &'a str>>) -> i64 {
    let mut acc = 0;
    while let Some(token) = tokens.next() {
        match dbg!(token) {
            "+" => {
                acc += dbg!(evalone2(tokens));
                dbg!(acc);
            }
            "*" => {
                acc *= dbg!(eval_star(tokens));
                dbg!(acc);
            }
            "(" => {
                acc = eval2(tokens);
                dbg!(acc);
            }
            ")" => break,
            token => {
                let num = num(token);
                acc = num;
                dbg!(acc);
            }
        }
    }

    dbg!(acc)
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| dbg!(tokens(line).collect_vec()))
        // .take(1)
        .map(|line| eval2(&mut line.into_iter().peekable()))
        .sum()
}

#[test]
fn test() {
    let input = read_input("input18.txt").unwrap();
    assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
    assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
    assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
    assert_eq!(
        part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        13632
    );
    assert_eq!(part1(&input), 11004703763391);

    assert_eq!(part2("1 + 2 * 3 + 4 * 5 + 6"), 231);
    assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
    assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    assert_eq!(
        part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        23340
    );
    assert_eq!(part2(&input), 0);
}
