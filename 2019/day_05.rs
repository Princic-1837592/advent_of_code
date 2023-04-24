//! https://adventofcode.com/2019/day/7
//! https://adventofcode.com/2019/day/7/input

use std::{fs::read_to_string, time::Instant};

use crate::int_code::parse_with_input;

fn generic_solve(input: &str, first_input: i64) -> i64 {
    let mut vm = parse_with_input(input, [first_input].into());
    vm.run_until_complete();
    vm.last_output().unwrap()
}

pub mod part1 {
    use crate::day_05::generic_solve;

    pub fn solve(input: &str) -> i64 {
        generic_solve(input, 1)
    }
}

pub mod part2 {
    use crate::day_05::generic_solve;

    pub fn solve(input: &str) -> i64 {
        generic_solve(input, 5)
    }
}

pub fn main(test: bool) {
    let test_input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_05_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
