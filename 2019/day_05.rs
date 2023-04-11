//! https://adventofcode.com/2019/day/7
//! https://adventofcode.com/2019/day/7/input

use std::{collections::VecDeque, fs::read_to_string, time::Instant};

use crate::int_code::{Instruction, Mode, Parameter};

fn run(instructions: &mut Vec<isize>, mut input_queue: VecDeque<isize>) -> isize {
    let mut pc = 0;
    let mut last_output = 0;
    while pc < instructions.len() {
        let (consumed, instruction) = Instruction::parse(&instructions[pc..]);
        match instruction {
            Instruction::Add(l, r, dest) => {
                let (l, r) = (l.get(instructions), r.get(instructions));
                if let Parameter {
                    value,
                    mode: Mode::Position,
                } = dest
                {
                    instructions[value as usize] = l + r;
                } else {
                    panic!("Invalid mode for writing: {:?}", dest.mode)
                }
            }
            Instruction::Mul(l, r, dest) => {
                let (l, r) = (l.get(instructions), r.get(instructions));
                if let Parameter {
                    value,
                    mode: Mode::Position,
                } = dest
                {
                    instructions[value as usize] = l * r;
                } else {
                    panic!("Invalid mode for writing: {:?}", dest.mode)
                }
            }
            Instruction::In(dest) => {
                if let Parameter {
                    value,
                    mode: Mode::Position,
                } = dest
                {
                    instructions[value as usize] = input_queue
                        .pop_front()
                        .expect("Expected input but queue is empty");
                } else {
                    panic!("Invalid mode for writing: {:?}", dest.mode)
                }
            }
            Instruction::Out(value) => {
                let value = value.get(instructions);
                last_output = value;
            }
            Instruction::Jit(cond, dest) => {
                let (cond, dest) = (cond.get(instructions), dest.get(instructions));
                if cond != 0 {
                    pc = dest as usize;
                } else {
                    pc += 3;
                }
            }
            Instruction::Jif(cond, dest) => {
                let (cond, dest) = (cond.get(instructions), dest.get(instructions));
                if cond == 0 {
                    pc = dest as usize;
                } else {
                    pc += 3;
                }
            }
            Instruction::Lt(l, r, dest) => {
                let (l, r) = (l.get(instructions), r.get(instructions));
                if let Parameter {
                    value,
                    mode: Mode::Position,
                } = dest
                {
                    instructions[value as usize] = if l < r { 1 } else { 0 };
                } else {
                    panic!("Invalid mode for writing: {:?}", dest.mode)
                }
            }
            Instruction::Eq(l, r, dest) => {
                let (l, r) = (l.get(instructions), r.get(instructions));
                if let Parameter {
                    value,
                    mode: Mode::Position,
                } = dest
                {
                    instructions[value as usize] = if l == r { 1 } else { 0 };
                } else {
                    panic!("Invalid mode for writing: {:?}", dest.mode)
                }
            }
            Instruction::Halt => break,
        }
        pc += consumed;
    }
    last_output
}

pub mod part1 {
    use crate::{day_05::run, int_code::parse};

    pub fn solve(input: &str) -> isize {
        let mut instructions = parse(input);
        run(&mut instructions, [1].into())
    }
}

pub mod part2 {
    use crate::{day_05::run, int_code::parse};

    pub fn solve(input: &str) -> isize {
        let mut instructions = parse(input);
        run(&mut instructions, [5].into())
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
