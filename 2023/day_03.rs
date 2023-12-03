//! https://adventofcode.com/2023/day/3
//! https://adventofcode.com/2023/day/3/input

use std::{fs::read_to_string, time::Instant};

use regex::Regex;

type Coord = (usize, usize);

#[derive(Clone, Debug)]
struct Number {
    value: usize,
    top_left: Coord,
    bottom_right: Coord,
}

#[derive(Copy, Clone, Debug)]
struct Symbol {
    char: char,
    coord: Coord,
}

impl Number {
    fn contains(&self, symbol: &Symbol) -> bool {
        (self.top_left.0..=self.bottom_right.0).contains(&symbol.coord.0)
            && (self.top_left.1..=self.bottom_right.1).contains(&symbol.coord.1)
    }
}

fn parse(input: &str) -> (Vec<Number>, Vec<Symbol>) {
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
    use crate::day_03::parse;

    pub fn solve(input: &str) -> usize {
        let (numbers, symbols) = parse(input);
        numbers
            .iter()
            .filter_map(|n| symbols.iter().any(|s| n.contains(s)).then_some(n.value))
            .sum()
    }
}

pub mod part2 {
    use crate::day_03::parse;

    pub fn solve(input: &str) -> usize {
        let (numbers, symbols) = parse(input);
        symbols
            .iter()
            .filter_map(|s| {
                if s.char == '*' {
                    let ns: Vec<_> = numbers.iter().filter(|n| n.contains(&s)).collect();
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

pub fn main(test: bool) {
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
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
