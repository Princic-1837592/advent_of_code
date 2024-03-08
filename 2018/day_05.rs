//! https://adventofcode.com/2018/day/5
//! https://adventofcode.com/2018/day/5/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<i8> {
    input.chars().map(|char| char as i8).collect()
}

fn solve_ignoring(bytes: &Vec<i8>, ignore_upper: i8) -> usize {
    let mut deleted = vec![false; bytes.len()];
    let mut count = bytes.len();
    loop {
        let result = bytes
            .iter()
            .enumerate()
            .fold((count, (0, ' ' as i8)), |(len, (j, prev)), (i, &char)| {
                if deleted[i] {
                    (len, (j, prev))
                } else if char == ignore_upper || char == ignore_upper + 32 {
                    deleted[i] = true;
                    (len - 1, (j, prev))
                } else if (char - prev).abs() == 32 {
                    deleted[i] = true;
                    deleted[j] = true;
                    (len - 2, (0, ' ' as i8))
                } else {
                    (len, (i, char))
                }
            })
            .0;
        if result == count {
            return count;
        }
        count = result;
    }
}

pub mod part1 {
    use super::{parse, solve_ignoring};

    pub fn solve(input: &str) -> usize {
        let bytes = parse(input);
        solve_ignoring(&bytes, ' ' as i8)
    }
}

pub mod part2 {
    use super::{parse, solve_ignoring};

    pub fn solve(input: &str) -> usize {
        let bytes = parse(input);
        ('A' as i8..='Z' as i8)
            .map(|ignored| solve_ignoring(&bytes, ignored))
            .min()
            .unwrap()
    }
}

pub fn main(test: bool) {
    let test_input = "dabAcCaCBAcCcaDA".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2018/day_05_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
