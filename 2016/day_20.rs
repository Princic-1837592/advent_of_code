//! https://adventofcode.com/2016/day/20
//! https://adventofcode.com/2016/day/20/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Range {
    left: usize,
    right: usize,
}

impl From<&str> for Range {
    fn from(string: &str) -> Self {
        let mut parts = string.split('-').map(|n| n.parse().unwrap());
        Range {
            left: parts.next().unwrap(),
            right: parts.next().unwrap(),
        }
    }
}

fn parse(input: &str) -> Vec<Range> {
    let mut result: Vec<_> = input.lines().map(Range::from).collect();
    result.sort();
    result
}

pub mod part1 {
    use crate::day_20::parse;

    pub fn solve(input: &str) -> usize {
        let ranges = parse(input);
        if ranges[0].left > 0 {
            return 0;
        }
        let mut right = ranges[0].right;
        let mut i = 1;
        while i < ranges.len() {
            if ranges[i].left <= right + 1 {
                right = right.max(ranges[i].right);
                i += 1
            } else {
                return right + 1;
            }
        }
        ranges[ranges.len() - 1].right + 1
    }
}

pub mod part2 {
    use crate::day_20::parse;

    pub fn solve(input: &str) -> usize {
        let ranges = parse(input);
        if ranges[0].left > 0 {
            return 0;
        }
        let mut allowed = 0;
        let mut right = ranges[0].right;
        let mut i = 1;
        while i < ranges.len() {
            if ranges[i].left <= right + 1 {
                right = right.max(ranges[i].right);
                i += 1
            } else {
                allowed += ranges[i].left - right - 1;
                right = ranges[i].right;
            }
        }
        allowed
    }
}

pub fn main(test: bool) {
    let test_input = "5-8
0-2
4-7"
    .to_owned();
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
