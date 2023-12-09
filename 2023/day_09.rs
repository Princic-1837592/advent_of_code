//! https://adventofcode.com/2023/day/9
//! https://adventofcode.com/2023/day/9/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

type History = Vec<isize>;

type Parsed = Vec<History>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

pub mod part1 {
    use super::{History, Parsed};

    fn reduce(mut history: History) -> isize {
        let mut result = 0;
        let mut all_zero = false;
        while !all_zero {
            all_zero = true;
            for i in 0..history.len() - 1 {
                let diff = history[i + 1] - history[i];
                if diff != 0 {
                    all_zero = false;
                }
                history[i] = diff;
            }
            result += history.pop().unwrap();
        }
        result
    }

    pub fn solve(histories: Parsed) -> isize {
        histories.into_iter().map(reduce).sum()
    }
}

pub mod part2 {
    use super::{History, Parsed};

    fn reduce(mut history: History) -> isize {
        let mut result = 0;
        let mut all_zero = false;
        let mut sum = true;
        while !all_zero {
            all_zero = true;
            result += if sum { history[0] } else { -history[0] };
            sum = !sum;
            for i in 0..history.len() - 1 {
                let diff = history[i + 1] - history[i];
                if diff != 0 {
                    all_zero = false;
                }
                history[i] = diff;
            }
            history.pop();
        }
        result
    }

    pub fn solve(histories: Parsed) -> isize {
        histories.into_iter().map(reduce).sum()
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
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
