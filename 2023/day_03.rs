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

type Parsed = (Vec<Number>, Vec<Symbol>);

fn parse(input: &str) -> Parsed {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let number = Regex::new(r"\d+|[^.\d]").unwrap();
    for (i, line) in input.lines().enumerate() {
        for capture in number.find_iter(line) {
            match capture.as_str().chars().next().unwrap() {
                '0'..='9' => {
                    let j = capture.start();
                    numbers.push(Number {
                        value: capture.as_str().parse().unwrap(),
                        top_left: (i.saturating_sub(1), j.saturating_sub(1)),
                        bottom_right: (i + 1, j + capture.len()),
                    })
                }
                symbol => symbols.push(Symbol {
                    char: symbol,
                    coord: (i, capture.start()),
                }),
            }
        }
    }
    (numbers, symbols)
}

pub mod part1 {
    use super::Parsed;

    pub fn solve(_input: &str, (numbers, symbols): Parsed) -> usize {
        numbers
            .iter()
            .filter_map(|n| {
                let first = symbols.binary_search_by_key(&n.top_left, |s| s.coord);
                match first {
                    Ok(_) => Some(n.value),
                    Err(mut s) => {
                        while s < symbols.len() && symbols[s].coord <= n.bottom_right {
                            if n.contains(&symbols[s]) {
                                return Some(n.value);
                            }
                            s += 1;
                        }
                        None
                    }
                }
            })
            .sum()
    }
}

pub mod part2 {
    use crate::day_03::Parsed;

    pub fn solve(_input: &str, (numbers, symbols): Parsed) -> usize {
        symbols
            .iter()
            .filter_map(|s| {
                if s.char == '*' {
                    let ns: Vec<_> = numbers.iter().filter(|n| n.contains(s)).collect();
                    if ns.len() == 2 {
                        Some(ns.iter().map(|n| n.value).product::<usize>())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .sum()
    }
}

pub fn main(test: bool) -> Duration {
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
