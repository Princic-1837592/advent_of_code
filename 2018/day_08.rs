//! https://adventofcode.com/2018/day/8
//! https://adventofcode.com/2018/day/8/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

pub mod part1 {
    use super::parse;

    fn count_metadata(numbers: &Vec<usize>, mut i: usize) -> (usize, usize) {
        if i >= numbers.len() {
            (i, 0)
        } else {
            let mut total = 0;
            let children = numbers[i];
            i += 1;
            let md_count = numbers[i];
            i += 1;
            for _ in 0..children {
                let child = count_metadata(numbers, i);
                i = child.0;
                total += child.1;
            }
            for _ in 0..md_count {
                total += numbers[i];
                i += 1;
            }
            (i, total)
        }
    }

    pub fn solve(input: &str) -> usize {
        let numbers = parse(input);
        count_metadata(&numbers, 0).1
    }
}

pub mod part2 {
    use super::parse;

    fn value_of(numbers: &Vec<usize>, mut i: usize) -> (usize, usize) {
        if i >= numbers.len() {
            (i, 0)
        } else {
            let mut children = vec![0; numbers[i]];
            i += 1;
            let mut mds = vec![0; numbers[i]];
            i += 1;
            for child in children.iter_mut() {
                let result = value_of(numbers, i);
                i = result.0;
                *child = result.1;
            }
            for md in mds.iter_mut() {
                *md = numbers[i];
                i += 1;
            }
            let result = (
                i,
                if !children.is_empty() {
                    let value = mds
                        .iter()
                        .map(|child| children.get(*child - 1).unwrap_or(&0))
                        .sum();
                    value
                } else {
                    let value = mds.iter().sum();
                    value
                },
            );
            result
        }
    }

    pub fn solve(input: &str) -> usize {
        let numbers = parse(input);
        value_of(&numbers, 0).1
    }
}

pub fn main(test: bool) {
    let test_input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_08_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
