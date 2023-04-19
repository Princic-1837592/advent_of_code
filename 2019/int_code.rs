use std::collections::{HashMap, VecDeque};
#[allow(unused)]
use std::io;

#[derive(Copy, Clone, Debug)]
pub(crate) enum Mode {
    Position,
    Immediate,
    Relative,
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
                2 => Mode::Relative,
                _ => panic!("Invalid mode: {}", mode),
            },
            value,
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
    Arb(Parameter),
    Halt,
}

#[derive(Clone, Debug)]
pub(crate) enum Interrupt {
    Input,
    Output(isize),
    Halt,
    #[allow(unused)]
    Error,
}

#[derive(Clone, Debug)]
pub(crate) struct IntCode {
    instructions: HashMap<isize, isize>,
    pc: isize,
    input_queue: VecDeque<isize>,
    output: VecDeque<isize>,
    relative_base: isize,
}

impl IntCode {
    pub(crate) fn new(instructions: Vec<isize>) -> Self {
        Self {
            instructions: instructions
                .into_iter()
                .enumerate()
                .map(|(i, v)| (i as isize, v))
                .collect(),
            pc: 0,
            input_queue: Default::default(),
            output: Default::default(),
            relative_base: 0,
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

    pub(crate) fn get_output(&self) -> &VecDeque<isize> {
        &self.output
    }

    fn get_param(&self, param: Parameter) -> isize {
        match param.mode {
            Mode::Position => *self.instructions.get(&param.value).unwrap_or(&0),
            Mode::Immediate => param.value,
            Mode::Relative => *self
                .instructions
                .get(&(param.value + self.relative_base))
                .unwrap_or(&0),
        }
    }

    pub(crate) fn run_until_complete(&mut self) {
        loop {
            match self.run_until_interrupt() {
                Interrupt::Input => {
                    // break;

                    // enable for day 25
                    // let mut buffer = String::new();
                    // let stdin = io::stdin();
                    // stdin.read_line(&mut buffer).unwrap();
                    // buffer
                    //     .chars()
                    //     .for_each(|char| self.input_queue.push_back(char as isize));
                }
                #[allow(unused)]
                Interrupt::Output(value) => {
                    //enable when you need live output, like in day 25
                    // print!("{}", value as u8 as char);
                }
                Interrupt::Halt => break,
                Interrupt::Error => break,
            }
        }
    }

    pub(crate) fn run_until_interrupt(&mut self) -> Interrupt {
        while self.instructions.contains_key(&self.pc) {
            let (consumed, instruction) = Instruction::parse(&self.instructions, self.pc);
            self.pc += consumed;
            match instruction {
                Instruction::Add(l, r, dest) => {
                    let (l, r) = (self.get_param(l), self.get_param(r));
                    let dest = match dest.mode {
                        Mode::Position => dest.value,
                        Mode::Relative => dest.value + self.relative_base,
                        _ => panic!("Invalid mode for writing: {:?}", dest.mode),
                    };
                    *self.instructions.entry(dest).or_insert(0) = l + r;
                }
                Instruction::Mul(l, r, dest) => {
                    let (l, r) = (self.get_param(l), self.get_param(r));
                    let dest = match dest.mode {
                        Mode::Position => dest.value,
                        Mode::Relative => dest.value + self.relative_base,
                        _ => panic!("Invalid mode for writing: {:?}", dest.mode),
                    };
                    *self.instructions.entry(dest).or_insert(0) = l * r;
                }
                Instruction::In(dest) => {
                    let dest = match dest.mode {
                        Mode::Position => dest.value,
                        Mode::Relative => dest.value + self.relative_base,
                        _ => panic!("Invalid mode for writing: {:?}", dest.mode),
                    };
                    if let Some(input) = self.input_queue.pop_front() {
                        *self.instructions.entry(dest).or_insert(0) = input;
                    } else {
                        self.pc -= consumed;
                        return Interrupt::Input;
                    }
                }
                Instruction::Out(value) => {
                    let value = self.get_param(value);
                    self.output.push_back(value);
                    return Interrupt::Output(value);
                }
                Instruction::Jit(cond, dest) => {
                    let (cond, dest) = (self.get_param(cond), self.get_param(dest));
                    if cond != 0 {
                        self.pc = dest;
                    }
                }
                Instruction::Jif(cond, dest) => {
                    let (cond, dest) = (self.get_param(cond), self.get_param(dest));
                    if cond == 0 {
                        self.pc = dest;
                    }
                }
                Instruction::Lt(l, r, dest) => {
                    let (l, r) = (self.get_param(l), self.get_param(r));
                    let dest = match dest.mode {
                        Mode::Position => dest.value,
                        Mode::Relative => dest.value + self.relative_base,
                        _ => panic!("Invalid mode for writing: {:?}", dest.mode),
                    };
                    *self.instructions.entry(dest).or_insert(0) = if l < r { 1 } else { 0 };
                }
                Instruction::Eq(l, r, dest) => {
                    let (l, r) = (self.get_param(l), self.get_param(r));
                    let dest = match dest.mode {
                        Mode::Position => dest.value,
                        Mode::Relative => dest.value + self.relative_base,
                        _ => panic!("Invalid mode for writing: {:?}", dest.mode),
                    };
                    *self.instructions.entry(dest).or_insert(0) = if l == r { 1 } else { 0 };
                }
                Instruction::Arb(value) => {
                    let value = self.get_param(value);
                    self.relative_base += value;
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
    pub(crate) fn parse(instructions: &HashMap<isize, isize>, index: isize) -> (isize, Self) {
        let op_and_params = *instructions.get(&index).unwrap_or(&0);
        let op = 10 * ith_digit(op_and_params, 2) + ith_digit(op_and_params, 1);
        let (first, second, third) = (
            Parameter::from(
                ith_digit(op_and_params, 3),
                *instructions.get(&(index + 1)).unwrap_or(&0),
            ),
            Parameter::from(
                ith_digit(op_and_params, 4),
                *instructions.get(&(index + 2)).unwrap_or(&0),
            ),
            Parameter::from(
                ith_digit(op_and_params, 5),
                *instructions.get(&(index + 3)).unwrap_or(&0),
            ),
        );
        match op {
            1 => (4, Self::Add(first, second, third)),
            2 => (4, Self::Mul(first, second, third)),
            3 => (2, Self::In(first)),
            4 => (2, Self::Out(first)),
            5 => (3, Self::Jit(first, second)),
            6 => (3, Self::Jif(first, second)),
            7 => (4, Self::Lt(first, second, third)),
            8 => (4, Self::Eq(first, second, third)),
            9 => (2, Self::Arb(first)),
            99 => (1, Self::Halt),
            _ => {
                panic!("Invalid instruction: {}", op_and_params)
            }
        }
    }
}

pub(crate) fn parse(input: &str) -> IntCode {
    parse_with_input(input, VecDeque::new())
}

pub(crate) fn parse_with_input(input: &str, input_queue: VecDeque<isize>) -> IntCode {
    IntCode::with_input(
        input.split(',').map(|n| n.parse().unwrap()).collect(),
        input_queue,
    )
}
