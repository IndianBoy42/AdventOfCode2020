use crate::utils::*;

pub fn part1(input: &str) -> usize {
    const FIELDS: &[&str; 7] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    input
        .split("\n\n")
        .filter(|passport| {
            let fields = passport
                .split_ascii_whitespace()
                .map(|field| field.split_once(':').unwrap())
                // .map(|field| field.split(':').collect_tuple().unwrap())
                .map(|(l, _)| l)
                .collect_vec();
            FIELDS.iter().copied().all(|req| fields.contains(&req))
        })
        .count()
}

fn validate2(fields: &FMap<&str, &str>) -> bool {
    let ok = [
        fields
            .get("byr")
            .map_or(false, |&val| (1920..=2002).contains(&val.parse().unwrap())),
        fields
            .get("iyr")
            .map_or(false, |&val| (2010..=2020).contains(&val.parse().unwrap())),
        fields
            .get("eyr")
            .map_or(false, |&val| (2020..=2030).contains(&val.parse().unwrap())),
        fields.get("hgt").map_or(false, |&val| {
            (val.ends_with("cm")
                && (150..=193).contains(&val[..val.len() - 2].parse().unwrap_or(0)))
                || (val.ends_with("in")
                    && (59..=76).contains(&val[..val.len() - 2].parse().unwrap_or(0)))
        }),
        fields.get("hcl").map_or(false, |&val| {
            val.starts_with('#') && val[1..7].bytes().all(|x| x.is_ascii_hexdigit())
        }),
        fields.get("ecl").map_or(false, |&val| {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val)
        }),
        fields.get("pid").map_or(false, |&val| {
            val.len() == 9 && val[..9].chars().all(char::is_numeric)
        }),
    ];

    ok.iter().copied().all(|x| x)
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
