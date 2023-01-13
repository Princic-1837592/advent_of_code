use std::time::Instant;

type Argument = isize;
enum Operation {
    Acc,
    Jmp,
    Nop,
}
struct Instruction(Operation, Argument);

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let mut parts = string.split(' ');
        let op = parts.next().unwrap();
        let arg = parts.next().unwrap().parse().unwrap();
        Instruction(
            match op.chars().next().unwrap() {
                'a' => Operation::Acc,
                'j' => Operation::Jmp,
                'n' => Operation::Nop,
                _ => panic!("Invalid instruction: {}", op),
            },
            arg,
        )
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn run(instructions: &Vec<Instruction>) -> (bool, isize) {
    let mut executed = vec![false; instructions.len()];
    let (mut instr_ptr, mut accumulator) = (0_isize, 0);
    while instr_ptr < instructions.len() as isize && !executed[instr_ptr as usize] {
        executed[instr_ptr as usize] = true;
        let Instruction(op, arg) = &instructions[instr_ptr as usize];
        match op {
            Operation::Acc => {
                accumulator += arg;
                instr_ptr += 1
            }
            Operation::Jmp => instr_ptr += *arg,
            Operation::Nop => instr_ptr += 1,
        }
    }
    (instr_ptr == instructions.len() as isize, accumulator)
}

pub mod part1 {
    use super::{parse, run, Instruction, Operation};

    pub fn solve(input: &str) -> isize {
        let instructions = parse(input);
        run(&instructions).1
    }
}

pub mod part2 {
    use super::{parse, run, Operation};

    pub fn solve(input: &str) -> isize {
        let mut instructions = parse(input);
        for i in 0..instructions.len() {
            let before = match &instructions[i].0 {
                Operation::Jmp => {
                    instructions[i].0 = Operation::Nop;
                    Operation::Jmp
                }
                Operation::Nop => {
                    instructions[i].0 = Operation::Jmp;
                    Operation::Nop
                }
                Operation::Acc => Operation::Acc,
            };
            let (finished, accumulator) = run(&instructions);
            if finished {
                return accumulator;
            }
            instructions[i].0 = before;
        }
        panic!()
    }
}

pub fn main(test: bool) {
    let test_input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_08_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
