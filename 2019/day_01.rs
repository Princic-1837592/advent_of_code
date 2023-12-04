//! https://adventofcode.com/2019/day/1
//! https://adventofcode.com/2019/day/1/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

pub mod part1 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let mass = parse(input);
        mass.iter().map(|mass| (mass / 3) - 2).sum()
    }
}

pub mod part2 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let mut fuel: Vec<_> = parse(input).into_iter().map(|m| m as isize).collect();
        let mut total = 0;
        loop {
            fuel = fuel
                .iter()
                .flat_map(|mass| {
                    let result = (mass / 3) as isize - 2;
                    (result > 0).then_some(result)
                })
                .collect();
            if fuel.is_empty() {
                break;
            }
            total += fuel.iter().sum::<isize>();
        }
        total as usize
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_01_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
