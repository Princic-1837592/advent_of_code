//! https://adventofcode.com/2015/day/10
//! https://adventofcode.com/2015/day/10/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

fn solve_iter(input: &str, iterations: usize) -> usize {
    let mut numbers = String::from(input);
    for _ in 0..iterations {
        let mut next = String::new();
        numbers
            .chars()
            .chain(" ".chars())
            .fold((' ', 1), |(previous, repetitions), char| {
                if char == previous {
                    (previous, repetitions + 1)
                } else {
                    if previous != ' ' {
                        next.push(char::from_digit(repetitions, 10).unwrap());
                        next.push(previous);
                    }
                    (char, 1)
                }
            });
        numbers = next;
    }
    numbers.len()
}

pub mod part1 {
    use super::solve_iter;

    pub fn solve(input: &str) -> usize {
        solve_iter(input, 40)
    }
}

pub mod part2 {
    use super::solve_iter;

    pub fn solve(input: &str) -> usize {
        solve_iter(input, 50)
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "1".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2015/day_10_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let result = part1::solve(&puzzle_input);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(&puzzle_input);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
