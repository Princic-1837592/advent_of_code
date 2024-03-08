//! https://adventofcode.com/2015/day/25
//! https://adventofcode.com/2015/day/25/input
//! https://adventofcode.com/2015/day/25
//! https://adventofcode.com/2015/day/25/input/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

type Parsed = (usize, usize);

fn parse(input: &str) -> Parsed {
    let parts: Vec<_> = input.split(' ').collect();
    let (row, col) = (parts[parts.len() - 3], parts[parts.len() - 1]);
    (
        row[0..row.len() - 1].parse().unwrap(),
        col[0..row.len() - 1].parse().unwrap(),
    )
}

pub mod part1 {
    use super::Parsed;

    pub fn solve((row, col): Parsed) -> usize {
        let mut code = 20151125;
        let prev_diags = row + col - 2;
        let prev_codes = (prev_diags * (prev_diags + 1)) / 2 + col - 1;
        for _ in 0..prev_codes {
            code = (code * 252533) % 33554393;
        }
        code
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "To continue, please consult the code grid in the manual.  Enter the code at row 5, column 3.".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2015/day_25_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    println!("Parsed in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part1::solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    total
}
