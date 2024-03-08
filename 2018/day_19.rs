//! https://adventofcode.com/2018/day/19
//! https://adventofcode.com/2018/day/19/input

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
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let (ip, instructions) = parse(input);
        let mut registers = [0; 6];
        while registers[ip] < instructions.len() {
            instructions[registers[ip]].apply(&mut registers);
            registers[ip] += 1;
        }
        registers[ip] -= 1;
        registers[0]
    }
}

pub mod part2 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let (ip, instructions) = parse(input);
        let mut registers = [0; 6];
        registers[0] = 1;
        while registers[ip] < instructions.len() && registers[1] < 10000 {
            instructions[registers[ip]].apply(&mut registers);
            registers[ip] += 1;
        }
        let target = registers[1];
        let sqrt = (target as f32).sqrt();
        let mut sum = if sqrt.floor() == sqrt {
            sqrt.floor() as usize
        } else {
            0
        };
        for n in 1..sqrt.floor() as usize {
            if target % n == 0 {
                sum += n + target / n;
            }
        }
        sum
    }
}

pub fn main(test: bool) {
    let test_input = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2018/day_19_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
