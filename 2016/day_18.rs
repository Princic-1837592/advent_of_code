//! https://adventofcode.com/2016/day/18
//! https://adventofcode.com/2016/day/18/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<bool> {
    input.chars().map(|char| char == '.').collect()
}

fn make_rows(mut row: Vec<bool>, rows: usize) -> usize {
    let mut safe_tiles = 0;
    for _ in 0..rows {
        safe_tiles += row.iter().filter(|&&safe| safe).count();
        let mut new_row = row.clone();
        for (i, tile) in new_row.iter_mut().enumerate() {
            let left = if i == 0 { true } else { row[i - 1] };
            let mid = row[i];
            let right = if i == row.len() - 1 { true } else { row[i + 1] };
            *tile = !matches!(
                (left, mid, right),
                (false, false, true)
                    | (true, false, false)
                    | (false, true, true)
                    | (true, true, false)
            );
        }
        row = new_row;
    }
    safe_tiles
}

pub mod part1 {
    use super::{make_rows, parse};

    pub fn solve(input: &str, rows: usize) -> usize {
        make_rows(parse(input), rows)
    }
}

pub mod part2 {
    use super::{make_rows, parse};

    pub fn solve(input: &str) -> usize {
        make_rows(parse(input), 400_000)
    }
}

pub fn main(test: bool) {
    let test_input = ".^^.^.^^^^".to_owned();
    let (puzzle_input, rows) = if test {
        (test_input, 10)
    } else {
        (read_to_string("inputs/day_18_input.txt").unwrap(), 40)
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input, rows));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
