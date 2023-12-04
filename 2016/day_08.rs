//! https://adventofcode.com/2016/day/8
//! https://adventofcode.com/2016/day/8/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
enum Operation {
    Rect,
    Row,
    Column,
}

impl From<&str> for Operation {
    fn from(string: &str) -> Self {
        if string.starts_with("rect") {
            Self::Rect
        } else if string.starts_with("rotate row") {
            Self::Row
        } else if string.starts_with("rotate column") {
            Self::Column
        } else {
            panic!("Invalid operation: {}", string)
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction {
    operation: Operation,
    a: usize,
    b: usize,
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let operation = Operation::from(string);
        match operation {
            Operation::Rect => {
                let mut parts = string.split_whitespace();
                let mut operands = parts.nth(1).unwrap().split('x');
                let a = operands.next().unwrap().parse().unwrap();
                let b = operands.next().unwrap().parse().unwrap();
                Instruction { operation, a, b }
            }
            _ => {
                let mut parts = string.split_whitespace();
                let a = parts
                    .nth(2)
                    .unwrap()
                    .split('=')
                    .nth(1)
                    .unwrap()
                    .parse()
                    .unwrap();
                let b = parts.nth(1).unwrap().parse().unwrap();
                Instruction { operation, a, b }
            }
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

pub mod part1 {
    use super::{parse, Instruction, Operation};

    pub fn solve(input: &str, width: usize, height: usize) -> usize {
        let instructions = parse(input);
        let mut screen = vec![vec![0; width]; height];
        for Instruction { operation, a, b } in instructions {
            match operation {
                Operation::Rect => {
                    for row in &mut screen[0..b] {
                        for pixel in &mut row[0..a] {
                            *pixel = 1;
                        }
                    }
                }
                Operation::Row => {
                    screen[a].rotate_right(b);
                }
                Operation::Column => {
                    let mut new_col = vec![0; screen.len()];
                    for (i, row) in screen.iter().enumerate() {
                        new_col[(i + b) % screen.len()] = row[a]
                    }
                    for (i, &pixel) in new_col.iter().enumerate() {
                        screen[i][a] = pixel;
                    }
                }
            }
        }
        for row in &screen {
            for &pixel in row {
                print!("{}", if pixel == 1 { '#' } else { '.' });
            }
            println!();
        }
        screen.iter().flatten().sum()
    }
}

pub fn main(test: bool) {
    let test_input = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1"
        .to_owned();
    let (puzzle_input, w, h) = if test {
        (test_input, 7, 3)
    } else {
        (read_to_string("inputs/day_08_input.txt").unwrap(), 50, 6)
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input, w, h));
    println!("Run in {:?}", start.elapsed());
}
