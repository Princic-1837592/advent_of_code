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
    Tgl(usize),
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let parts: Vec<_> = string.split_whitespace().collect();
        match parts[0].chars().next().unwrap() {
            'c' => Instruction::Copy(Val::from(parts[1]), Val::from(parts[2])),
            'i' => Instruction::Inc(parts[1].chars().next().unwrap() as usize - 'a' as usize),
            'd' => Instruction::Dec(parts[1].chars().next().unwrap() as usize - 'a' as usize),
            'j' => Instruction::Jnz(Val::from(parts[1]), Val::from(parts[2])),
            't' => Instruction::Tgl(parts[1].chars().next().unwrap() as usize - 'a' as usize),
            _ => panic!("Invalid instruction: {}", string),
        }
    }
}

impl Instruction {
    fn toggle(self) -> Self {
        match self {
            Instruction::Inc(reg) => Instruction::Dec(reg),
            Instruction::Dec(reg) => Instruction::Inc(reg),
            Instruction::Tgl(reg) => Instruction::Inc(reg),
            Instruction::Jnz(val, offset) => Instruction::Copy(val, offset),
            Instruction::Copy(val, reg) => {
                let result = Instruction::Jnz(val, reg);
                dbg!(result);
                result
            }
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn solve_with(input: &str, a: isize) -> isize {
    let mut instructions = parse(input);
    let mut registers = [0; 4];
    registers[0] = a;
    let mut pc = 0;
    while (pc as usize) < instructions.len() {
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
            Instruction::Tgl(reg) => {
                let target = (pc + registers[reg]) as usize;
                if target < instructions.len() {
                    instructions[target] = instructions[target].toggle();
                }
            }
        }
        pc += 1;
    }
    registers[0]
}

pub mod part1 {
    use super::solve_with;

    pub fn solve(input: &str) -> isize {
        solve_with(input, 7)
    }
}

pub mod part2 {
    use super::solve_with;

    pub fn solve(input: &str) -> isize {
        solve_with(input, 12)
    }
}

pub fn main(test: bool) {
    let test_input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2016/day_23_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
