//! https://adventofcode.com/2015/day/8
//! https://adventofcode.com/2015/day/8/input

use std::{fs::read_to_string, time::Instant};

fn find_difference(string: &str) -> usize {
    let (code, memory, _, _) = string[1..string.len() - 1].chars().fold(
        (2, 0, false, 0),
        |(code, memory, last_escaped, skip), char| {
            if skip > 0 {
                (code + 1, memory, false, skip - 1)
            } else if last_escaped {
                if char == 'x' {
                    (code + 1, memory + 1, false, 2)
                } else {
                    (code + 1, memory + 1, false, 0)
                }
            } else if char == '\\' {
                (code + 1, memory, true, 0)
            } else {
                (code + 1, memory + 1, false, 0)
            }
        },
    );
    code - memory
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

pub fn main(test: bool) {
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
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
