//! https://adventofcode.com/2017/day/13
//! https://adventofcode.com/2017/day/13/input

use std::{collections::HashMap, fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug, Default)]
struct Scanner {
    range: usize,
    round_size: usize,
}

fn parse(input: &str) -> (HashMap<usize, Scanner>, usize) {
    let result: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let (i, range) = (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            );
            (
                i,
                Scanner {
                    range,
                    round_size: (range - 1) * 2,
                },
            )
        })
        .collect();
    let max = *result.keys().max().unwrap();
    (result, max)
}

pub mod part1 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let (scanners, max) = parse(input);
        let mut severity = 0;
        for layer @ ps in 0..=max {
            if let Some(scanner) = scanners.get(&layer) {
                if ps % scanner.round_size == 0 {
                    severity += layer * scanner.range;
                }
            }
        }
        severity
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use super::{parse, Scanner};

    fn caught(scanners: &HashMap<usize, Scanner>, max: usize, delay: usize) -> bool {
        for (layer, ps) in (delay..=delay + max).enumerate() {
            if let Some(scanner) = scanners.get(&layer) {
                if ps % scanner.round_size == 0 {
                    return true;
                }
            }
        }
        false
    }

    pub fn solve(input: &str) -> usize {
        let (scanners, max) = parse(input);
        (0..).find(|&delay| !caught(&scanners, max, delay)).unwrap()
    }
}

pub fn main(test: bool) {
    let test_input = "0: 3
1: 2
4: 4
6: 4"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_13_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
