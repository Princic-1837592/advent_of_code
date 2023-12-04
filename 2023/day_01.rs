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

fn find_both(line: &str, values: Vec<&str>) -> (usize, usize) {
    (
        values
            .iter()
            .enumerate()
            .min_by_key(|&(_, v)| line.find(v).unwrap_or(usize::MAX))
            .unwrap()
            .0,
        values
            .iter()
            .enumerate()
            .max_by_key(|&(_, v)| line.rfind(v).map(|u| u as isize).unwrap_or(isize::MIN))
            .unwrap()
            .0,
    )
}

pub mod part1 {
    use super::{find_both, Parsed};

    pub fn solve(_input: &str, lines: Parsed) -> usize {
        lines
            .iter()
            .map(|l| {
                let (first, last) =
                    find_both(l, vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]);
                first * 10 + last
            })
            .sum()
    }
}

pub mod part2 {
    use super::{find_both, Parsed};

    pub fn solve(_input: &str, lines: Parsed) -> usize {
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

pub fn main(test: bool) -> Duration {
    let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_01_input.txt").unwrap()
    };

    let parsed = parse(&puzzle_input);
    let mut total = Duration::default();

    let start = Instant::now();
    let result = part1::solve(&puzzle_input, parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Run in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(&puzzle_input, parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Run in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
