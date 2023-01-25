//! https://adventofcode.com/2015/day/13
//! https://adventofcode.com/2015/day/13/input

use std::{collections::HashMap, time::Instant};
use std::fs::read_to_string;

use itertools::Itertools;

fn parse(input: &str) -> Vec<Vec<isize>> {
    let edges: Vec<_> = input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(' ').collect();
            (
                parts[0],
                &parts[10][..parts[10].len() - 1],
                parts[3].parse::<isize>().unwrap() * if parts[2] == "lose" { -1 } else { 1 },
            )
        })
        .collect();
    let nodes = ((edges.len()) as f32).sqrt() as usize + 1;
    let mut graph = vec![vec![0; nodes]; nodes];
    let mut indexes = HashMap::new();
    for (src, dst, weight) in edges {
        let len = indexes.len();
        let src_index = *indexes.entry(src).or_insert(len);
        let len = indexes.len();
        let dst_index = *indexes.entry(dst).or_insert(len);
        graph[src_index][dst_index] = weight;
    }
    graph
}

fn find_shortest(graph: Vec<Vec<isize>>, init: isize, cmp: fn(&isize, &isize) -> bool) -> isize {
    let mut min_distance = init;
    (0..graph.len()).permutations(graph.len()).for_each(|path| {
        let distance = path
            .iter()
            .fold((0, path[path.len() - 1]), |(distance, previous), &node| {
                (
                    distance + graph[previous][node] + graph[node][previous],
                    node,
                )
            })
            .0;
        if cmp(&distance, &min_distance) {
            min_distance = distance;
        }
    });
    min_distance
}

pub mod part1 {
    use crate::day_13::{find_shortest, parse};

    pub fn solve(input: &str) -> isize {
        let graph = parse(input);
        find_shortest(graph, isize::MIN, <isize as PartialOrd>::gt)
    }
}

pub mod part2 {
    use crate::day_13::{find_shortest, parse};

    pub fn solve(input: &str) -> isize {
        let mut graph = parse(input);
        graph.push(vec![0; graph.len()]);
        graph.iter_mut().for_each(|node| node.push(0));
        find_shortest(graph, isize::MIN, <isize as PartialOrd>::gt)
    }
}

pub fn main(test: bool) {
    let test_input = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_13_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
