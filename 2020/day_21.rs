use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn parse(input: &str) -> Vec<(HashSet<&str>, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" (contains ");
            let ingredients = parts.next().unwrap().split(' ').collect();
            let allergens = parts
                .next()
                .unwrap()
                .trim_end_matches(')')
                .split(", ")
                .collect();
            (ingredients, allergens)
        })
        .collect()
}

fn get_possibilities<'a>(
    recipes: &Vec<(HashSet<&'a str>, Vec<&'a str>)>,
) -> HashMap<&'a str, &'a str> {
    let mut possibilities: HashMap<_, HashSet<_>> = HashMap::new();
    for (ingredients, allergens) in recipes {
        for allergen in allergens {
            possibilities
                .entry(allergen)
                .and_modify(|set| *set = set.intersection(ingredients).copied().collect())
                .or_insert_with(|| ingredients.clone());
        }
    }
    while possibilities.values().any(|set| set.len() != 1) {
        let fixed = possibilities
            .iter()
            .filter_map(|(_, set)| (set.len() == 1).then_some(*set.iter().next().unwrap()))
            .collect::<HashSet<_>>();
        possibilities.iter_mut().for_each(|(_, set)| {
            if set.len() > 1 {
                *set = set.difference(&fixed).copied().collect();
            }
        });
    }
    possibilities
        .iter()
        .map(|(k, v)| (**k, *v.iter().next().unwrap()))
        .collect()
}
pub mod part1 {
    use super::{get_possibilities, parse};

    pub fn solve(input: &str) -> usize {
        let recipes = parse(input);
        let possibilities = get_possibilities(&recipes);
        let fixed: Vec<_> = possibilities.values().collect();
        recipes
            .iter()
            .map(|(ingredients, _)| {
                ingredients
                    .iter()
                    .filter(|&ingredient| !fixed.contains(&ingredient))
                    .count()
            })
            .sum()
    }
}

pub mod part2 {
    use crate::day_21::{get_possibilities, parse};

    pub fn solve(input: &str) -> String {
        let recipes = parse(input);
        let possibilities = get_possibilities(&recipes);
        let mut pairs: Vec<_> = possibilities.iter().collect();
        pairs.sort_by_key(|(k, _)| *k);
        pairs
            .iter()
            .map(|(_, v)| *v.to_owned())
            .collect::<Vec<_>>()
            .join(",")
    }
}

pub fn main(test: bool) {
    let test_input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_21_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
