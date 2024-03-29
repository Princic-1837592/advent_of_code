//! https://adventofcode.com/2023/day/1
//! https://adventofcode.com/2023/day/1/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

type Parsed<'a> = Vec<&'a str>;

fn parse(input: &str) -> Parsed {
    input.lines().collect()
}

pub mod part1 {
    use super::Parsed;

    pub fn solve(lines: Parsed) -> usize {
        lines
            .iter()
            .map(|l| {
                let mut first = 0;
                for char in l.chars() {
                    if char.is_ascii_digit() {
                        first = char.to_digit(10).unwrap() as usize;
                        break;
                    }
                }
                let mut last = 0;
                for char in l.chars().rev() {
                    if char.is_ascii_digit() {
                        last = char.to_digit(10).unwrap() as usize;
                        break;
                    }
                }
                first * 10 + last
            })
            .sum()
    }
}

pub mod part2 {
    use super::Parsed;

    fn find_one(line: &str, values: &[&str]) -> usize {
        for i in 0..line.len() {
            for (v, value) in values.iter().enumerate() {
                if line[i..].starts_with(value) {
                    return v;
                }
            }
        }
        unreachable!()
    }

    fn find_one_reversed(line: &str, values: &[&str]) -> usize {
        for i in (0..=line.len()).rev() {
            for (v, value) in values.iter().enumerate() {
                if line[..i].ends_with(value) {
                    return v;
                }
            }
        }
        unreachable!()
    }

    fn find_both(line: &str, values: Vec<&str>) -> (usize, usize) {
        (find_one(line, &values), find_one_reversed(line, &values))
    }

    pub fn solve(lines: Parsed) -> usize {
        lines
            .iter()
            .map(|l| {
                let (first, last) = find_both(
                    l,
                    vec![
                        "0", "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6",
                        "six", "7", "seven", "8", "eight", "9", "nine",
                    ],
                );
                ((first + 1) / 2) * 10 + ((last + 1) / 2)
            })
            .sum()
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_01_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    if verbose {
        println!("Parsed in {:?}", elapsed);
        total += elapsed;
    }

    let start = Instant::now();
    let result = part1::solve(parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    if verbose {
        println!("Total {:?}", total);
    }
    total
}
