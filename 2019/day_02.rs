//! https://adventofcode.com/2019/day/2
//! https://adventofcode.com/2019/day/2/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<usize> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

pub mod part1 {
    use crate::day_02::parse;

    pub fn solve(input: &str) -> usize {
        let mut program = parse(input);
        program[1] = 12;
        program[2] = 2;
        let mut pc = 0;
        while pc < program.len() {
            match program[pc] {
                1 => {
                    let (left, right, dest) = (program[pc + 1], program[pc + 2], program[pc + 3]);
                    program[dest] = program[left] + program[right];
                }
                2 => {
                    let (left, right, dest) = (program[pc + 1], program[pc + 2], program[pc + 3]);
                    program[dest] = program[left] * program[right];
                }
                99 => break,
                invalid => panic!("Invalid opcode: {}", invalid),
            }
            pc += 4;
        }
        program[0]
    }
}

pub mod part2 {
    use itertools::Itertools;

    use crate::day_02::parse;

    pub fn solve(input: &str) -> usize {
        let program = parse(input);
        for (noun, verb) in (0..=99).cartesian_product(0..=99) {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;
            let mut pc = 0;
            while pc < program.len() {
                match program[pc] {
                    1 => {
                        let (left, right, dest) =
                            (program[pc + 1], program[pc + 2], program[pc + 3]);
                        program[dest] = program[left] + program[right];
                    }
                    2 => {
                        let (left, right, dest) =
                            (program[pc + 1], program[pc + 2], program[pc + 3]);
                        program[dest] = program[left] * program[right];
                    }
                    99 => break,
                    invalid => panic!("Invalid opcode: {}", invalid),
                }
                pc += 4;
            }
            if program[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
        unreachable!()
    }
}

pub fn main(test: bool) {
    let test_input = "1,9,10,3,2,3,11,0,99,30,40,50".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_02_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
