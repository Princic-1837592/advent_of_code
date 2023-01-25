//! https://adventofcode.com/2015/day/25
//! https://adventofcode.com/2015/day/25/input
//! https://adventofcode.com/2015/day/25
//! https://adventofcode.com/2015/day/25/input/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> (usize, usize) {
    let parts: Vec<_> = input.split(' ').collect();
    let (row, col) = (parts[parts.len() - 3], parts[parts.len() - 1]);
    (
        row[0..row.len() - 1].parse().unwrap(),
        col[0..row.len() - 1].parse().unwrap(),
    )
}

pub mod part1 {
    use crate::day_25::parse;

    pub fn solve(input: &str) -> usize {
        let (row, col) = parse(input);
        let mut code = 20151125;
        let prev_diags = row + col - 2;
        let prev_codes = (prev_diags * (prev_diags + 1)) / 2 + col - 1;
        for _ in 0..prev_codes {
            code = (code * 252533) % 33554393;
        }
        code
    }
}

pub fn main(test: bool) {
    let test_input = "To continue, please consult the code grid in the manual.  Enter the code at row 5, column 3.".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_25_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
