use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Mode {
    Position,
    Immediate,
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Parameter {
    pub(crate) value: isize,
    pub(crate) mode: Mode,
}

impl Parameter {
    pub(crate) fn from(mode: isize, value: isize) -> Self {
        Self {
            mode: match mode {
                0 => Mode::Position,
                1 => Mode::Immediate,
                _ => panic!("Invalid mode: {}", mode),
            },
            value,
        }
    }

    pub(crate) fn get(&self, instructions: &[isize]) -> isize {
        match self.mode {
            Mode::Position => instructions[self.value as usize],
            Mode::Immediate => self.value,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    In(Parameter),
    Out(Parameter),
    Jit(Parameter, Parameter),
    Jif(Parameter, Parameter),
    Lt(Parameter, Parameter, Parameter),
    Eq(Parameter, Parameter, Parameter),
    Halt,
}

#[derive(Clone, Debug)]
pub(crate) enum Interrupt {
    Input(Vec<isize>, usize),
    Output(Vec<isize>, usize, isize),
    Halt(Vec<isize>, usize),
    Error,
}

fn ith_digit(n: isize, i: u32) -> isize {
    (n / 10_isize.pow(i - 1)) % 10
}

impl Instruction {
    pub(crate) fn parse(instructions: &[isize]) -> (usize, Self) {
        let op_and_params = instructions[0];
        let op = 10 * ith_digit(op_and_params, 2) + ith_digit(op_and_params, 1);
        let (first, second, third) = (
            ith_digit(op_and_params, 3),
            ith_digit(op_and_params, 4),
            ith_digit(op_and_params, 5),
        );
        match op {
            1 => (
                4,
                Self::Add(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                    Parameter::from(third, instructions[3]),
                ),
            ),
            2 => (
                4,
                Self::Mul(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                    Parameter::from(third, instructions[3]),
                ),
            ),
            3 => (2, Self::In(Parameter::from(first, instructions[1]))),
            4 => (2, Self::Out(Parameter::from(first, instructions[1]))),
            5 => (
                0,
                Self::Jit(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                ),
            ),
            6 => (
                0,
                Self::Jif(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                ),
            ),
            7 => (
                4,
                Self::Lt(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                    Parameter::from(third, instructions[3]),
                ),
            ),
            8 => (
                4,
                Self::Eq(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                    Parameter::from(third, instructions[3]),
                ),
            ),
            99 => (1, Self::Halt),
            _ => {
                panic!("Invalid instruction: {}", instructions[0])
            }
        }
    }
}

pub(crate) fn parse(input: &str) -> Vec<isize> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

pub(crate) fn run(mut instructions: Vec<isize>, mut input_queue: VecDeque<isize>) -> Interrupt {
    let mut pc = 0;
    while pc < instructions.len() {
        let (consumed, instruction) = Instruction::parse(&instructions[pc..]);
        pc += consumed;
        match instruction {
            Instruction::Add(l, r, dest) => {
                let (l, r) = (l.get(&instructions), r.get(&instructions));
                if let Parameter {
                    value,
                    mode: Mode::Position,
                } = dest
                {
                    instructions[value as usize] = l + r;
                } else {
                    panic!("Invalid mode for writing: {:?}", dest.mode)
                }
            }
            Instruction::Mul(l, r, dest) => {
                let (l, r) = (l.get(&instructions), r.get(&instructions));
                if let Parameter {
                    value,
                    mode: Mode::Position,
                } = dest
                {
                    instructions[value as usize] = l * r;
                } else {
                    panic!("Invalid mode for writing: {:?}", dest.mode)
                }
            }
            Instruction::In(dest) => {
                if let Parameter {
                    value,
                    mode: Mode::Position,
                } = dest
                {
                    if let Some(input) = input_queue.pop_front() {
                        instructions[value as usize] = input
                    } else {
                        return Interrupt::Input(instructions, pc);
                    }
                } else {
                    panic!("Invalid mode for writing: {:?}", dest.mode)
                }
            }
            Instruction::Out(value) => {
                let value = value.get(&instructions);
                return Interrupt::Output(instructions, pc, value);
            }
            Instruction::Jit(cond, dest) => {
                let (cond, dest) = (cond.get(&instructions), dest.get(&instructions));
                if cond != 0 {
                    pc = dest as usize;
                }
            }
            Instruction::Jif(cond, dest) => {
                let (cond, dest) = (cond.get(&instructions), dest.get(&instructions));
                if cond == 0 {
                    pc = dest as usize;
                }
            }
            Instruction::Lt(l, r, dest) => {
                let (l, r) = (l.get(&instructions), r.get(&instructions));
                if let Parameter {
                    value,
                    mode: Mode::Position,
                } = dest
                {
                    instructions[value as usize] = if l < r { 1 } else { 0 };
                } else {
                    panic!("Invalid mode for writing: {:?}", dest.mode)
                }
            }
            Instruction::Eq(l, r, dest) => {
                let (l, r) = (l.get(&instructions), r.get(&instructions));
                if let Parameter {
                    value,
                    mode: Mode::Position,
                } = dest
                {
                    instructions[value as usize] = if l == r { 1 } else { 0 };
                } else {
                    panic!("Invalid mode for writing: {:?}", dest.mode)
                }
            }
            Instruction::Halt => return Interrupt::Halt(instructions, pc),
        }
    }
    Interrupt::Halt(instructions, pc)
}
