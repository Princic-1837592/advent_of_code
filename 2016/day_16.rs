//! https://adventofcode.com/2016/day/16
//! https://adventofcode.com/2016/day/16/input

use std::{fs::read_to_string, time::Instant};

fn find_checksum(mut disc: Vec<u8>, disc_length: usize) -> String {
    while disc.len() < disc_length {
        let b: Vec<_> = disc.iter().rev().map(|b| 1 - b).collect();
        disc.push(0);
        disc.extend(b);
    }
    disc.truncate(disc_length);
    let mut checksum = disc;
    while checksum.len() % 2 == 0 || checksum.len() == disc_length {
        for i in (0..checksum.len()).step_by(2) {
            checksum[i / 2] = if checksum[i] == checksum[i + 1] { 1 } else { 0 };
        }
        checksum.truncate(checksum.len() / 2);
    }
    checksum.iter().map(ToString::to_string).collect()
}

pub mod part1 {
    use crate::day_16::find_checksum;

    pub fn solve(input: &str, disc_length: usize) -> String {
        find_checksum(
            input
                .chars()
                .map(|char| if char == '0' { 0 } else { 1 })
                .collect(),
            disc_length,
        )
    }
}

pub mod part2 {
    use crate::day_16::find_checksum;

    pub fn solve(input: &str) -> String {
        find_checksum(
            input
                .chars()
                .map(|char| if char == '0' { 0 } else { 1 })
                .collect(),
            35651584,
        )
    }
}

pub fn main(test: bool) {
    let test_input = "10000".to_owned();
    let (puzzle_input, disc_length) = if test {
        (test_input, 20)
    } else {
        (read_to_string("inputs/day_16_input.txt").unwrap(), 272)
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input, disc_length));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
