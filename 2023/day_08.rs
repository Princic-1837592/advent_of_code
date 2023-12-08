//! https://adventofcode.com/2023/day/8
//! https://adventofcode.com/2023/day/8/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

use utils::parsing::parse_alpha;

type Node = [usize; 2];

type Parsed = (Vec<usize>, Vec<Node>);

fn parse(input: &str) -> Parsed {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect();
    let mut nodes = vec![[usize::MAX, usize::MAX]];
    for line in lines.skip(1) {
        let node = parse_alpha::<'A'>(&line[0..3]);
        if nodes.len() <= node {
            nodes.extend(vec![[usize::MAX, usize::MAX]; node - nodes.len() + 1]);
        }
        nodes[node][0] = parse_alpha::<'A'>(&line[7..7 + 3]);
        nodes[node][1] = parse_alpha::<'A'>(&line[12..12 + 3]);
    }
    (instructions, nodes)
}

pub mod part1 {
    use super::Parsed;

    pub fn solve((instructions, graph): Parsed) -> usize {
        let mut node = 0;
        for i in 0.. {
            if node == 25 * 26 * 26 + 25 * 26 + 25 {
                return i;
            }
            node = graph[node][instructions[i % instructions.len()]];
        }
        unreachable!()
    }
}

pub mod part2 {
    use utils::math::lcm;

    use super::Parsed;

    pub fn solve((instructions, graph): Parsed) -> usize {
        let nodes: Vec<_> = graph
            .iter()
            .enumerate()
            .filter_map(|(n, d)| (n % 26 == 0 && d[0] != usize::MAX).then_some(n))
            .collect();
        let cycles: Vec<_> = nodes
            .into_iter()
            .map(|mut node| {
                for i in 0.. {
                    if node % 26 == 25 {
                        return i;
                    }
                    node = graph[node][instructions[i % instructions.len()]];
                }
                unreachable!()
            })
            .collect();
        cycles.into_iter().fold(1, lcm)
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_08_input.txt").unwrap()
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

    let start = Instant::now();
    let result = part2::solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    if verbose {
        println!("Total {:?}", total);
    }
    total
}
