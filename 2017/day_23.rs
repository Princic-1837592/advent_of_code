//! https://adventofcode.com/2017/day/23
//! https://adventofcode.com/2017/day/23/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
enum Operand {
    Register(usize),
    Integer(isize),
}

impl From<&str> for Operand {
    fn from(string: &str) -> Self {
        match string.chars().next().unwrap() {
            register @ 'a'..='z' => Self::Register(register as usize - 'a' as usize),
            _ => Self::Integer(string.parse().unwrap()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Set(usize, Operand),
    Sub(usize, Operand),
    Mul(usize, Operand),
    Jnz(Operand, Operand),
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let parts: Vec<_> = string.split_whitespace().collect();
        let register = (parts[1].chars().next().unwrap() as usize).saturating_sub('a' as usize);
        let operand = parts.get(2).map(|&operand| Operand::from(operand));
        match parts[0] {
            "set" => Self::Set(register, operand.unwrap()),
            "sub" => Self::Sub(register, operand.unwrap()),
            "mul" => Self::Mul(register, operand.unwrap()),
            "jnz" => Self::Jnz(Operand::from(parts[1]), operand.unwrap()),
            _ => panic!("Invalid instruction: {}", string),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

pub mod part1 {
    use super::{parse, Instruction, Operand};

    pub fn solve(input: &str) -> usize {
        let instructions = parse(input);
        let mut registers = [0; 'h' as usize - 'a' as usize + 1];
        let mut muls = 0;
        let mut ip = 0;
        while (ip as usize) < instructions.len() {
            match instructions[ip as usize] {
                Instruction::Set(r, s) => {
                    registers[r] = match s {
                        Operand::Register(s) => registers[s],
                        Operand::Integer(i) => i,
                    }
                }
                Instruction::Sub(r, s) => {
                    registers[r] -= match s {
                        Operand::Register(s) => registers[s],
                        Operand::Integer(i) => i,
                    }
                }
                Instruction::Mul(r, s) => {
                    registers[r] *= match s {
                        Operand::Register(s) => registers[s],
                        Operand::Integer(i) => i,
                    };
                    muls += 1;
                }
                Instruction::Jnz(r, s) => {
                    if match r {
                        Operand::Register(r) => registers[r],
                        Operand::Integer(i) => i,
                    } != 0
                    {
                        ip =
                            ip + match s {
                                Operand::Register(s) => registers[s],
                                Operand::Integer(i) => i,
                            } - 1;
                    }
                }
            }
            ip += 1;
        }
        muls
    }
}

pub mod part2 {
    use super::{parse, Instruction, Operand};

    pub fn solve(input: &str) -> usize {
        fn sieve(primes: &mut Vec<isize>, factor: isize) {
            for value in primes {
                if *value != 0 && *value != factor && *value % factor == 0 {
                    *value = 0;
                }
            }
        }

        let instructions = parse(input);
        let mut registers = [0; 'h' as usize - 'a' as usize + 1];
        registers[0] = 1;
        let mut ip = 0;
        let mut x = 0;
        while (ip as usize) < instructions.len() && x < 10 {
            x += 1;
            match instructions[ip as usize] {
                Instruction::Set(r, s) => {
                    registers[r] = match s {
                        Operand::Register(s) => registers[s],
                        Operand::Integer(i) => i,
                    }
                }
                Instruction::Sub(r, s) => {
                    registers[r] -= match s {
                        Operand::Register(s) => registers[s],
                        Operand::Integer(i) => i,
                    }
                }
                Instruction::Mul(r, s) => {
                    registers[r] *= match s {
                        Operand::Register(s) => registers[s],
                        Operand::Integer(i) => i,
                    };
                }
                Instruction::Jnz(r, s) => {
                    if match r {
                        Operand::Register(r) => registers[r],
                        Operand::Integer(i) => i,
                    } != 0
                    {
                        ip =
                            ip + match s {
                                Operand::Register(s) => registers[s],
                                Operand::Integer(i) => i,
                            } - 1;
                    }
                }
            }
            ip += 1;
        }
        let (lower, upper) = (
            registers['b' as usize - 'a' as usize] as usize,
            registers['c' as usize - 'a' as usize],
        );
        let mut primes: Vec<_> = (0..=upper).collect();
        for i in 2..primes.len() {
            let factor = primes[i];
            if factor != 0 {
                sieve(&mut primes, factor);
            }
        }
        primes
            .iter()
            .enumerate()
            .filter(|&(i, &prime)| i >= lower && i % 17 == lower % 17 && prime == 0)
            .count()
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2017/day_23_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
