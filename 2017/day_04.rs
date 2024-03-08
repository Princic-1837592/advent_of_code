//! https://adventofcode.com/2017/day/4
//! https://adventofcode.com/2017/day/4/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect()
}

pub mod part1 {
    use std::collections::HashSet;

    use super::parse;

    pub fn solve(input: &str) -> usize {
        let passphrases = parse(input);
        passphrases
            .iter()
            .filter(|passphrase| {
                passphrase.iter().collect::<HashSet<_>>().len() == passphrase.len()
            })
            .count()
    }
}

pub mod part2 {
    use std::collections::HashSet;

    use super::parse;

    pub fn solve(input: &str) -> usize {
        let passphrases = parse(input);
        passphrases
            .iter()
            .filter(|passphrase| {
                passphrase
                    .iter()
                    .map(|word| {
                        let mut chars: Vec<_> = word.chars().collect();
                        chars.sort();
                        chars
                    })
                    .collect::<HashSet<_>>()
                    .len()
                    == passphrase.len()
            })
            .count()
    }
}

pub fn main(test: bool) {
    let test_input = "a ab abc abd abf abj".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2017/day_04_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
