//! https://adventofcode.com/2017/day/8
//! https://adventofcode.com/2017/day/8/input

use std::{collections::HashMap, fs::read_to_string, time::Instant};

enum Operator {
    Eq,
    Ne,
    Gt,
    Get,
    Lt,
    Let,
}

impl From<&str> for Operator {
    fn from(string: &str) -> Self {
        match string {
            "==" => Self::Eq,
            "!=" => Self::Ne,
            ">" => Self::Gt,
            ">=" => Self::Get,
            "<" => Self::Lt,
            "<=" => Self::Let,
            _ => panic!("Invalid operator: {}", string),
        }
    }
}

struct Instruction {
    dest: String,
    delta: isize,
    target: String,
    operator: Operator,
    n: isize,
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let mut parts = string.split_whitespace();
        let dest = parts.next().unwrap().to_owned();
        let delta = if parts.next().unwrap() == "inc" {
            1
        } else {
            -1
        } * parts.next().unwrap().parse::<isize>().unwrap();
        let target = parts.nth(1).unwrap().to_owned();
        let operator = parts.next().unwrap();
        let n = parts.next().unwrap().parse().unwrap();
        Instruction {
            dest,
            delta,
            target,
            operator: Operator::from(operator),
            n,
        }
    }
}

impl Instruction {
    fn apply(&self, registers: &HashMap<String, isize>) -> bool {
        let target = *registers.get(&self.target).unwrap_or(&0);
        match self.operator {
            Operator::Eq => target == self.n,
            Operator::Ne => target != self.n,
            Operator::Gt => target > self.n,
            Operator::Get => target >= self.n,
            Operator::Lt => target < self.n,
            Operator::Let => target <= self.n,
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

pub mod part1 {
    use std::collections::HashMap;

    use super::parse;

    pub fn solve(input: &str) -> isize {
        let instructions = parse(input);
        let mut registers = HashMap::new();
        for instruction in instructions {
            if instruction.apply(&registers) {
                *registers.entry(instruction.dest).or_insert(0) += instruction.delta;
            }
        }
        *registers.values().max().unwrap()
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use super::parse;

    pub fn solve(input: &str) -> isize {
        let instructions = parse(input);
        let mut registers = HashMap::new();
        let mut max = 0;
        for instruction in instructions {
            if instruction.apply(&registers) {
                *registers.entry(instruction.dest.clone()).or_insert(0) += instruction.delta;
                max = max.max(*registers.get(&instruction.dest).unwrap());
            }
        }
        max
    }
}

pub fn main(test: bool) {
    let test_input = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_08_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
