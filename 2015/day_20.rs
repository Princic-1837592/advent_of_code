//! https://adventofcode.com/2015/day/20
//! https://adventofcode.com/2015/day/20/input

use std::{fs::read_to_string, time::Instant};

const MAX: usize = 1_000_000;

pub mod part1 {
    use crate::day_20::MAX;

    pub fn solve(input: &str) -> usize {
        let target = input.parse().unwrap();
        let mut houses = vec![0; MAX];
        for i in 1..=MAX {
            let addition = i * 10;
            let mut j = i;
            while j < MAX {
                houses[j] += addition;
                j += i;
            }
        }
        houses
            .iter()
            .enumerate()
            .find(|(_, &presents)| presents >= target)
            .unwrap()
            .0
    }
}

pub mod part2 {
    use crate::day_20::MAX;

    pub fn solve(input: &str) -> usize {
        let target = input.parse().unwrap();
        let mut houses = vec![0; MAX];
        for i in 1..=MAX {
            let addition = i * 11;
            let mut j = i;
            let mut delivered = 0;
            while j < MAX && delivered < 50 {
                houses[j] += addition;
                j += i;
                delivered += 1;
            }
        }
        houses
            .iter()
            .enumerate()
            .find(|(_, &presents)| presents >= target)
            .unwrap()
            .0
    }
}

pub fn main(test: bool) {
    let test_input = "29000000".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_20_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
