use std::iter::once;

use crate::utils::*;

pub fn part1(input: &str) -> i32 {
    let (requirements, your_ticket, nearby_tickets) = input.split("\n\n").collect_tuple().unwrap();

    let requirements: Vec<(&str, (_, _))> = requirements
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(fieldname, ranges)| {
            (
                fieldname,
                ranges
                    .split("or")
                    .filter_map(|range| {
                        range
                            .trim()
                            .split_once('-')
                            .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
                            .map(|(l, r): (i32, i32)| l..=r)
                    })
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect_vec();

    let reqs = || {
        requirements
            .iter()
            .map(|(_, r)| r)
            .flat_map(|(a, b)| once(a).chain(once(b)))
    };

    nearby_tickets
        .lines()
        .skip(1)
        .flat_map(|line| line.split(','))
        .map(|n| n.parse().unwrap())
        .filter(|&n| !reqs().any(|range| range.contains(&n)))
        // .map(|x| dbg!(x))
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let (requirements, your_ticket, nearby_tickets) = input.split("\n\n").collect_tuple().unwrap();

    let requirements: Vec<(&str, (_, _))> = requirements
        .lines()
        .map(|line| line.split_once(':').unwrap())
        .map(|(fieldname, ranges)| {
            (
                fieldname,
                ranges
                    .split("or")
                    .map(|range| {
                        range
                            .trim()
                            .split_once('-')
                            .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
                            .map(|(l, r): (i32, i32)| l..=r)
                            .unwrap()
                    })
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect_vec();

    let possibilities = {
        let reqs = || requirements.iter().map(|(_, r)| r);

        let nearby_tickets = nearby_tickets.lines().skip(1).filter_map(|line| {
            let ticket = line
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec();
            let ok = ticket
                .iter()
                .all(|n| reqs().any(|(req1, req2)| req1.contains(n) || req2.contains(n)));
            ok.as_some(ticket)
        });

        // Each position in a row -> which requirements it could be
        let mut possibilities = (0..requirements.len())
            .map(|_| (0..requirements.len()).collect::<BitSet>())
            .collect_vec();

        for ticket in nearby_tickets {
            for (i, n, poss) in izip!(0.., ticket, &mut possibilities) {
                reqs()
                    .enumerate()
                    .filter(|(_, (req1, req2))| !req1.contains(&n) && !req2.contains(&n))
                    .for_each(|(j, r)| {
                        poss.remove(j);
                        // if poss.len() <= 1 {
                        //     dbg!(i, j, r, n, &poss);
                        // }
                    });
            }

            debug_assert!(!possibilities.iter().any(|poss| poss.len() == 0));
            if possibilities.iter().all(|poss| poss.len() == 1) {
                break;
            }
        }

        possibilities
    };

    let requirements = {
        let mut possibilities = possibilities;

        // Each position in a row -> which requirement
        let mut assignments = vec![-1; possibilities.len()];
        while let Some((i, poss)) = possibilities
            .iter()
            .enumerate()
            .find(|&(i, poss)| poss.len() == 1)
        {
            assignments[i] = poss.iter().next().unwrap() as i32;
            possibilities.iter_mut().for_each(|poss| {
                poss.remove(assignments[i].try_into().unwrap());
            });
        }

        debug_assert!(assignments.iter().all(|&i| i >= 0));
        debug_assert!(possibilities.iter().all(|p| p.len() == 0));

        dbg!(assignments)
            .into_iter()
            .map(|ass| requirements[ass as usize].clone())
            .collect_vec()
    };

    let your_ticket = your_ticket.lines().nth(1).unwrap()
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect_vec();

    requirements
        .into_iter()
        .enumerate()
        .filter(|(_, (r, _))| r.starts_with("departure"))
        .map(|(i, _)| your_ticket[i] as i64).product()
}

#[test]
fn test() {
    let input = read_input("input16.txt").unwrap();
    assert_eq!(part1(&input), 26869);
    assert_eq!(part2(&input), 855275529001);
}
