use std::borrow::Cow;

use crate::utils::*;
use regex::Regex;

enum Rule<'a> {
    Compound(Vec<Vec<i32>>),
    Basic(&'a str),
}
impl Rule<'_> {
    fn new(i: i32, specs: &str) -> Rule {
        if let Some(rule) = specs.trim().strip_prefix('"') {
            let rule = rule.strip_suffix('"').unwrap();
            Rule::Basic(rule)
        } else {
            let specs = specs
                .split('|')
                .map(|rule| {
                    rule.trim()
                        .split(' ')
                        .map(|n| n.trim().parse::<i32>().unwrap())
                        .collect_vec()
                })
                .collect_vec();
            Rule::Compound(specs)
        }
    }
}

type Dict<'a> = FMap<i32, Rule<'a>>;

fn expand_regex<'a, 'b>(i: i32, dict: &'a Dict<'b>) -> Option<Cow<'b, str>> {
    fn inner<'a, 'b>(i: i32, dict: &'a Dict<'b>, depth: i32) -> Option<Cow<'b, str>> {
        if depth > 10 {
            None?
        }
        let rule = dict.get(&i)?;
        let rule: Cow<str> = match rule {
            Rule::Compound(rules) => {
                let rule = rules
                    .iter()
                    .map(|rule| {
                        rule.iter()
                            .filter_map(|&subrule| {
                                inner(subrule, dict, if subrule == i { depth + 1 } else { 0 })
                            })
                            .join("")
                    })
                    .join("|");
                Cow::Owned(format!("({})", rule))
            }
            Rule::Basic(rule) => Cow::Borrowed(rule),
        };
        Some(rule)
    }
    inner(i, dict, 0)
}

pub fn part1(input: &str) -> usize {
    let (rules, lines) = input.split_once("\n\n").unwrap();
    let rules: Dict = rules
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(ruleno, specs)| {
            let ruleno = ruleno.parse().unwrap();
            (ruleno, Rule::new(ruleno, specs))
        })
        .collect();

    let rule0 = dbg!(expand_regex(0, &rules)).unwrap();
    let rule0 = format!("^{}$", rule0);
    let rule0 = Regex::new(&rule0).unwrap();

    lines.lines().filter(|line| rule0.is_match(line)).count()
}

pub fn part2(input: &str) -> usize {
    let (rules, lines) = input.split_once("\n\n").unwrap();
    let rules: Dict = rules
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(ruleno, specs)| {
            let ruleno = ruleno.parse().unwrap();
            let specs = if ruleno == 8 {
                " 42 | 42 8"
            } else if ruleno == 11 {
                " 42 31 | 42 11 31"
            } else {
                specs
            };
            (ruleno, Rule::new(ruleno, specs))
        })
        .collect();

    let rule0 = dbg!(expand_regex(0, &rules)).unwrap();
    let rule0 = format!("^{}$", rule0);
    let rule0 = Regex::new(&rule0).unwrap();

    lines.lines().filter(|line| rule0.is_match(line)).count()
}

#[test]
fn test() {
    // let input = read_input("test.txt").unwrap();
    let input = read_input("input19.txt").unwrap();
    assert_eq!(part1(&input), 124);
    // dbg!(part1(&input));
    assert_eq!(part2(&input), 0);
}
