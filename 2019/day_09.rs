//! https://adventofcode.com/2019/day/9
//! https://adventofcode.com/2019/day/9/input

use std::{fs::read_to_string, time::Instant};

use crate::int_code::parse_with_input;

fn generic_solve(input: &str, first_input: i64) -> i64 {
    let mut vm = parse_with_input(input, [first_input].into());
    vm.run_until_complete();
    vm.last_output().unwrap()
}

pub mod part1 {
    use super::generic_solve;

    pub fn solve(input: &str) -> i64 {
        generic_solve(input, 1)
    }
}

pub mod part2 {
    use super::generic_solve;

    pub fn solve(input: &str) -> i64 {
        generic_solve(input, 2)
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2019/day_09_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
