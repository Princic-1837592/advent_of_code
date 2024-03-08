//! https://adventofcode.com/2015/day/20
//! https://adventofcode.com/2015/day/20/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

const MAX: usize = 1_000_000;

pub mod part1 {
    use super::MAX;

    pub fn solve(input: &str) -> usize {
        let target = input.parse().unwrap();
        let mut houses = vec![0; MAX];
        for elf in 1..=MAX {
            let addition = elf * 10;
            let mut house = elf;
            while house < MAX {
                houses[house] += addition;
                house += elf;
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
    use super::MAX;

    pub fn solve(input: &str) -> usize {
        let target = input.parse().unwrap();
        let mut houses = vec![0; MAX];
        for elf in 1..=MAX {
            let addition = elf * 11;
            let mut house = elf;
            let mut delivered = 0;
            while house < MAX && delivered < 50 {
                houses[house] += addition;
                house += elf;
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

pub fn main(test: bool) -> Duration {
    let test_input = "29000000".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2015/day_20_input.txt").unwrap()
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
