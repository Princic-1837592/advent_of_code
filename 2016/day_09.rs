//! https://adventofcode.com/2016/day/9
//! https://adventofcode.com/2016/day/9/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<char> {
    input
        .split_whitespace()
        .flat_map(|line| line.chars())
        .collect()
}

pub mod part1 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let file = parse(input);
        let mut i = 0;
        let mut decompressed_len = 0;
        while i < file.len() {
            if file[i] == '(' {
                let mut next = 0;
                let mut j = i + 1;
                while file[j] != 'x' {
                    next *= 10;
                    next += file[j].to_digit(10).unwrap() as usize;
                    j += 1;
                }
                j += 1;
                let mut times = 0;
                while file[j] != ')' {
                    times *= 10;
                    times += file[j].to_digit(10).unwrap() as usize;
                    j += 1
                }
                i = j + 1;
                decompressed_len += next.min(file.len().saturating_sub(i)) * times;
                i += next;
            } else {
                decompressed_len += 1;
                i += 1;
            }
        }
        decompressed_len
    }
}

pub mod part2 {
    use super::parse;

    fn decompress(file: &Vec<char>, start: usize, end: usize) -> usize {
        let mut decompressed_len = 0;
        let mut i = start;
        while i < end {
            if file[i] == '(' {
                let mut next = 0;
                let mut j = i + 1;
                while file[j] != 'x' {
                    next *= 10;
                    next += file[j].to_digit(10).unwrap() as usize;
                    j += 1;
                }
                j += 1;
                let mut times = 0;
                while file[j] != ')' {
                    times *= 10;
                    times += file[j].to_digit(10).unwrap() as usize;
                    j += 1
                }
                i = j + 1;
                decompressed_len += decompress(file, i, i + next) * times;
                i += next;
            } else {
                decompressed_len += 1;
                i += 1;
            }
        }
        decompressed_len
    }

    pub fn solve(input: &str) -> usize {
        let file = parse(input);
        decompress(&file, 0, file.len())
    }
}

pub fn main(test: bool) {
    let test_input = "X(8x2)(3x3)ABCY".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2016/day_09_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
