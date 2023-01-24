//! https://adventofcode.com/2015/day/2

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.split('x').map(|n| n.parse().unwrap());
            (
                numbers.next().unwrap(),
                numbers.next().unwrap(),
                numbers.next().unwrap(),
            )
        })
        .collect()
}

fn find_wrapping((l, w, h): &(usize, usize, usize)) -> usize {
    2 * l * w + 2 * w * h + 2 * h * l + (l * w).min((w * h).min(h * l))
}

fn find_ribbon((l, w, h): &(usize, usize, usize)) -> usize {
    let total = l + w + h;
    let perimeter = total - l.max(w.max(h));
    perimeter * 2 + l * w * h
}

pub mod part1 {
    use crate::day_02::{find_wrapping, parse};

    pub fn solve(input: &str) -> usize {
        parse(input).iter().map(find_wrapping).sum()
    }
}

pub mod part2 {
    use crate::day_02::{find_ribbon, parse};

    pub fn solve(input: &str) -> usize {
        parse(input).iter().map(find_ribbon).sum()
    }
}

pub fn main(test: bool) {
    let test_input = "2x3x4".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_02_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
