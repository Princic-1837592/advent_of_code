//! https://adventofcode.com/2015/day/24
//! https://adventofcode.com/2015/day/24/input

use std::{fs::read_to_string, time::Instant};

use itertools::Itertools;

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .rev()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn solve_locking_k(packages: &[usize], target: usize, min_len: usize, k: usize) -> Option<usize> {
    let mut results = Vec::new();
    for locked in packages[..min_len].iter().combinations(k) {
        let partial_weight: usize = locked.iter().copied().sum();
        let partial_qe: usize = locked.iter().copied().product();
        for o in 1..=min_len - k + 1 {
            for group in packages[min_len..].iter().combinations(o) {
                if group.iter().copied().sum::<usize>() + partial_weight == target {
                    results.push(group.into_iter().product::<usize>() * partial_qe);
                }
            }
        }
    }
    results.into_iter().min()
}

fn solve_greedy(input: &str, groups: usize) -> usize {
    let mut packages = parse(input);
    packages.sort();
    packages.reverse();
    let target = packages.iter().sum::<usize>() / groups;
    let mut weight = 0;
    let mut min_len = 0;
    while weight < target {
        weight += packages[min_len];
        min_len += 1;
    }
    for k in (1..=min_len - 1).rev() {
        if let Some(qe) = solve_locking_k(&packages, target, min_len, k) {
            return qe;
        }
    }
    unreachable!("No solution found");
}

pub mod part1 {
    use crate::day_24::solve_greedy;

    pub fn solve(input: &str) -> usize {
        solve_greedy(input, 3)
    }
}

pub mod part2 {
    use crate::day_24::solve_greedy;

    pub fn solve(input: &str) -> usize {
        solve_greedy(input, 4)
    }
}

pub fn main(test: bool) {
    let test_input = "1
2
3
4
5
7
8
9
10
11"
    .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_24_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
