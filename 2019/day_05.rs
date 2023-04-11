//! https://adventofcode.com/2019/day/7
//! https://adventofcode.com/2019/day/7/input

use std::{collections::VecDeque, fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
enum Mode {
    Position,
    Immediate,
}

#[derive(Copy, Clone, Debug)]
struct Parameter {
    value: isize,
    mode: Mode,
}

impl Parameter {
    fn from(mode: isize, value: isize) -> Self {
        Self {
            mode: match mode {
                0 => Mode::Position,
                1 => Mode::Immediate,
                _ => panic!("Invalid mode: {}", mode),
            },
            value,
        }
    }

    fn get(&self, instructions: &[isize]) -> isize {
        match self.mode {
            Mode::Position => instructions[self.value as usize],
            Mode::Immediate => self.value,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    In(Parameter),
    Out(Parameter),
    Jit(Parameter, Parameter),
    Jif(Parameter, Parameter),
    Lt(Parameter, Parameter, Parameter),
    Eq(Parameter, Parameter, Parameter),
    Halt,
}

fn ith_digit(n: isize, i: u32) -> isize {
    (n / 10_isize.pow(i - 1)) % 10
}

impl Instruction {
    fn parse(instructions: &[isize]) -> (usize, Self) {
        let op_and_params = instructions[0];
        let op = 10 * ith_digit(op_and_params, 2) + ith_digit(op_and_params, 1);
        let (first, second, third) = (
            ith_digit(op_and_params, 3),
            ith_digit(op_and_params, 4),
            ith_digit(op_and_params, 5),
        );
        match op {
            1 => (
                4,
                Self::Add(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                    Parameter::from(third, instructions[3]),
                ),
            ),
            2 => (
                4,
                Self::Mul(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                    Parameter::from(third, instructions[3]),
                ),
            ),
            3 => (2, Self::In(Parameter::from(first, instructions[1]))),
            4 => (2, Self::Out(Parameter::from(first, instructions[1]))),
            5 => (
                0,
                Self::Jit(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                ),
            ),
            6 => (
                0,
                Self::Jif(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                ),
            ),
            7 => (
                4,
                Self::Lt(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                    Parameter::from(third, instructions[3]),
                ),
            ),
            8 => (
                4,
                Self::Eq(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                    Parameter::from(third, instructions[3]),
                ),
            ),
            99 => (1, Self::Halt),
            _ => {
                panic!("Invalid instruction: {}", instructions[0])
            }
        }
    }
}

fn parse(input: &str) -> Vec<isize> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

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
    use crate::day_05::{parse, run};

    pub fn solve(input: &str) -> isize {
        let mut instructions = parse(input);
        run(&mut instructions, [1].into())
    }
}

pub mod part2 {
    use crate::day_05::{parse, run};

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
