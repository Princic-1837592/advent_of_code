//! https://adventofcode.com/2018/day/7
//! https://adventofcode.com/2018/day/7/input

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
};

fn parse(input: &str) -> HashMap<usize, HashSet<usize>> {
    let mut result = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let src = (parts.nth(1).unwrap().chars().next().unwrap() as u8 - b'A') as usize;
        let dst = (parts.nth(5).unwrap().chars().next().unwrap() as u8 - b'A') as usize;
        result.entry(src).or_insert_with(HashSet::new);
        result.entry(dst).or_insert_with(HashSet::new).insert(src);
    }
    result
}

pub mod part1 {
    use crate::day_07::parse;

    pub fn solve(input: &str) -> String {
        let mut graph = parse(input);
        let mut result = String::new();
        while !graph.is_empty() {
            let min = *graph
                .iter()
                .filter(|(_, srcs)| srcs.is_empty())
                .map(|(dst, _)| dst)
                .min()
                .unwrap();
            result.push((min as u8 + b'A') as char);
            graph.values_mut().for_each(|srcs| {
                srcs.remove(&min);
            });
            graph.remove(&min);
        }
        result
    }
}

pub mod part2 {
    use crate::day_07::parse;

    pub fn solve(input: &str, workers: usize, seconds: usize) -> usize {
        let mut graph = parse(input);
        let mut workers = vec![(0, 0); workers];
        let mut total_seconds = 0;
        while !graph.is_empty() || workers.iter().map(|(seconds, _)| seconds).sum::<usize>() != 0 {
            for worker in workers.iter_mut().filter(|(seconds, _)| *seconds != 0) {
                worker.0 -= 1;
                if worker.0 == 0 {
                    graph.values_mut().for_each(|srcs| {
                        srcs.remove(&worker.1);
                    });
                }
            }
            for worker in workers.iter_mut().filter(|(seconds, _)| *seconds == 0) {
                if let Some(&min) = graph
                    .iter()
                    .filter(|(_, srcs)| srcs.is_empty())
                    .map(|(dst, _)| dst)
                    .min()
                {
                    *worker = (seconds + min + 1, min);
                    graph.remove(&min);
                }
            }
            total_seconds += 1;
        }
        total_seconds - 1
    }
}

pub fn main(test: bool) {
    let test_input = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
        .to_owned();
    let (puzzle_input, workers, seconds) = if test {
        (test_input, 2, 0)
    } else {
        (read_to_string("inputs/day_07_input.txt").unwrap(), 5, 60)
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input, workers, seconds));
    println!("Run in {:?}", start.elapsed());
}
