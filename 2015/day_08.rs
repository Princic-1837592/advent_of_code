//! https://adventofcode.com/2015/day/8
//! https://adventofcode.com/2015/day/8/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

fn find_difference(string: &str) -> usize {
    let (memory, _, _) = string[1..string.len() - 1].chars().fold(
        (0, false, 0),
        |(memory, last_escaped, skip), char| {
            if skip > 0 {
                (memory, false, skip - 1)
            } else if last_escaped {
                if char == 'x' {
                    (memory + 1, false, 2)
                } else {
                    (memory + 1, false, 0)
                }
            } else if char == '\\' {
                (memory, true, 0)
            } else {
                (memory + 1, false, 0)
            }
        },
    );
    string.len() - memory
}

pub mod part1 {
    use crate::day_08::find_difference;

    pub fn solve(input: &str) -> usize {
        input.lines().map(find_difference).sum()
    }
}

pub mod part2 {
    use crate::day_08::find_difference;

    pub fn solve(input: &str) -> usize {
        input
            .lines()
            .map(|line| {
                let mut result = String::from('"');
                line.chars().for_each(|char| {
                    if char == '\\' || char == '"' {
                        result.push('\\')
                    };
                    result.push(char);
                });
                result.push('"');
                result
            })
            .map(|string| find_difference(&string))
            .sum()
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = r#"""
"abc"
"aaa\"aaa"
"\x27""#
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_08_input.txt").unwrap()
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
