//! https://adventofcode.com/2015/day/17
//! https://adventofcode.com/2015/day/17/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<usize> {
    let mut result: Vec<_> = input.lines().map(|line| line.parse().unwrap()).collect();
    result.sort();
    result.reverse();
    result
}

pub mod part1 {
    use crate::day_17::parse;

    fn explore(
        containers: &[usize],
        container: usize,
        states: &mut [bool],
        total: usize,
        target: usize,
    ) -> usize {
        if total == target {
            return 1;
        }
        if total > target || container >= containers.len() {
            return 0;
        }
        let mut result = 0;
        result += explore(containers, container + 1, states, total, target);
        states[container] = true;
        result += explore(
            containers,
            container + 1,
            states,
            total + containers[container],
            target,
        );
        states[container] = false;
        result
    }

    pub fn solve(input: &str, target: usize) -> usize {
        let containers = parse(input);
        explore(
            &containers,
            0,
            &mut vec![false; containers.len()],
            0,
            target,
        )
    }
}

pub mod part2 {
    use crate::day_17::parse;

    fn explore(
        containers: &[usize],
        container: usize,
        states: &mut [bool],
        used: usize,
        total: usize,
        target: usize,
        solutions: &mut [usize],
    ) {
        if total == target {
            solutions[used - 1] += 1;
            return;
        }
        if total > target || container >= containers.len() {
            return;
        }
        explore(
            containers,
            container + 1,
            states,
            used,
            total,
            target,
            solutions,
        );
        states[container] = true;
        explore(
            containers,
            container + 1,
            states,
            used + 1,
            total + containers[container],
            target,
            solutions,
        );
        states[container] = false;
    }

    pub fn solve(input: &str, target: usize) -> usize {
        let containers = parse(input);
        let mut solutions = vec![0; containers.len()];
        explore(
            &containers,
            0,
            &mut vec![false; containers.len()],
            0,
            0,
            target,
            &mut solutions,
        );
        *solutions.iter().find(|&&solution| solution != 0).unwrap()
    }
}

pub fn main(test: bool) {
    let test_input = "20
15
10
5
5"
    .to_owned();
    let (puzzle_input, target) = if test {
        (test_input, 25)
    } else {
        (read_to_string("inputs/day_17_input.txt").unwrap(), 150)
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input, target));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input, target));
    println!("Run in {:?}", start.elapsed());
}
