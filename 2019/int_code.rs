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
    Input,
    Output(isize),
    Halt,
    Error,
}

#[derive(Clone, Debug)]
pub(crate) struct IntCode {
    instructions: Vec<isize>,
    pc: usize,
    input_queue: VecDeque<isize>,
    output: VecDeque<isize>,
}

impl IntCode {
    pub(crate) fn new(instructions: Vec<isize>) -> Self {
        Self {
            instructions,
            pc: 0,
            input_queue: Default::default(),
            output: Default::default(),
        }
    }

    pub(crate) fn with_input(instructions: Vec<isize>, input: VecDeque<isize>) -> Self {
        let mut result = Self::new(instructions);
        result.input_queue = input;
        result
    }

    pub(crate) fn last_output(&self) -> Option<isize> {
        self.output.back().cloned()
    }

    pub(crate) fn push_input(&mut self, value: isize) {
        self.input_queue.push_back(value);
    }

    pub(crate) fn run_until_complete(&mut self) {
        loop {
            match self.run_until_interrupt() {
                Interrupt::Input => {}
                Interrupt::Output(_) => {}
                Interrupt::Halt => break,
                Interrupt::Error => break,
            }
        }
    }

    pub(crate) fn run_until_interrupt(&mut self) -> Interrupt {
        while self.pc < self.instructions.len() {
            let (consumed, instruction) = Instruction::parse(&self.instructions[self.pc..]);
            self.pc += consumed;
            match instruction {
                Instruction::Add(l, r, dest) => {
                    let (l, r) = (l.get(&self.instructions), r.get(&self.instructions));
                    if let Parameter {
                        value,
                        mode: Mode::Position,
                    } = dest
                    {
                        self.instructions[value as usize] = l + r;
                    } else {
                        panic!("Invalid mode for writing: {:?}", dest.mode)
                    }
                }
                Instruction::Mul(l, r, dest) => {
                    let (l, r) = (l.get(&self.instructions), r.get(&self.instructions));
                    if let Parameter {
                        value,
                        mode: Mode::Position,
                    } = dest
                    {
                        self.instructions[value as usize] = l * r;
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
                        if let Some(input) = self.input_queue.pop_front() {
                            self.instructions[value as usize] = input
                        } else {
                            return Interrupt::Input;
                        }
                    } else {
                        panic!("Invalid mode for writing: {:?}", dest.mode)
                    }
                }
                Instruction::Out(value) => {
                    let value = value.get(&self.instructions);
                    self.output.push_back(value);
                    return Interrupt::Output(value);
                }
                Instruction::Jit(cond, dest) => {
                    let (cond, dest) = (cond.get(&self.instructions), dest.get(&self.instructions));
                    if cond != 0 {
                        self.pc = dest as usize;
                    }
                }
                Instruction::Jif(cond, dest) => {
                    let (cond, dest) = (cond.get(&self.instructions), dest.get(&self.instructions));
                    if cond == 0 {
                        self.pc = dest as usize;
                    }
                }
                Instruction::Lt(l, r, dest) => {
                    let (l, r) = (l.get(&self.instructions), r.get(&self.instructions));
                    if let Parameter {
                        value,
                        mode: Mode::Position,
                    } = dest
                    {
                        self.instructions[value as usize] = if l < r { 1 } else { 0 };
                    } else {
                        panic!("Invalid mode for writing: {:?}", dest.mode)
                    }
                }
                Instruction::Eq(l, r, dest) => {
                    let (l, r) = (l.get(&self.instructions), r.get(&self.instructions));
                    if let Parameter {
                        value,
                        mode: Mode::Position,
                    } = dest
                    {
                        self.instructions[value as usize] = if l == r { 1 } else { 0 };
                    } else {
                        panic!("Invalid mode for writing: {:?}", dest.mode)
                    }
                }
                Instruction::Halt => return Interrupt::Halt,
            }
        }
        Interrupt::Halt
    }
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
                3,
                Self::Jit(
                    Parameter::from(first, instructions[1]),
                    Parameter::from(second, instructions[2]),
                ),
            ),
            6 => (
                3,
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
