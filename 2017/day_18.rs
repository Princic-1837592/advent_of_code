//! https://adventofcode.com/2017/day/18
//! https://adventofcode.com/2017/day/18/input

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
    Snd(Operand),
    Set(usize, Operand),
    Add(usize, Operand),
    Mul(usize, Operand),
    Mod(usize, Operand),
    Rcv(usize),
    Jgz(Operand, Operand),
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let parts: Vec<_> = string.split_whitespace().collect();
        let register = (parts[1].chars().next().unwrap() as usize).saturating_sub('a' as usize);
        let operand = parts.get(2).map(|&operand| Operand::from(operand));
        match parts[0] {
            "snd" => Self::Snd(Operand::from(parts[1])),
            "set" => Self::Set(register, operand.unwrap()),
            "add" => Self::Add(register, operand.unwrap()),
            "mul" => Self::Mul(register, operand.unwrap()),
            "mod" => Self::Mod(register, operand.unwrap()),
            "rcv" => Self::Rcv(register),
            "jgz" => Self::Jgz(Operand::from(parts[1]), operand.unwrap()),
            _ => panic!("Invalid instruction: {}", string),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

pub mod part1 {
    use super::{parse, Instruction, Operand};

    pub fn solve(input: &str) -> isize {
        let instructions = parse(input);
        let mut registers = [0; 'z' as usize - 'a' as usize + 1];
        let mut frequency = None;
        let mut ip = 0;
        while (ip as usize) < instructions.len() {
            match instructions[ip as usize] {
                Instruction::Snd(r) => {
                    frequency = Some(match r {
                        Operand::Register(r) => registers[r],
                        Operand::Integer(i) => i,
                    })
                }
                Instruction::Set(r, s) => {
                    registers[r] = match s {
                        Operand::Register(s) => registers[s],
                        Operand::Integer(i) => i,
                    }
                }
                Instruction::Add(r, s) => {
                    registers[r] += match s {
                        Operand::Register(s) => registers[s],
                        Operand::Integer(i) => i,
                    }
                }
                Instruction::Mul(r, s) => {
                    registers[r] *= match s {
                        Operand::Register(s) => registers[s],
                        Operand::Integer(i) => i,
                    }
                }
                Instruction::Mod(r, s) => {
                    registers[r] %= match s {
                        Operand::Register(s) => registers[s],
                        Operand::Integer(i) => i,
                    }
                }
                Instruction::Rcv(r) if registers[r] != 0 => return frequency.unwrap(),
                Instruction::Jgz(r, s) => {
                    if match r {
                        Operand::Register(r) => registers[r],
                        Operand::Integer(i) => i,
                    } > 0
                    {
                        ip =
                            ip + match s {
                                Operand::Register(s) => registers[s],
                                Operand::Integer(i) => i,
                            } - 1;
                    }
                }
                _ => {}
            }
            ip += 1;
        }
        0
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    use super::{parse, Instruction, Operand};

    pub fn solve(input: &str) -> usize {
        let instructions = parse(input);
        let mut registers = [[0; 'z' as usize - 'a' as usize + 1]; 2];
        registers[1]['p' as usize - 'a' as usize] = 1;
        let mut ips = [0, 0];
        let mut queues = [VecDeque::new(), VecDeque::new()];
        let mut locked = [false; 2];
        let mut sent = [0; 2];
        while ips.iter().all(|&ip| (ip as usize) < instructions.len())
            && locked.iter().any(|&lock| !lock)
        {
            for program in 0..=1 {
                let ip = &mut ips[program];
                let registers = &mut registers[program];
                match instructions[*ip as usize] {
                    Instruction::Snd(r) => {
                        queues[1 - program].push_back(match r {
                            Operand::Register(r) => registers[r],
                            Operand::Integer(i) => i,
                        });
                        sent[program] += 1;
                    }
                    Instruction::Set(r, s) => {
                        registers[r] = match s {
                            Operand::Register(s) => registers[s],
                            Operand::Integer(i) => i,
                        };
                    }
                    Instruction::Add(r, s) => {
                        registers[r] += match s {
                            Operand::Register(s) => registers[s],
                            Operand::Integer(i) => i,
                        };
                    }
                    Instruction::Mul(r, s) => {
                        registers[r] *= match s {
                            Operand::Register(s) => registers[s],
                            Operand::Integer(i) => i,
                        };
                    }
                    Instruction::Mod(r, s) => {
                        registers[r] %= match s {
                            Operand::Register(s) => registers[s],
                            Operand::Integer(i) => i,
                        };
                    }
                    Instruction::Rcv(r) => {
                        if let Some(msg) = queues[program].pop_front() {
                            locked[program] = false;
                            registers[r] = msg;
                        } else {
                            locked[program] = true;
                            *ip -= 1;
                        }
                    }
                    Instruction::Jgz(r, s) => {
                        if match r {
                            Operand::Register(r) => registers[r],
                            Operand::Integer(i) => i,
                        } > 0
                        {
                            *ip =
                                *ip + match s {
                                    Operand::Register(s) => registers[s],
                                    Operand::Integer(i) => i,
                                } - 1;
                        }
                    }
                }
                *ip += 1;
            }
        }
        sent[1]
    }
}

pub fn main(test: bool) {
    let test_input = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_18_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
