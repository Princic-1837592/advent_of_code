extern crate core;

use std::time::Instant;

mod part1 {
    use std::collections::{HashMap, HashSet};

    use regex::Regex;

    fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
        let mut map = HashMap::new();
        let pattern = Regex::new(r"\w{3,} \w+ bag").unwrap();
        input
            .lines()
            .map(|l| {
                let mut matches = pattern.find_iter(l).map(|m| m.as_str());
                let first = matches.next().unwrap();
                let others: Vec<_> = matches.collect();
                (first, others)
            })
            .for_each(|(first, others)| {
                others.iter().for_each(|&o| {
                    if !map.contains_key(o) {
                        map.insert(o, HashSet::new());
                    }
                    map.get_mut(o).unwrap().insert(first);
                })
            });
        map
    }

    pub(crate) fn solve(input: &str) -> usize {
        let map = parse(input);
        let mut can_be_reached = HashSet::new();
        can_be_reached.insert("shiny gold bag");
        let mut previous_len = 0;
        while can_be_reached.len() != previous_len {
            previous_len = can_be_reached.len();
            can_be_reached.clone().iter().for_each(|b| {
                if let Some(others) = map.get(b) {
                    can_be_reached.extend(others);
                }
            });
        }
        can_be_reached.len() - 1
    }
}

mod part2 {
    use std::collections::{HashMap, HashSet};

    use regex::Regex;

    fn parse(input: &str) -> HashMap<&str, HashSet<(usize, &str)>> {
        let line_pattern = Regex::new(r"(\w+ \w+) bags contain (.+)\.").unwrap();
        let content_pattern = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
        let mut map = HashMap::new();
        input.lines().for_each(|l| {
            let captures = line_pattern.captures(l).unwrap();
            let container = captures.get(1).unwrap().as_str();
            map.insert(
                container,
                captures
                    .get(2)
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .filter_map(|c| {
                        let content_info = content_pattern.captures(c);
                        content_info.map(|content| {
                            (
                                content.get(1).unwrap().as_str().parse().unwrap(),
                                content.get(2).unwrap().as_str(),
                            )
                        })
                    })
                    .collect(),
            );
        });
        map
    }

    fn explore(map: &HashMap<&str, HashSet<(usize, &str)>>, bag: &str) -> usize {
        let mut bags = 1;
        for (quantity, content) in map.get(bag).unwrap() {
            bags += quantity * explore(map, content);
        }
        bags
    }

    pub(crate) fn solve(input: &str) -> usize {
        let bags = parse(input);
        explore(&bags, "shiny gold") - 1
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_07_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
}
