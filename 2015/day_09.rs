//! https://adventofcode.com/2015/day/9
//! https://adventofcode.com/2015/day/9/input

use std::{
    collections::HashMap,
    fs::read_to_string,
    time::{Duration, Instant},
};

use itertools::Itertools;

type Parsed = Vec<Vec<usize>>;

fn parse(input: &str) -> Parsed {
    let edges: Vec<_> = input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(' ').collect();
            (parts[0], parts[2], parts[4].parse().unwrap())
        })
        .collect();
    let nodes = ((edges.len() * 2) as f32).sqrt() as usize + 1;
    let mut graph = vec![vec![0; nodes]; nodes];
    let mut indexes = HashMap::new();
    for (src, dst, weight) in edges {
        let len = indexes.len();
        let src_index = *indexes.entry(src).or_insert(len);
        let len = indexes.len();
        let dst_index = *indexes.entry(dst).or_insert(len);
        graph[src_index][dst_index] = weight;
        graph[dst_index][src_index] = weight;
    }
    graph
}

fn find_shortest(graph: Vec<Vec<usize>>, init: usize, cmp: fn(&usize, &usize) -> bool) -> usize {
    let mut min_distance = init;
    (0..graph.len()).permutations(graph.len()).for_each(|path| {
        let distance = path
            .iter()
            .skip(1)
            .fold((0, path[0]), |(distance, previous), &node| {
                (distance + graph[previous][node], node)
            })
            .0;
        if cmp(&distance, &min_distance) {
            min_distance = distance;
        }
    });
    min_distance
}

pub mod part1 {
    use crate::day_09::{find_shortest, Parsed};

    pub fn solve(_input: &str, graph: Parsed) -> usize {
        find_shortest(graph, usize::MAX, <usize as PartialOrd>::lt)
    }
}

pub mod part2 {
    use crate::day_09::{find_shortest, Parsed};

    pub fn solve(_input: &str, graph: Parsed) -> usize {
        find_shortest(graph, usize::MIN, <usize as PartialOrd>::gt)
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_09_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    println!("Parsed in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part1::solve(&puzzle_input, parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(&puzzle_input, parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
