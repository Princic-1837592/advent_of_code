//! https://adventofcode.com/2016/day/15
//! https://adventofcode.com/2016/day/15/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
struct Disc {
    positions: usize,
    begin: usize,
}

impl From<&str> for Disc {
    fn from(string: &str) -> Self {
        let mut parts = string.split_whitespace();
        Disc {
            positions: parts.nth(3).unwrap().parse().unwrap(),
            begin: parts.nth(7).unwrap().trim_end_matches('.').parse().unwrap(),
        }
    }
}

fn parse(input: &str) -> Vec<Disc> {
    input.lines().map(Disc::from).collect()
}

fn find_t(discs: Vec<Disc>) -> usize {
    for t in 0.. {
        let mut correct = true;
        for (i, disc) in discs.iter().enumerate() {
            if (disc.begin + t + i + 1) % disc.positions != 0 {
                correct = false;
                break;
            }
        }
        if correct {
            return t;
        }
    }
    unreachable!()
}

pub mod part1 {
    use crate::day_15::{find_t, parse};

    pub fn solve(input: &str) -> usize {
        let discs = parse(input);
        find_t(discs)
    }
}

pub mod part2 {
    use crate::day_15::{find_t, parse, Disc};

    pub fn solve(input: &str) -> usize {
        let mut discs = parse(input);
        discs.push(Disc {
            positions: 11,
            begin: 0,
        });
        find_t(discs)
    }
}

pub fn main(test: bool) {
    let test_input = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_15_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
