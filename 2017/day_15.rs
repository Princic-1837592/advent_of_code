//! https://adventofcode.com/2017/day/15
//! https://adventofcode.com/2017/day/15/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> (usize, usize) {
    let mut lines = input
        .lines()
        .map(|line| line.split_whitespace().last().unwrap().parse().unwrap());
    (lines.next().unwrap(), lines.next().unwrap())
}

pub mod part1 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let (mut a, mut b) = parse(input);
        let mut matches = 0;
        for _ in 0..40_000_000 {
            a = (a * 16807) % 2147483647;
            b = (b * 48271) % 2147483647;
            if a as u16 == b as u16 {
                matches += 1;
            }
        }
        matches
    }
}

pub mod part2 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let (mut a, mut b) = parse(input);
        let mut matches = 0;
        for _ in 0..5_000_000 {
            loop {
                a = (a * 16807) % 2147483647;
                if a % 4 == 0 {
                    break;
                }
            }
            loop {
                b = (b * 48271) % 2147483647;
                if b % 8 == 0 {
                    break;
                }
            }
            if a as u16 == b as u16 {
                matches += 1;
            }
        }
        matches
    }
}

pub fn main(test: bool) {
    let test_input = "Generator A starts with 65
Generator B starts with 8921"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2017/day_15_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
