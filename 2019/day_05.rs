//! https://adventofcode.com/2019/day/7
//! https://adventofcode.com/2019/day/7/input

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use crate::int_code::{parse, run, Interrupt};

    pub fn solve(input: &str) -> isize {
        let mut instructions = parse(input);
        let mut last_output = 0;
        loop {
            match run(instructions.clone(), [1].into()) {
                Interrupt::Output(new_instructions, _, output) => {
                    instructions = new_instructions;
                    last_output = output;
                }
                Interrupt::Halt(_, _) => break,
                _ => {}
            }
        }
        last_output
    }
}

pub mod part2 {
    use crate::int_code::{parse, run, Interrupt};

    pub fn solve(input: &str) -> isize {
        let mut instructions = parse(input);
        if let Interrupt::Output(_, _, result) = run(instructions, [1].into()) {
            return result;
        }
        unreachable!()
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
