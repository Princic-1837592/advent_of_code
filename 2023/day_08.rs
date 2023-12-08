//! https://adventofcode.com/2023/day/8
//! https://adventofcode.com/2023/day/8/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

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
        let mut numbers = [0; 3];
        let mut i = 0;
        let mut chars = line.chars().peekable();
        while chars.peek().is_some() {
            let next = chars.next().unwrap();
            if next.is_ascii_alphabetic() {
                numbers[i] = (next as usize - 'A' as usize) * 26 * 26
                    + (chars.next().unwrap() as usize - 'A' as usize) * 26
                    + (chars.next().unwrap() as usize - 'A' as usize);
                i += 1;
            }
        }
        let [node, left, right] = numbers;
        if nodes.len() <= node {
            nodes.extend(vec![[usize::MAX, usize::MAX]; node - nodes.len() + 1]);
        }
        nodes[node][0] = left;
        nodes[node][1] = right;
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
    use super::Parsed;

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    fn lcm(a: usize, b: usize) -> usize {
        if a > b {
            (a / gcd(a, b)) * b
        } else {
            (b / gcd(a, b)) * a
        }
    }

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
