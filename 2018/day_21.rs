//! https://adventofcode.com/2018/day/21
//! https://adventofcode.com/2018/day/21/input

use std::{fs::read_to_string, time::Instant};

type Registers = [usize; 6];

#[derive(Copy, Clone, Debug)]
struct Instruction {
    opcode: Behavior,
    a: usize,
    b: usize,
    c: usize,
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let mut parts = string.split_whitespace();
        Instruction {
            opcode: Behavior::from(parts.next().unwrap()),
            a: parts.next().unwrap().parse().unwrap(),
            b: parts.next().unwrap().parse().unwrap(),
            c: parts.next().unwrap().parse().unwrap(),
        }
    }
}

impl Instruction {
    fn get_arguments(&self, registers: &Registers) -> (usize, usize) {
        match self.opcode {
            Behavior::Addr
            | Behavior::Mulr
            | Behavior::Banr
            | Behavior::Borr
            | Behavior::Setr
            | Behavior::Gtrr
            | Behavior::Eqrr => (registers[self.a], registers[self.b]),
            Behavior::Addi
            | Behavior::Muli
            | Behavior::Bani
            | Behavior::Bori
            | Behavior::Gtri
            | Behavior::Eqri => (registers[self.a], self.b),
            Behavior::Seti => (self.a, 0),
            Behavior::Gtir | Behavior::Eqir => (self.a, registers[self.b]),
        }
    }

    fn apply(&self, registers: &mut Registers) {
        let (a, b) = self.get_arguments(registers);
        registers[self.c] = match self.opcode {
            Behavior::Addr | Behavior::Addi => a + b,
            Behavior::Mulr | Behavior::Muli => a * b,
            Behavior::Banr | Behavior::Bani => a & b,
            Behavior::Borr | Behavior::Bori => a | b,
            Behavior::Setr | Behavior::Seti => a,
            Behavior::Gtir | Behavior::Gtri | Behavior::Gtrr => (a > b) as usize,
            Behavior::Eqir | Behavior::Eqri | Behavior::Eqrr => (a == b) as usize,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Behavior {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl From<&str> for Behavior {
    fn from(string: &str) -> Self {
        match string {
            "addr" => Behavior::Addr,
            "addi" => Behavior::Addi,
            "mulr" => Behavior::Mulr,
            "muli" => Behavior::Muli,
            "banr" => Behavior::Banr,
            "bani" => Behavior::Bani,
            "borr" => Behavior::Borr,
            "bori" => Behavior::Bori,
            "setr" => Behavior::Setr,
            "seti" => Behavior::Seti,
            "gtir" => Behavior::Gtir,
            "gtri" => Behavior::Gtri,
            "gtrr" => Behavior::Gtrr,
            "eqir" => Behavior::Eqir,
            "eqri" => Behavior::Eqri,
            "eqrr" => Behavior::Eqrr,
            _ => {
                panic!("Invalid instruction: {}", string)
            }
        }
    }
}

fn parse(input: &str) -> (usize, Vec<Instruction>) {
    let mut lines = input.lines();
    (
        lines
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap(),
        lines.map(Instruction::from).collect(),
    )
}

pub mod part1 {
    use crate::day_21::parse;

    pub fn solve(input: &str) -> usize {
        let (ip, instructions) = parse(input);
        let mut registers = [0; 6];
        while registers[ip] < instructions.len() {
            instructions[registers[ip]].apply(&mut registers);
            registers[ip] += 1;
            if registers[ip] == 29 {
                break;
            }
        }
        registers[4]
    }
}

pub mod part2 {
    use std::collections::HashSet;

    use crate::day_21::parse;

    pub fn solve(input: &str) -> usize {
        let (ip, instructions) = parse(input);
        let mut registers = [0; 6];
        let mut r4s = HashSet::new();
        let mut last = 0;
        while registers[ip] < instructions.len() {
            instructions[registers[ip]].apply(&mut registers);
            registers[ip] += 1;
            if registers[ip] == 28 {
                if r4s.contains(&registers[4]) {
                    break;
                }
                last = registers[4];
                r4s.insert(last);
            }
        }
        last
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_21_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
