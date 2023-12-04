//! https://adventofcode.com/2015/day/7
//! https://adventofcode.com/2015/day/7/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

fn letters_to_index(input: &str) -> usize {
    input
        .chars()
        .rfold((1, 0), |(coefficient, tot), char| {
            (
                coefficient * 26,
                tot + (char as usize - 'a' as usize + 1) * coefficient,
            )
        })
        .1
}

#[derive(Copy, Clone, Debug)]
pub enum Operand {
    Const(u16),
    Wire(usize),
}

#[derive(Copy, Clone, Debug)]
pub enum Wire {
    Const(Operand),
    And(Operand, Operand),
    Or(Operand, Operand),
    Rshift(Operand, u8),
    Lshift(Operand, u8),
    Not(Operand),
}

impl From<&str> for Operand {
    fn from(string: &str) -> Self {
        if string.chars().next().unwrap().is_ascii_digit() {
            Operand::Const(string.parse().unwrap())
        } else {
            Operand::Wire(letters_to_index(string))
        }
    }
}

impl From<&str> for Wire {
    fn from(string: &str) -> Self {
        let spaces = string.chars().filter(|&c| c == ' ').count();
        if spaces == 0 {
            Wire::Const(Operand::from(string))
        } else {
            let parts: Vec<_> = string.split(' ').collect();
            if spaces != 1 {
                let operation = parts[1];
                match operation.chars().next().unwrap() {
                    'A' => Wire::And(Operand::from(parts[0]), Operand::from(parts[2])),
                    'O' => Wire::Or(Operand::from(parts[0]), Operand::from(parts[2])),
                    'R' => Wire::Rshift(Operand::from(parts[0]), parts[2].parse().unwrap()),
                    'L' => Wire::Lshift(Operand::from(parts[0]), parts[2].parse().unwrap()),
                    _ => panic!("Invalid instruction: {}", string),
                }
            } else {
                Wire::Not(Operand::from(parts[1]))
            }
        }
    }
}

type Parsed = [Option<Wire>; 676];

fn parse(input: &str) -> Parsed {
    let mut result = [None; 676];
    input.lines().for_each(|line| {
        let mut parts = line.split(" -> ");
        let left = parts.next().unwrap();
        let wire = parts.next().unwrap();
        let wire = letters_to_index(wire);
        let left = Wire::from(left);
        result[wire] = Some(left);
    });
    result
}

fn resolve(wire: usize, wires: &mut [Option<Wire>; 676], values: &mut [Option<u16>; 676]) -> u16 {
    values[wire].unwrap_or_else(|| {
        let value = match wires[wire].expect("Invalid wire") {
            Wire::Const(operand) => match operand {
                Operand::Const(value) => value,
                Operand::Wire(operand) => resolve(operand, wires, values),
            },
            Wire::And(left, right) => {
                let left = match left {
                    Operand::Const(value) => value,
                    Operand::Wire(operand) => resolve(operand, wires, values),
                };
                let right = match right {
                    Operand::Const(value) => value,
                    Operand::Wire(operand) => resolve(operand, wires, values),
                };
                left & right
            }
            Wire::Or(left, right) => {
                let left = match left {
                    Operand::Const(value) => value,
                    Operand::Wire(operand) => resolve(operand, wires, values),
                };
                let right = match right {
                    Operand::Const(value) => value,
                    Operand::Wire(operand) => resolve(operand, wires, values),
                };
                left | right
            }
            Wire::Rshift(operand, offset) => {
                (match operand {
                    Operand::Const(value) => value,
                    Operand::Wire(operand) => resolve(operand, wires, values),
                }) >> offset
            }
            Wire::Lshift(operand, offset) => {
                (match operand {
                    Operand::Const(value) => value,
                    Operand::Wire(operand) => resolve(operand, wires, values),
                }) << offset
            }
            Wire::Not(operand) => !match operand {
                Operand::Const(value) => value,
                Operand::Wire(operand) => resolve(operand, wires, values),
            },
        };
        values[wire] = Some(value);
        value
    })
}

pub mod part1 {
    use crate::day_07::{resolve, Parsed};

    pub fn solve(_input: &str, mut wires: Parsed) -> u16 {
        let mut values = [None; 676];
        resolve(1, &mut wires, &mut values);
        values[1].unwrap()
    }
}

pub mod part2 {
    use crate::day_07::{resolve, Operand, Parsed, Wire};

    pub fn solve(_input: &str, mut wires: Parsed) -> u16 {
        let mut values = [None; 676];
        resolve(1, &mut wires, &mut values);
        wires[2] = Some(Wire::Const(Operand::Const(values[1].unwrap())));
        let mut values = [None; 676];
        resolve(1, &mut wires, &mut values);
        values[1].unwrap()
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_07_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    println!("Parsed in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part1::solve(&puzzle_input, parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(&puzzle_input, parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
