//! https://adventofcode.com/2017/day/1
//! https://adventofcode.com/2017/day/1/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<usize> {
    input
        .chars()
        .map(|char| char.to_digit(10).unwrap() as usize)
        .collect()
}

fn sum(digits: &Vec<usize>, step: usize) -> usize {
    let mut sum = 0;
    for i in 0..digits.len() {
        if digits[i] == digits[(i + step) % digits.len()] {
            sum += digits[i];
        }
    }
    sum
}

pub mod part1 {
    use super::{parse, sum};

    pub fn solve(input: &str) -> usize {
        let digits = parse(input);
        sum(&digits, 1)
    }
}

pub mod part2 {
    use super::{parse, sum};

    pub fn solve(input: &str) -> usize {
        let digits = parse(input);
        sum(&digits, digits.len() / 2)
    }
}

pub fn main(test: bool) {
    let test_input = "1221".to_owned();
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
