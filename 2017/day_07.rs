//! https://adventofcode.com/2017/day/7
//! https://adventofcode.com/2017/day/7/input

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
};

#[allow(clippy::type_complexity)]
fn parse(
    input: &str,
) -> (
    HashMap<&str, HashSet<&str>>,
    HashMap<&str, isize>,
    HashMap<&str, isize>,
) {
    let mut graph = HashMap::new();
    let mut weights = HashMap::new();
    let mut incoming = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let mut left = parts.next().unwrap().split_whitespace();
        let name = left.next().unwrap();
        let weight = left
            .next()
            .unwrap()
            .trim_matches(|c| c == '(' || c == ')')
            .parse()
            .unwrap();
        graph.insert(name, HashSet::new());
        weights.insert(name, weight);
        if let Some(children) = parts.next() {
            for child in children.split(", ") {
                graph.get_mut(name).unwrap().insert(child);
                match incoming.entry(child) {
                    Entry::Occupied(mut entry) => *entry.get_mut() += 1,
                    Entry::Vacant(entry) => {
                        entry.insert(1);
                    }
                }
            }
        }
    }
    (graph, weights, incoming)
}

fn find_root<'a>(
    graph: &HashMap<&'a str, HashSet<&str>>,
    incoming: &HashMap<&str, isize>,
) -> &'a str {
    graph
        .keys()
        .find(|&&node| *incoming.get(node).unwrap_or(&0) == 0)
        .unwrap()
}

pub mod part1 {
    use super::{find_root, parse};

    pub fn solve(input: &str) -> &str {
        let (graph, _, incoming) = parse(input);
        find_root(&graph, &incoming)
    }
}

pub mod part2 {
    use std::collections::{HashMap, HashSet};

    use super::{find_root, parse};

    enum Exploration {
        Result(isize),
        Weight(isize),
    }

    fn explore(
        node: &str,
        graph: &HashMap<&str, HashSet<&str>>,
        weights: &HashMap<&str, isize>,
    ) -> Exploration {
        if graph.get(node).unwrap().is_empty() {
            return Exploration::Weight(*weights.get(node).unwrap());
        }
        let mut sub_weights = HashMap::new();
        for child in graph.get(node).unwrap().iter() {
            match explore(child, graph, weights) {
                Exploration::Weight(weight) => {
                    sub_weights.entry(weight).or_insert((child, 0)).1 += 1;
                }
                result => return result,
            }
        }
        if sub_weights.len() == 1 {
            Exploration::Weight(
                *sub_weights.keys().next().unwrap() * graph.get(node).unwrap().len() as isize
                    + weights.get(node).unwrap(),
            )
        } else {
            let total_right = *sub_weights
                .iter()
                .max_by_key(|(_, (_, count))| count)
                .unwrap()
                .0;
            let (total_wrong, (&wrong_node, _)) = sub_weights
                .iter()
                .min_by_key(|(_, (_, count))| count)
                .unwrap();
            let wrong = *weights.get(wrong_node).unwrap();
            Exploration::Result(wrong + total_right - total_wrong)
        }
    }

    pub fn solve(input: &str) -> isize {
        let (graph, weights, incoming) = parse(input);
        if let Exploration::Result(result) = explore(find_root(&graph, &incoming), &graph, &weights)
        {
            return result;
        }
        unreachable!()
    }
}

pub fn main(test: bool) {
    let test_input = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2017/day_07_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
