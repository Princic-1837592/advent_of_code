//! https://adventofcode.com/2017/day/17
//! https://adventofcode.com/2017/day/17/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> usize {
    input.parse().unwrap()
}

pub mod part1 {
    use super::parse;

    #[derive(Copy, Clone, Debug, Default)]
    struct Node {
        prev: usize,
        next: usize,
    }

    pub fn solve(input: &str) -> usize {
        let step = parse(input);
        let mut values = [Node::default(); 2018];
        let mut current_position = 0;
        for v in 1..=2017 {
            for _ in 0..step {
                current_position = values[current_position].next;
            }
            values[v].prev = current_position;
            values[v].next = values[current_position].next;
            values[values[current_position].next].prev = v;
            values[current_position].next = v;
            current_position = v;
        }
        values[2017].next
    }
}

pub mod part2 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let step = parse(input);
        let mut result = 0;
        let mut current_position = 0;
        for v in 1..=50_000_000 {
            current_position = (current_position + step) % v + 1;
            if current_position == 1 {
                result = v;
            }
        }
        result
    }
}

pub fn main(test: bool) {
    let test_input = "3".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2017/day_17_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
