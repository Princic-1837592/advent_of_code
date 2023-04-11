//! https://adventofcode.com/2019/day/9
//! https://adventofcode.com/2019/day/9/input

use std::{fs::read_to_string, time::Instant};

use crate::int_code::{parse, IntCode};

fn generic_solve(input: &str, first_input: isize) -> isize {
    let mut vm = IntCode::with_input(parse(input), [first_input].into());
    vm.run_until_complete();
    vm.last_output().unwrap()
}

pub mod part1 {
    use crate::day_09::generic_solve;

    pub fn solve(input: &str) -> isize {
        generic_solve(input, 1)
    }
}

pub mod part2 {
    use crate::day_09::generic_solve;

    pub fn solve(input: &str) -> isize {
        generic_solve(input, 2)
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_09_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
