//! https://adventofcode.com/2015/day/19
//! https://adventofcode.com/2015/day/19/input

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    time::{Duration, Instant},
};

type Parsed<'a> = (HashMap<&'a str, HashSet<&'a str>>, String);

fn parse(input: &str) -> Parsed {
    let mut lines = input.lines();
    let mut replacements = HashMap::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(" => ");
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        replacements
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
    }
    let molecule = lines.next().unwrap();
    (replacements, molecule.to_owned())
}

pub mod part1 {
    use std::collections::{HashSet, VecDeque};

    use itertools::Itertools;
    use regex::Regex;

    use super::Parsed;

    pub fn solve((replacements, molecule): Parsed) -> usize {
        let pattern = Regex::new(r"[A-Z][a-z]?").unwrap();
        let atoms: Vec<_> = pattern
            .find_iter(&molecule)
            .map(|molecule| molecule.as_str())
            .collect();
        let (mut before, mut after) = (
            String::with_capacity(molecule.len()),
            atoms.iter().collect::<VecDeque<_>>(),
        );
        let mut result = HashSet::new();
        atoms.iter().for_each(|&atom| {
            after.pop_front();
            result.extend(
                replacements
                    .get(atom)
                    .unwrap_or(&HashSet::new())
                    .iter()
                    .map(|replacement| {
                        format!("{}{}{}", before, replacement, after.iter().join(""))
                    }),
            );
            before.push_str(atom);
        });
        result.len()
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use rand::{prelude::SliceRandom, thread_rng};

    use super::Parsed;

    pub fn solve((replacements, molecule): Parsed) -> usize {
        let mut reversed = HashMap::with_capacity(replacements.values().map(|v| v.len()).sum());
        for (k, v) in replacements.into_iter() {
            for v in v {
                reversed.entry(v).or_insert(k);
            }
        }
        let mut shuffled_rules: Vec<_> = reversed.keys().copied().collect();
        loop {
            shuffled_rules.shuffle(&mut thread_rng());
            let mut molecule = molecule.clone();
            let mut steps = 0;
            while molecule != "e" {
                if let Some(rule) = shuffled_rules.iter().find(|&rule| molecule.contains(rule)) {
                    molecule = molecule.replacen(rule, reversed.get(rule).unwrap(), 1);
                } else {
                    steps = 0;
                    break;
                }
                steps += 1;
            }
            if steps != 0 {
                return steps;
            }
        }
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "e => H
e => O
H => HO
H => OH
O => HH

HOHOHO"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_19_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    println!("Parsed in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part1::solve(parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
