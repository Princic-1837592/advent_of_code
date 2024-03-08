//! https://adventofcode.com/2015/day/23
//! https://adventofcode.com/2015/day/23/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

enum Instruction {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(isize),
    Jie(usize, isize),
    Jio(usize, isize),
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let parts: Vec<_> = string.split(' ').collect();
        match parts[0] {
            "hlf" => Instruction::Hlf((parts[1].chars().next().unwrap() as u8 - b'a') as usize),
            "tpl" => Instruction::Tpl((parts[1].chars().next().unwrap() as u8 - b'a') as usize),
            "inc" => Instruction::Inc((parts[1].chars().next().unwrap() as u8 - b'a') as usize),
            "jmp" => Instruction::Jmp(parts[1].parse().unwrap()),
            "jie" => Instruction::Jie(
                (parts[1].chars().next().unwrap() as u8 - b'a') as usize,
                parts[2].parse().unwrap(),
            ),
            "jio" => Instruction::Jio(
                (parts[1].chars().next().unwrap() as u8 - b'a') as usize,
                parts[2].parse().unwrap(),
            ),
            _ => panic!("Invalid instruction: {}", string),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn compute(a: usize, instructions: Vec<Instruction>) -> usize {
    let mut registers = [a, 0];
    let mut pc: isize = 0;
    while (pc as usize) < instructions.len() {
        pc += match instructions[pc as usize] {
            Instruction::Hlf(reg) => {
                registers[reg] /= 2;
                1
            }
            Instruction::Tpl(reg) => {
                registers[reg] *= 3;
                1
            }
            Instruction::Inc(reg) => {
                registers[reg] += 1;
                1
            }
            Instruction::Jmp(offset) => {
                pc += offset;
                0
            }
            Instruction::Jie(reg, offset) => {
                if registers[reg] % 2 == 0 {
                    pc += offset;
                    0
                } else {
                    1
                }
            }
            Instruction::Jio(reg, offset) => {
                if registers[reg] == 1 {
                    pc += offset;
                    0
                } else {
                    1
                }
            }
        };
    }
    registers[1]
}

pub mod part1 {
    use super::{compute, parse};

    pub fn solve(input: &str, a: usize) -> usize {
        let instructions = parse(input);
        compute(a, instructions)
    }
}

pub mod part2 {
    use super::{compute, parse};

    pub fn solve(input: &str) -> usize {
        let instructions = parse(input);
        compute(1, instructions)
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "inc a
jio a, +2
tpl a
inc a"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2015/day_23_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let result = part1::solve(&puzzle_input, 0);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(&puzzle_input);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
