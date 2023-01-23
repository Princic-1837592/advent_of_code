//! https://adventofcode.com/2015/day/10

use std::time::Instant;

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
    use crate::day_10::solve_iter;

    pub fn solve(input: &str) -> usize {
        solve_iter(input, 40)
    }
}

pub mod part2 {
    use crate::day_10::solve_iter;

    pub fn solve(input: &str) -> usize {
        solve_iter(input, 50)
    }
}

pub fn main(test: bool) {
    let test_input = "1".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_10_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
