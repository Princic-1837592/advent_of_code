//! https://adventofcode.com/2019/day/4
//! https://adventofcode.com/2019/day/4/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> (usize, usize) {
    let mut parts = input.split('-').map(|part| part.parse().unwrap());
    (parts.next().unwrap(), parts.next().unwrap())
}

pub mod part1 {
    use crate::day_04::parse;

    fn is_valid(password: usize) -> bool {
        let p: Vec<_> = password.to_string().chars().collect();
        let mut pair = false;
        for i in 0..5 {
            if !pair && i < 5 && p[i] == p[i + 1] {
                pair = true;
            }
            if p[i] > p[i + 1] {
                return false;
            }
        }
        pair
    }

    pub fn solve(input: &str) -> usize {
        let (left, right) = parse(input);
        let mut valid = 0;
        for p in left..=right {
            if is_valid(p) {
                valid += 1;
            }
        }
        valid
    }
}

pub mod part2 {
    use crate::day_04::parse;

    fn is_valid(password: usize) -> bool {
        let p: Vec<_> = password.to_string().chars().collect();
        let mut pair = false;
        for i in 0..5 {
            if !pair
                && i < 5
                && p[i] == p[i + 1]
                && (i == 0 || p[i - 1] != p[i])
                && (i == 4 || p[i + 2] != p[i])
            {
                pair = true;
            }
            if p[i] > p[i + 1] {
                return false;
            }
        }
        pair
    }

    pub fn solve(input: &str) -> usize {
        let (left, right) = parse(input);
        let mut valid = 0;
        for p in left..=right {
            if is_valid(p) {
                valid += 1;
            }
        }
        valid
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_04_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
