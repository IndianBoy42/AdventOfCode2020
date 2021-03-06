use std::unimplemented;

use crate::utils::*;

pub fn part1(input: &str) -> usize {
    let lines = input.lines().map(|line| line).collect_vec();
    let start: usize = lines[0].parse().unwrap();
    let buses: Vec<usize> = lines[1]
        .split(',')
        .filter_map(|bus| bus.parse().ok())
        .collect_vec();

    let (bus, minute) = (start..)
        .find_map(|minute| {
            buses
                .iter()
                .find(|&&bus| minute % bus == 0)
                .map(|&bus| (bus, minute))
        })
        .unwrap();

    bus * (minute - start)
}

fn egcd<I: num::Integer + Copy>(a: I, b: I) -> (I, I, I) {
    if a == I::zero() {
        (b, I::zero(), I::one())
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv<I: num::Integer + Copy>(x: I, n: I) -> Option<I> {
    // let (g, x) = {let e = x.extended_gcd(&n); (e.gcd, e.x)};
    let (g, x, _) = egcd(x, n);
    if g == I::one() {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder<I>(residues: &[I], modulii: &[I]) -> Option<I>
where
    I: num::Integer + std::iter::Product<I> + std::marker::Copy + std::ops::AddAssign,
{
    let prod = modulii.iter().copied().product::<I>();

    let mut sum = I::zero();

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

pub fn part2(input: &str) -> i64 {
    let lines = input.lines().map(|line| line).collect_vec();
    let buses = lines[1]
        .split(',')
        .enumerate()
        .filter_map(|(pos, bus)| bus.parse().map(|bus: i64| (pos as i64, bus)).ok());

    if false {
        let mut buses = buses.sorted_by_key(|&(_, bus)| Reverse(bus));
        debug_assert!(buses
            .clone()
            .map(|(_, bus)| bus)
            .tuple_combinations()
            .all(|(x, y)| num::integer::gcd(x, y) == 1)); // pairwise coprime, CRT applies

        if true {
            // let (prod, ans) = (1, 0);
            let (pos, bus) = buses.next().unwrap();
            let prod = bus;
            let ans = -pos % prod;

            let (prod, ans) = buses.fold((prod, ans), |(prod, ans), (pos, bus)| {
                let ans = (ans..)
                    .step_by(prod as usize)
                    .find(|&t| (t + pos) % bus == 0)
                    .unwrap();

                let prod = prod * bus; // If not pairwise coprime then use num::integer::lcm here?
                (prod, ans)
            });

            ans
        } else {
            let f = |(pos2, bus2): (i64, i64), (pos, bus)| {
                let ans = (((-pos2) % bus2)..)
                    .step_by(bus2 as usize)
                    .find(|&t| (t + pos) % bus == 0)
                    .unwrap();

                let prod = bus2 * bus; // If not pairwise coprime then use num::integer::lcm here?

                (-ans, prod)
            };
            let init = (0, 1);

            // let (ans, prod) = buses.collect_vec().into_par_iter().reduce(|| init, f);
            let (ans, prod) = buses.tree_fold1(f).unwrap();
            // let (ans, prod) = buses.fold(init, f);

            -ans
        }
    } else {
        let (poss, buses): (Vec<i64>, Vec<i64>) = buses.map(|(pos, bus)| (bus - pos, bus)).unzip();
        debug_assert!(buses
            .iter()
            .copied()
            .tuple_combinations()
            .all(|(x, y)| num::integer::gcd(x, y) == 1)); // pairwise coprime, CRT applies

        let t = chinese_remainder(&poss, &buses).unwrap();

        for (pos, bus) in izip!(poss, buses) {
            debug_assert_eq!((t - pos) % bus, 0, "p{}, b{}", pos, bus)
        }
        t
    }
}

#[test]
fn test() {
    let input = read_input("input13.txt").unwrap();
    assert_eq!(part1(&input), 119);
    assert_eq!(part2(&input), 1106724616194525);
}
