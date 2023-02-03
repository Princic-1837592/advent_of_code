//! https://adventofcode.com/2018/day/16
//! https://adventofcode.com/2018/day/16/input

use std::{fs::read_to_string, time::Instant};

use crate::LINE_ENDING;

type Registers = [usize; 4];

#[derive(Copy, Clone, Debug)]
struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let mut parts = string.split_whitespace();
        Instruction {
            opcode: parts.next().unwrap().parse().unwrap(),
            a: parts.next().unwrap().parse().unwrap(),
            b: parts.next().unwrap().parse().unwrap(),
            c: parts.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Sample {
    before: Registers,
    instruction: Instruction,
    after: Registers,
}

impl From<&str> for Sample {
    fn from(string: &str) -> Self {
        let mut parts = string.lines();
        let mut before_line = parts.next().unwrap();
        before_line = &before_line[9..before_line.len() - 1];
        let mut before = [0; 4];
        for (i, value) in before_line.split(", ").enumerate() {
            before[i] = value.parse().unwrap();
        }
        let instruction = Instruction::from(parts.next().unwrap());
        let mut after_line = parts.next().unwrap();
        after_line = &after_line[9..after_line.len() - 1];
        let mut after = [0; 4];
        for (i, value) in after_line.split(", ").enumerate() {
            after[i] = value.parse().unwrap();
        }
        Sample {
            before,
            instruction,
            after,
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

const OPCODES: [Behavior; 16] = [
    Behavior::Addr,
    Behavior::Addi,
    Behavior::Mulr,
    Behavior::Muli,
    Behavior::Banr,
    Behavior::Bani,
    Behavior::Borr,
    Behavior::Bori,
    Behavior::Setr,
    Behavior::Seti,
    Behavior::Gtir,
    Behavior::Gtri,
    Behavior::Gtrr,
    Behavior::Eqir,
    Behavior::Eqri,
    Behavior::Eqrr,
];

impl Instruction {
    fn get_arguments(&self, registers: &Registers, opcodes: &[Behavior; 16]) -> (usize, usize) {
        match opcodes[self.opcode] {
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

    fn apply(&self, registers: &mut Registers, opcodes: &[Behavior; 16]) {
        let (a, b) = self.get_arguments(registers, opcodes);
        registers[self.c] = match opcodes[self.opcode] {
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

fn parse(input: &str) -> (Vec<Sample>, Vec<Instruction>) {
    let separator = LINE_ENDING.repeat(4);
    let mut parts = input.split(&separator);
    let separator = LINE_ENDING.repeat(2);
    let samples = parts.next().unwrap().split(&separator);
    let instructions = parts.next().unwrap().lines();
    (
        samples.map(Sample::from).collect(),
        instructions.map(Instruction::from).collect(),
    )
}

pub mod part1 {
    use crate::day_16::{parse, Sample, OPCODES};

    pub fn solve(input: &str) -> usize {
        let (samples, _) = parse(input);
        let mut result = 0;
        for Sample {
            before,
            mut instruction,
            after,
        } in samples
        {
            let mut valid = 0;
            for opcode in 0..16 {
                let mut before = before;
                instruction.opcode = opcode;
                instruction.apply(&mut before, &OPCODES);
                if before == after {
                    valid += 1;
                }
                if valid >= 3 {
                    break;
                }
            }
            if valid >= 3 {
                result += 1;
            }
        }
        result
    }
}

pub mod part2 {
    use std::collections::HashSet;

    use crate::day_16::{parse, Behavior, Sample, OPCODES};

    pub fn solve(input: &str) -> usize {
        let (samples, instructions) = parse(input);
        let mut valid = [
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
        ];
        for Sample {
            before,
            mut instruction,
            after,
        } in samples
        {
            let original = instruction.opcode;
            for opcode in 0..16 {
                let mut before = before;
                instruction.opcode = opcode;
                instruction.apply(&mut before, &OPCODES);
                if before == after {
                    valid[original].insert(opcode);
                }
            }
        }
        while valid.iter().any(|valid| valid.len() != 1) {
            let mut fixed = [false; 16];
            for opcode in valid
                .iter()
                .filter_map(|valid| (valid.len() == 1).then_some(*valid.iter().next().unwrap()))
            {
                fixed[opcode] = true
            }
            valid
                .iter_mut()
                .filter(|valid| valid.len() > 1)
                .for_each(|valid| valid.retain(|opcode| !fixed[*opcode]));
        }
        let mut opcodes = [Behavior::Addr; 16];
        for (opcode, valid) in valid.iter().enumerate() {
            opcodes[opcode] = OPCODES[*valid.iter().next().unwrap()];
        }
        let mut registers = [0; 4];
        for instruction in instructions {
            instruction.apply(&mut registers, &opcodes);
        }
        registers[0]
    }
}

pub fn main(test: bool) {
    let test_input = "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]



1 2 3 4"
        .to_owned()
        .replace('\n', "\r\n");
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_16_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
