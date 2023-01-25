//! https://adventofcode.com/2015/day/9
//! https://adventofcode.com/2015/day/9/input

use std::{collections::HashMap, time::Instant};
use std::fs::read_to_string;

use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<usize>> {
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
    use crate::day_09::{find_shortest, parse};

    pub fn solve(input: &str) -> usize {
        let graph = parse(input);
        find_shortest(graph, usize::MAX, <usize as PartialOrd>::lt)
    }
}

pub mod part2 {
    use crate::day_09::{find_shortest, parse};

    pub fn solve(input: &str) -> usize {
        let graph = parse(input);
        find_shortest(graph, usize::MIN, <usize as PartialOrd>::gt)
    }
}

pub fn main(test: bool) {
    let test_input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_09_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
