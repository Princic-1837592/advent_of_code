//! https://adventofcode.com/2015/day/24
//! https://adventofcode.com/2015/day/24/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<usize> {
    input
        .lines()
        .rev()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn explore<const G: usize>(
    packages: &[usize],
    package: usize,
    max_per_group: usize,
    groups: &mut [usize; G],
    state @ (legroom, qe): (usize, usize),
    mut min_state: (usize, usize),
) -> (usize, usize) {
    if package == packages.len() {
        return state;
    }
    if state >= min_state {
        return (usize::MAX, usize::MAX);
    }

    let weight = packages[package];
    if groups[0] + weight <= max_per_group {
        groups[0] += weight;
        let first = explore(
            packages,
            package + 1,
            max_per_group,
            groups,
            (legroom + 1, qe * weight),
            min_state,
        );
        groups[0] -= weight;
        if first < min_state {
            min_state = first;
        }
    }
    for group in 1..G {
        if groups[group] + weight <= max_per_group {
            groups[group] += weight;
            let result = explore(
                packages,
                package + 1,
                max_per_group,
                groups,
                state,
                min_state,
            );
            groups[group] -= weight;
            if result <= min_state {
                min_state = result;
            }
        }
    }
    min_state
}

pub mod part1 {
    use crate::day_24::{explore, parse};

    pub fn solve(input: &str) -> usize {
        let packages = parse(input);
        explore(
            &packages,
            0,
            packages.iter().sum::<usize>() / 3,
            &mut [0; 3],
            (0, 1),
            (usize::MAX, usize::MAX),
        )
        .1
    }
}

pub mod part2 {
    use crate::day_24::{explore, parse};

    pub fn solve(input: &str) -> usize {
        let packages = parse(input);
        explore(
            &packages,
            0,
            packages.iter().sum::<usize>() / 4,
            &mut [0; 4],
            (0, 1),
            (usize::MAX, usize::MAX),
        )
        .1
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
