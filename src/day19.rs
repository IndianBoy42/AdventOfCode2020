use std::borrow::Cow;

use crate::utils::*;
use regex::Regex;

#[derive(Debug, Clone)]
enum Rule<'a> {
    Compound(Vec<Vec<i32>>),
    Built(String),
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

fn expand_regex<'a, 'b>(i: i32, dict: &'a mut Dict<'b>) -> String {
    fn inner<'a, 'b>(i: i32, dict: &'a mut Dict<'b>, buf: &mut String, depth: i32) {
        if depth > 10 {
            return;
        }
        let rule = dict.get(&i).unwrap();
        match rule {
            Rule::Compound(rules) => {
                let rules = rules.clone();
                let start = buf.len();
                buf.push('(');
                {
                    rules[0].iter().for_each(|&subrule| {
                        inner(subrule, dict, buf, if subrule == i { depth + 1 } else { 0 })
                    });

                    rules[1..].iter().for_each(|rule: &Vec<i32>| {
                        buf.push('|');
                        rule.iter().for_each(|&subrule| {
                            inner(subrule, dict, buf, if subrule == i { depth + 1 } else { 0 })
                        })
                    });
                }
                buf.push(')');
                *dict.get_mut(&i).unwrap() = Rule::Built(buf[start..].to_owned());
            }
            Rule::Built(rule) => buf.push_str(rule),
            Rule::Basic(rule) => buf.push_str(rule),
        };
    }

    let mut buf = String::with_capacity(1000);
    buf.push('^');
    inner(i, dict, &mut buf, 0);
    buf.push('$');
    buf
}

pub fn part1(input: &str) -> usize {
    let (rules, lines) = input.split_once("\n\n").unwrap();
    let mut rules: Dict = rules
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(ruleno, specs)| {
            let ruleno = ruleno.parse().unwrap();
            (ruleno, Rule::new(ruleno, specs))
        })
        .collect();

    let rule0 = expand_regex(0, &mut rules);
    let rule0 = Regex::new(&rule0).unwrap();

    lines.lines().filter(|line| rule0.is_match(line)).count()
}

pub fn part2(input: &str) -> usize {
    let (rules, lines) = input.split_once("\n\n").unwrap();
    let mut rules: Dict = rules
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

    let rule0 = expand_regex(0, &mut rules);
    
    let rule0 = Regex::new(&rule0).unwrap();
    lines.lines().filter(|line| rule0.is_match(line)).count()
}

#[test]
fn test() {
    // let input = read_input("test.txt").unwrap();
    let input = read_input("input19.txt").unwrap();
    assert_eq!(part1(&input), 124);
    // (part1(&input));
    assert_eq!(part2(&input), 228);
}
