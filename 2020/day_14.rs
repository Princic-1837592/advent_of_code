#[derive(Clone, Copy, Debug)]
enum Operation {
    Mask,
    Mem(usize),
}

#[derive(Clone, Copy, Debug)]
struct Value {
    pub(crate) bits: [u8; 36],
    pub(crate) actual: u64,
}

#[derive(Clone, Copy, Debug)]
struct Instruction {
    pub(crate) operation: Operation,
    pub(crate) value: Value,
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        fn parse_val(val: &str) -> [u8; 36] {
            let mut value = [0; 36];
            if val.len() == 36 {
                val.chars().enumerate().for_each(|(i, b)| {
                    value[i] = match b {
                        '0' => 0,
                        '1' => 1,
                        _x => 2,
                    }
                });
            } else {
                let binary: u64 = val.parse().unwrap();
                let mut bit = 1;
                for i in (0..value.len()).rev() {
                    if binary & bit != 0 {
                        value[i] = 1
                    }
                    bit <<= 1;
                }
            }
            value
        }
        let mut parts = string.split(" = ");
        let op = parts.next().unwrap();
        let val = parts.next().unwrap();
        let value = Value {
            bits: parse_val(val),
            actual: val.parse().unwrap_or(0),
        };
        Instruction {
            operation: match op.chars().nth(1).unwrap() {
                'a' => Operation::Mask,
                _e => Operation::Mem(
                    op.chars()
                        .skip(4)
                        .take(op.len() - 4 - 1)
                        .collect::<String>()
                        .parse()
                        .unwrap(),
                ),
            },
            value,
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

mod part1 {
    use crate::{parse, Instruction, Operation, Value};

    fn apply_mask(mask: &[u8; 36], value: &[u8; 36]) -> u64 {
        let mut result = 0;
        for i in 0..mask.len() {
            result <<= 1;
            match mask[i] {
                2 => result |= value[i] as u64,
                zero_or_one => result |= zero_or_one as u64,
            }
        }
        result
    }

    pub(crate) fn solve(input: &str) -> u64 {
        let instructions = parse(input);
        let indexes = instructions.iter().filter_map(|i| match i.operation {
            Operation::Mem(index) => Some(index),
            _mask => None,
        });
        let (min, max) = (indexes.clone().min().unwrap(), indexes.max().unwrap());
        let size = max - min + 1;
        let mut memory = vec![0; size as usize];
        let mut mask = [2; 36];
        instructions.iter().for_each(|i| match i {
            Instruction {
                operation: Operation::Mask,
                value,
            } => mask = value.bits,
            Instruction {
                operation: Operation::Mem(address),
                value: Value { bits, .. },
            } => memory[*address - min] = apply_mask(&mask, bits),
        });
        memory.iter().sum()
    }
}

mod part2 {
    use std::collections::HashMap;

    use crate::{parse, Instruction, Operation, Value};

    fn write(memory: &mut HashMap<usize, u64>, address: usize, mask: &[u8; 36], value: u64) {
        fn recursive(
            memory: &mut HashMap<usize, u64>,
            mask: &mut [u8],
            address: usize,
            mut masked: usize,
            value: u64,
            i: usize,
            bit: usize,
        ) {
            if i >= mask.len() {
                memory.insert(masked, value);
            } else {
                masked <<= 1;
                match mask[i] {
                    0 => recursive(
                        memory,
                        mask,
                        address,
                        masked | (address & (1 << bit) != 0) as usize,
                        value,
                        i + 1,
                        bit - 1,
                    ),
                    1 => recursive(memory, mask, address, masked | 1, value, i + 1, bit - 1),
                    _x => {
                        recursive(memory, mask, address, masked | 1, value, i + 1, bit - 1);
                        recursive(memory, mask, address, masked, value, i + 1, bit - 1);
                    }
                }
            }
        }
        recursive(memory, &mut mask.clone(), address, 0, value, 0, 35);
    }

    pub(crate) fn solve(input: &str) -> u64 {
        let instructions = parse(input);
        let mut memory = HashMap::new();
        let mut mask = [2; 36];
        instructions.iter().for_each(|i| match i {
            Instruction {
                operation: Operation::Mask,
                value,
            } => mask = value.bits,
            Instruction {
                operation: Operation::Mem(address),
                value: Value { actual, .. },
            } => write(&mut memory, *address, &mask, *actual),
        });
        memory.values().sum()
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_14_input.txt").unwrap()
    };
    println!("{}", part1::solve(&puzzle_input));
    println!("{}", part2::solve(&puzzle_input));
}
