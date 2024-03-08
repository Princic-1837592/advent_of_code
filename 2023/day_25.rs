//! https://adventofcode.com/2023/day/25
//! https://adventofcode.com/2023/day/25/input

use std::{
    collections::HashMap,
    fs::read_to_string,
    time::{Duration, Instant},
};

type Parsed = Vec<Vec<usize>>;

fn parse(input: &str) -> Parsed {
    let mut indexes = HashMap::new();
    for line in input.lines() {
        for wire in line[5..].split_whitespace().chain([&line[..3]]) {
            if !indexes.contains_key(wire) {
                indexes.insert(wire, indexes.len());
            }
        }
    }
    let mut result = vec![vec![]; indexes.len()];
    for line in input.lines() {
        let from = *indexes.get(&line[..3]).unwrap();
        for wire in line[5..]
            .split_whitespace()
            .map(|wire| *indexes.get(wire).unwrap())
        {
            result[from].push(wire);
            result[wire].push(from);
        }
    }
    result
}

pub mod part1 {
    use std::collections::HashSet;

    use super::Parsed;

    pub fn solve(graph: Parsed) -> usize {
        let nodes: HashSet<_> = graph
            .iter()
            .enumerate()
            .flat_map(|(n, edges)| {
                edges
                    .iter()
                    .filter_map(move |&e| if n < e { Some((n, e)) } else { None })
            })
            .collect();
        println!("neato -Tsvg -o out.svg");
        println!("graph {{");
        for (from, to) in nodes {
            println!("{}--{};", from, to);
        }
        println!("}}");
        0
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_25_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    if verbose {
        println!("Parsed in {:?}", elapsed);
        total += elapsed;
    }

    let start = Instant::now();
    let result = part1::solve(parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    if verbose {
        println!("Total {:?}", total);
    }
    total
}
