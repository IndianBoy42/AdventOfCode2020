use crate::utils::*;

pub fn part1(input: &str) -> usize {
    const REQUIRED: &[&str; 7] = &["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

    input
        .split("\n\n")
        .par_bridge()
        .filter(|passport| {
            let fields = passport
                .split_ascii_whitespace()
                // .flat_map(|field| field.split(':'))
                // .step_by(2);
                .map(|field| field.split_once(':').unwrap())
                .map(|(l, _)| l);

            // fields.filter(|x| REQUIRED.contains(x)).count() == 7
            let fields: ArrayVec<[&str; 8]> = fields.collect();
            // (fields.len() > 7) || (fields.len() == 7 && REQUIRED.iter().all(|req| fields.contains(&req)))
            (fields.len() > 7)
                || (fields.len() == 7 && fields.iter().all(|fld| REQUIRED.contains(fld)))
        })
        .count()
}

struct IdxMap<'a> {
    arr: [&'a str; 8],
}
impl<'a> FromIterator<(&'a str, &'a str)> for IdxMap<'a> {
    fn from_iter<I: IntoIterator<Item = (&'a str, &'a str)>>(iter: I) -> Self {
        let mut arr = [""; 8];
        for (k, v) in iter {
            let k = match k {
                "byr" => 0,
                "iyr" => 1,
                "eyr" => 2,
                "hgt" => 3,
                "hcl" => 4,
                "ecl" => 5,
                "pid" => 6,
                "cid" => 7,
                _ => unreachable!(),
            };
            arr[k] = v;
        }

        IdxMap { arr }
    }
}
impl<'a> IdxMap<'a> {
    fn get(&self, k: &str) -> Option<&&str> {
        let got = unsafe {
            self.arr.get_unchecked(match k {
                "byr" => 0,
                "iyr" => 1,
                "eyr" => 2,
                "hgt" => 3,
                "hcl" => 4,
                "ecl" => 5,
                "pid" => 6,
                "cid" => 7,
                _ => unreachable!(),
            })
        };

        (!got.is_empty()).as_some(got)
    }
    fn geti(&self, k: usize) -> Option<&&str> {
        let got = unsafe { self.arr.get_unchecked(k) };

        (!got.is_empty()).as_some(got)
    }
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
// type Fields<'a> = SmolMap<'a>;
type Fields<'a> = IdxMap<'a>;

fn validate2(fields: &Fields) -> bool {
    let byr = || {
        fields.geti(0).map_or(false, |&val| {
            (1920..(2002 + 1)).contains(&val.parse().unwrap())
        })
    };
    let iyr = || {
        fields.geti(1).map_or(false, |&val| {
            (2010..(2020 + 1)).contains(&val.parse().unwrap())
        })
    };
    let eyr = || {
        fields.geti(2).map_or(false, |&val| {
            (2020..(2030 + 1)).contains(&val.parse().unwrap())
        })
    };
    let hgt = || {
        fields.geti(3).map_or(false, |&val| -> bool {
            let cm = || {
                val.strip_suffix("cm").map_or(false, |val| {
                    (150..(193 + 1)).contains(&val.parse().unwrap_or(0))
                })
            };
            let inch = || {
                val.strip_suffix("in").map_or(false, |val| {
                    (59..(76 + 1)).contains(&val.parse().unwrap_or(0))
                })
            };

            cm() || inch()
        })
    };
    let hcl = || {
        fields.geti(4).map_or(false, |&val| {
            val.strip_prefix('#')
                .map_or(false, |val| val.bytes().all(|x| x.is_ascii_hexdigit()))
        })
    };
    let ecl = || {
        fields.geti(5).map_or(false, |&val| {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val)
        })
    };
    let pid = || {
        fields.geti(6).map_or(false, |&val| {
            val.len() == 9 && val.bytes().all(|b| (b'0'..=b'9').contains(&b))
        })
    };
    byr() && iyr() && eyr() && hgt() && hcl() && ecl() && pid()
}

pub fn part2(input: &str) -> usize {
    input
        .split("\n\n").par_bridge()
        .filter(|passport| {
            let fields = passport
                .split_ascii_whitespace()
                .map(|field| field.split_once(':').unwrap())
                // .map(|field| field.split(':').collect_tuple().unwrap())
                .collect();
            validate2(&fields)
        })
        .count()
}

#[test]
fn test() {
    let input = read_input("input4.txt").unwrap();
    assert_eq!(part1(&input), 260);
    assert_eq!(part2(&input), 153);
}
