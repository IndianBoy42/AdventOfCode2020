use std::collections::VecDeque;

use crate::utils::*;

fn bagmap(input: &str) -> FMap<&str, Vec<(i32, &str)>> {
    let iter = input
        .lines()
        .map(|line| line.split_once(" contain ").unwrap())
        .map(|(outer, inners)| {
            (
                outer.strip_suffix(" bags").unwrap(),
                inners
                    .strip_suffix('.')
                    .and_then(|s| (s != "no other bags").as_some(s))
                    .map_or(Vec::new(), |inners| {
                        inners
                            .split(", ")
                            .map(|phrase| {
                                phrase
                                    .strip_suffix(" bags")
                                    .or_else(|| phrase.strip_suffix(" bag"))
                                    .unwrap()
                                    .split_once(' ')
                                    .map(|(num, color)| (num.parse().unwrap(), color))
                                    .unwrap()
                            })
                            .collect_vec()
                    }),
            )
        });
    // iter.collect()
    let mut map = fmap(2000);
    iter.for_each(|(l, r)| {
        map.insert(l, r);
    });
    map
}

fn invmap<'a>(map: &FMap<&'a str, Vec<(i32, &'a str)>>) -> FMap<&'a str, Vec<(i32, &'a str)>> {
    let contained = map
        .iter()
        .flat_map(|(&k, v)| v.iter().map(move |&(l, r)| (r, (l, k))));

    let mut invmap = fmap(2000);

    for (color, rhs) in contained {
        // dbg!((color, rhs));
        invmap
            .entry(color)
            .and_modify(|v: &mut Vec<_>| v.push(rhs))
            .or_insert_with(|| vec![rhs]);
    }
    invmap
}

fn invbagmap(input: &str) -> FMap<&str, Vec<&str>> {
    let contained = input
        .lines()
        .map(|line| line.split_once(" contain ").unwrap())
        .flat_map(|(outer, inners)| {
            let outer = outer.strip_suffix('s').unwrap();
            inners
                .strip_suffix('.')
                .and_then(|s| (s != "no other bags").as_some(s))
                .into_iter()
                .flat_map(|inners| {
                    inners.split(", ").map(|phrase| {
                        phrase
                            .strip_suffix('s')
                            .unwrap_or(phrase)
                            // .strip_suffix(" bags").or_else(|| phrase.strip_suffix(" bag")).unwrap()
                            .split_once(' ')
                            .map(|(_, color)| (color))
                            .unwrap()
                    })
                })
                .map(move |contained| (contained, outer))
        });

    let mut invmap = fmap(2000);

    for (color, rhs) in contained {
        // dbg!((color, rhs));
        invmap
            .entry(color)
            .and_modify(|v: &mut Vec<_>| v.push(rhs))
            .or_insert_with(|| vec![rhs]);
    }

    dbg!(invmap)
}

pub fn part1(input: &str) -> usize {
    // let map = bagmap(input);
    // let invmap = invmap(&map);
    let invmap = invbagmap(input);

    let mut queue: VecDeque<&str> = VecDeque::with_capacity(invmap.len());
    queue.push_front("shiny gold bag");
    let mut found: FSet<&str> = fset(invmap.len());

    while let Some(next) = queue.pop_front() {
        if let Some(containers) = invmap.get(&next) {
            queue.extend(containers.iter().filter(|color| found.insert(color)))
        }
        // queue.extend(invmap.get(&next).into_iter().flat_map(|containers| {
        // containers
        // .iter()
        // .map(|(_, container)| container)
        // .filter(|color| found.insert(color))
        // .collect_vec()
        // }))
    }

    // dbg!(found);
    // dbg!((map.len(), map.values().flat_map(|v| v.iter().map(|(l,r)|r)).unique().count()));
    // dbg!(map);

    found.len()
}

pub fn part2(input: &str) -> i32 {
    let map = bagmap(input);

    let mut queue: VecDeque<(i32, &str)> = VecDeque::with_capacity(map.len());
    queue.push_front((1, "shiny gold"));
    let mut count = 0;

    while let Some((outernum, color)) = queue.pop_front() {
        // if let Some(containers) = map.get(&color) {
        //     queue.extend(
        //         containers
        //             .iter()
        //             .map(|&(num, color)| (outernum * num, color))
        //             .inspect(|(num, _)| count += num),
        //     )
        // }
        // queue.extend(map.get(&color).into_iter().flat_map(|containers| {
        // containers
        // .iter()
        // .map(|&(num, color)| (outernum * num, color))
        // .inspect(|(num, _)| count += num)
        // .collect_vec()
        // }))
        queue.extend(
            map.get(&color)
                .unwrap()
                .iter()
                .map(|&(num, color)| (outernum * num, color))
                .inspect(|(num, _)| count += num), // .collect_vec(),
        )
    }

    count
}

#[test]
fn test() {
    let input = read_input("input7.txt").unwrap();
    assert_eq!(part1(&input), 213);
    assert_eq!(part2(&input), 38426);
}
