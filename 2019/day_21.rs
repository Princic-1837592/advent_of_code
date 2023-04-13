//! https://adventofcode.com/2019/day/21
//! https://adventofcode.com/2019/day/21/input

use std::{fs::read_to_string, time::Instant};

use crate::int_code::parse_with_input;

fn solve_with_program(input: &str, program: &str) -> usize {
    let inputs: Vec<_> = program
        .lines()
        .into_iter()
        .flat_map(|line| {
            let mut line = line.to_owned();
            line.push('\n');
            line.chars().map(|char| char as isize).collect::<Vec<_>>()
        })
        .collect();
    let mut spring_droid = parse_with_input(input, inputs.into());
    spring_droid.run_until_complete();
    spring_droid.last_output().unwrap() as usize
}

pub mod part1 {
    use crate::day_21::solve_with_program;

    pub fn solve(input: &str) -> usize {
        let program = r"OR C T
AND A T
NOT T J
AND D J
WALK";
        solve_with_program(input, program)
    }
}

pub mod part2 {
    use crate::day_21::solve_with_program;

    pub fn solve(input: &str) -> usize {
        let program = r"NOT B J
NOT C T
OR T J
AND D J
AND H J
NOT A T
OR T J
RUN";
        solve_with_program(input, program)
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_21_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
