//! https://adventofcode.com/2016/day/25
//! https://adventofcode.com/2016/day/25/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
enum Val {
    Lit(isize),
    Reg(usize),
}

impl Val {
    fn evaluate(&self, registers: &[isize; 4]) -> isize {
        match *self {
            Val::Lit(lit) => lit,
            Val::Reg(reg) => registers[reg],
        }
    }
}

impl From<&str> for Val {
    fn from(string: &str) -> Self {
        if let Ok(offset) = string.parse() {
            Val::Lit(offset)
        } else {
            Val::Reg(string.chars().next().unwrap() as usize - 'a' as usize)
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Copy(Val, Val),
    Inc(usize),
    Dec(usize),
    Jnz(Val, Val),
    Out(Val),
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let parts: Vec<_> = string.split_whitespace().collect();
        match parts[0].chars().next().unwrap() {
            'c' => Instruction::Copy(Val::from(parts[1]), Val::from(parts[2])),
            'i' => Instruction::Inc(parts[1].chars().next().unwrap() as usize - 'a' as usize),
            'd' => Instruction::Dec(parts[1].chars().next().unwrap() as usize - 'a' as usize),
            'j' => Instruction::Jnz(Val::from(parts[1]), Val::from(parts[2])),
            'o' => Instruction::Out(Val::from(parts[1])),
            _ => panic!("Invalid instruction: {}", string),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn solve_with(instructions: &Vec<Instruction>, a: isize) -> bool {
    let mut registers = [0; 4];
    registers[0] = a;
    let mut expected = 0;
    let mut matched = 0;
    let mut pc = 0;
    while (pc as usize) < instructions.len() && matched < 1000 {
        match instructions[pc as usize] {
            Instruction::Copy(val, reg) => {
                if let Val::Reg(reg) = reg {
                    registers[reg] = val.evaluate(&registers);
                }
            }
            Instruction::Inc(reg) => registers[reg] += 1,
            Instruction::Dec(reg) => registers[reg] -= 1,
            Instruction::Jnz(val, offset) => {
                if val.evaluate(&registers) != 0 {
                    pc = pc + offset.evaluate(&registers) - 1
                }
            }
            Instruction::Out(val) => {
                let val = val.evaluate(&registers);
                if val == expected {
                    expected = 1 - expected;
                    matched += 1;
                } else {
                    return false;
                }
            }
        }
        pc += 1;
    }
    true
}

pub mod part1 {
    use super::{parse, solve_with};

    pub fn solve(input: &str) -> isize {
        let instructions = parse(input);
        for i in 0.. {
            if solve_with(&instructions, i) {
                return i;
            }
        }
        unreachable!()
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_25_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
