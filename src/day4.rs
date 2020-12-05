use std::iter::FromIterator;

use crate::utils::*;
use arrayvec::ArrayVec;

pub fn part1(input: &str) -> usize {
    const REQUIRED: &[&str; 7] = &["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

    input
        .split("\n\n")
        .filter(|passport| {
            let fields = passport
                .split_ascii_whitespace()
                .map(|field| field.split_once(':').unwrap())
                // .map(|field| field.split(':').collect_tuple().unwrap())
                .map(|(l, _)| l);
            // fields.filter(|x| REQUIRED.contains(x)).count() == 7
            let fields: ArrayVec<[&str; 8]> = fields.collect();
            // (fields.len() > 7) || (fields.len() == 7 && REQUIRED.iter().all(|req| fields.contains(&req)))
            (fields.len() > 7) || (fields.len() == 7 && fields.iter().all(|fld| REQUIRED.contains(&fld)))
        })
        .count()
}

struct SmolMap<'a> {
    arr: ArrayVec<[(&'a str, &'a str); 8]>,
}
impl<'a> FromIterator<(&'a str, &'a str)> for SmolMap<'a> {
    fn from_iter<I: IntoIterator<Item = (&'a str, &'a str)>>(iter: I) -> Self {
        SmolMap {
            arr: iter.into_iter().collect(),
        }
    }
}
impl<'a> SmolMap<'a> {
    fn get(&self, i: &str) -> Option<&&str> {
        self.arr
            .iter()
            .find_map(|(k, v)| if *k == i { Some(v) } else { None })
    }
}

// type Fields<'a> = FMap<&'a str, &'a str>;
type Fields<'a> = SmolMap<'a>;

fn validate2(fields: &Fields) -> bool {
    let byr = || {
        fields
            .get("byr")
            .map_or(false, |&val| (1920..=2002).contains(&val.parse().unwrap()))
    };
    let iyr = || {
        fields
            .get("iyr")
            .map_or(false, |&val| (2010..=2020).contains(&val.parse().unwrap()))
    };
    let eyr = || {
        fields
            .get("eyr")
            .map_or(false, |&val| (2020..=2030).contains(&val.parse().unwrap()))
    };
    let hgt = || {
        fields.get("hgt").map_or(false, |&val| -> bool {
            let cm = || {
                val.strip_suffix("cm")
                    .map_or(false, |val| (150..=193).contains(&val.parse().unwrap_or(0)))
            };
            let inch = || {
                val.strip_suffix("in")
                    .map_or(false, |val| (59..=76).contains(&val.parse().unwrap_or(0)))
            };

            cm() || inch()
        })
    };
    let hcl = || {
        fields.get("hcl").map_or(false, |&val| {
            val.strip_prefix('#')
                .map_or(false, |val| val.bytes().all(|x| x.is_ascii_hexdigit()))
        })
    };
    let ecl = || {
        fields.get("ecl").map_or(false, |&val| {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val)
        })
    };
    let pid = || {
        fields.get("pid").map_or(false, |&val| {
            val.len() == 9 && val.bytes().all(|b| (b'0'..=b'9').contains(&b))
        })
    };
    byr() && iyr() && eyr() && hgt() && hcl() && ecl() && pid()
}

pub fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        // .map(|x| dbg!(x))
        .filter(|passport| {
            let fields = passport
                .split_ascii_whitespace()
                .map(|field| field.split_once(':').unwrap())
                // .map(|field| field.split(':').collect_tuple().unwrap())
                .collect();
            validate2(&fields)
        })
        // .map(|x| dbg!(x))
        .count()
}

#[test]
fn test4() {
    let input = read_input("input4.txt").unwrap();
    assert_eq!(part1(&input), 260);
    assert_eq!(part2(&input), 153);
}
