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
