//! https://adventofcode.com/2023/day/3
//! https://adventofcode.com/2023/day/3/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

use regex::Regex;

type Coord = (usize, usize);

#[derive(Clone, Debug)]
pub struct Number {
    value: usize,
    top_left: Coord,
    bottom_right: Coord,
}

#[derive(Copy, Clone, Debug)]
pub struct Symbol {
    char: char,
    coord: Coord,
}

impl Number {
    fn contains(&self, symbol: &Symbol) -> bool {
        (self.top_left.0..=self.bottom_right.0).contains(&symbol.coord.0)
            && (self.top_left.1..=self.bottom_right.1).contains(&symbol.coord.1)
    }
}

type Parsed = Vec<Vec<char>>;

fn parse(input: &str) -> Parsed {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_numbers_near(lines: [Option<&Vec<char>>; 3], i: usize, j: usize) -> Vec<usize> {
    let mut used = [[false; 3]; 3];
    unimplemented!()
}

fn expand_number(line: &Vec<char>, j: usize) -> (usize, usize, usize) {
    let mut result = line[j].to_digit(10).unwrap();
}

pub mod part1 {
    use super::{find_numbers_near, Parsed};

    pub fn solve(chars: Parsed) -> usize {
        let mut result = 0;
        for (i, line) in chars.iter().enumerate() {
            for (j, &char) in line.iter().enumerate() {
                if char != '.' && !char.is_ascii_digit() {
                    result +=
                        find_numbers_near([chars.get(i - 1), chars.get(i), chars.get(i + 1)], i, j)
                            .into_iter()
                            .sum::<usize>();
                }
            }
        }
        result
    }
}

pub mod part2 {
    use super::{find_numbers_near, Parsed};

    pub fn solve(chars: Parsed) -> usize {
        let mut result = 0;
        for (i, line) in chars.iter().enumerate() {
            for (j, &char) in line.iter().enumerate() {
                if char != '.' && !char.is_ascii_digit() {
                    let near =
                        find_numbers_near([chars.get(i - 1), chars.get(i), chars.get(i + 1)], i, j);
                    if near.len() == 2 {
                        result += near.into_iter().sum::<usize>();
                    }
                }
            }
        }
        result
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_03_input.txt").unwrap()
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
