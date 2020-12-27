use std::collections;

use collections::hash_map::Entry::{Occupied, Vacant};

use crate::searcher::BFSearcher;
use crate::utils::*;

pub fn part1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line.split_once('(').unwrap_or_else(|| (line, "")))
        .map(|(ingr, alle)| {
            (
                ingr,
                alle.strip_prefix("contains ")
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap(),
            )
        })
        .map(|(ingr, alle)| (ingr.split(' ').filter(|i| !i.is_empty()), alle.split(", ")))
        .collect_vec();

    let mut ingredients = fmap(lines.len() * 10);
    let mut allergens = fmap(lines.len() * 10);
    for (ingr, alle) in &lines {
        for ingredient in ingr.clone() {
            ingredients
                .entry(ingredient)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        for allergen in alle.clone() {
            allergens
                .entry(allergen)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }

    let mut possible_ingredients: FMap<&str, FSet<&str>> = fmap(allergens.len());
    for (ingr, alle) in lines {
        let ingr: FSet<&str> = ingr.collect();
        let alle: FSet<&str> = alle.collect();
        for &allergen in &alle {
            match possible_ingredients.entry(allergen) {
                Occupied(mut occ) => occ
                    .get_mut()
                    .retain(|&ingredient| ingr.contains(ingredient)),
                Vacant(vac) => {
                    vac.insert(ingr.clone());
                }
            }
        }
    }

    let allergic_ingredients =
        possible_ingredients
            .values()
            .fold(fset::<&str>(allergens.len()), |mut set, ingr| {
                set.extend(ingr);
                set
            });

    ingredients
        .iter()
        .filter(|(&i, _)| !allergic_ingredients.contains(i))
        .map(|(i, &v)| v)
        .sum()
}

pub fn part2(input: &str) -> String {
    let lines = input
        .lines()
        .map(|line| line.split_once('(').unwrap_or_else(|| (line, "")))
        .map(|(ingr, alle)| {
            (
                ingr,
                alle.strip_prefix("contains ")
                    .unwrap()
                    .strip_suffix(')')
                    .unwrap(),
            )
        })
        .map(|(ingr, alle)| (ingr.split(' ').filter(|i| !i.is_empty()), alle.split(", ")))
        .collect_vec();

    let mut ingredients = fmap(lines.len() * 10);
    let mut allergens = fmap(lines.len() * 10);
    for (ingr, alle) in &lines {
        for ingredient in ingr.clone() {
            ingredients
                .entry(ingredient)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        for allergen in alle.clone() {
            allergens
                .entry(allergen)
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
    }

    let mut possible_ingredients: FMap<&str, FSet<&str>> = fmap(allergens.len());
    for (ingr, alle) in lines {
        let ingr: FSet<&str> = ingr.collect();
        let alle: FSet<&str> = alle.collect();
        for &allergen in &alle {
            match possible_ingredients.entry(allergen) {
                Occupied(mut occ) => occ
                    .get_mut()
                    .retain(|&ingredient| ingr.contains(ingredient)),
                Vacant(vac) => {
                    vac.insert(ingr.clone());
                }
            }
        }
    }

    let allergic_ingredients =
        possible_ingredients
            .values()
            .fold(fset::<&str>(allergens.len()), |mut set, ingr| {
                set.extend(ingr);
                set
            });

    let init = possible_ingredients
        .iter()
        .filter(|&(&alle, ingrs)| ingrs.len() == 1)
        .map(|(&alle, _)| alle)
        .collect_vec();
    let bfs: BFSearcher<&str, FSet<_>, _> = BFSearcher::new_all(init, |&allergen: &&str| {
        let ingredient = possible_ingredients.get(allergen).unwrap();
        debug_assert_eq!(ingredient.len(), 1);
        let ingredient = *ingredient.iter().next().unwrap();

        let mut ns = Vec::with_capacity(allergens.len());
        for (&k, v) in possible_ingredients.iter_mut() {
            if k == allergen {
                continue;
            }
            v.remove(ingredient);
            debug_assert!(v.len() >= 1);
            if v.len() == 1 {
                ns.push(k);
            }
        }
        ns
    });
    for _ in bfs {}

    let list = (&possible_ingredients)
        .into_iter()
        .sorted_by_key(|&(&allergen, ingredient)| allergen)
        .map(|(_, ingredient)| ingredient.iter().next().unwrap())
        .join(",");

    list
}

#[test]
fn test() {
    let input = read_input("input21.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part1(&input), 2125);
    assert_eq!(part2(&input), "phc,spnd,zmsdzh,pdt,fqqcnm,lsgqf,rjc,lzvh");
}
