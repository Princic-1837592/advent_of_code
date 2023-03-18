//! https://adventofcode.com/2016/day/12
//! https://adventofcode.com/2016/day/12/input

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
        if string.chars().next().unwrap().is_ascii_digit() {
            Val::Lit(string.parse().unwrap())
        } else {
            Val::Reg(string.chars().next().unwrap() as usize - 'a' as usize)
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Copy(Val, usize),
    Inc(usize),
    Dec(usize),
    Jnz(Val, isize),
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let parts: Vec<_> = string.split_whitespace().collect();
        match parts[0].chars().next().unwrap() {
            'c' => Instruction::Copy(
                Val::from(parts[1]),
                parts[2].chars().next().unwrap() as usize - 'a' as usize,
            ),
            'i' => Instruction::Inc(parts[1].chars().next().unwrap() as usize - 'a' as usize),
            'd' => Instruction::Dec(parts[1].chars().next().unwrap() as usize - 'a' as usize),
            'j' => Instruction::Jnz(Val::from(parts[1]), parts[2].parse().unwrap()),
            _ => panic!("Invalid instruction: {}", string),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn run(instructions: Vec<Instruction>, registers: &mut [isize; 4]) {
    let mut pc = 0;
    while (pc as usize) < instructions.len() {
        match instructions[pc as usize] {
            Instruction::Copy(val, reg) => registers[reg] = val.evaluate(registers),
            Instruction::Inc(reg) => registers[reg] += 1,
            Instruction::Dec(reg) => registers[reg] -= 1,
            Instruction::Jnz(val, offset) => {
                if val.evaluate(registers) != 0 {
                    pc = pc + offset - 1
                }
            }
        }
        pc += 1;
    }
}

pub mod part1 {
    use crate::day_12::{parse, run};

    pub fn solve(input: &str) -> isize {
        let instructions = parse(input);
        let mut registers = [0; 4];
        run(instructions, &mut registers);
        registers[0]
    }
}

pub mod part2 {
    use crate::day_12::{parse, run};

    pub fn solve(input: &str) -> isize {
        let instructions = parse(input);
        let mut registers = [0, 0, 1, 0];
        run(instructions, &mut registers);
        registers[0]
    }
}

pub fn main(test: bool) {
    let test_input = "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_12_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
