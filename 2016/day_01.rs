//! https://adventofcode.com/2016/day/1
//! https://adventofcode.com/2016/day/1/input

use std::{fs::read_to_string, time::Instant};

enum Rotation {
    R,
    L,
}

struct Instruction {
    rotation: Rotation,
    steps: isize,
}

impl From<char> for Rotation {
    fn from(char: char) -> Self {
        match char {
            'R' => Self::R,
            'L' => Self::L,
            _ => panic!("Invalid direction: {}", char),
        }
    }
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        Instruction {
            rotation: Rotation::from(string[0..1].chars().next().unwrap()),
            steps: string[1..].parse().unwrap(),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.split(", ").map(Instruction::from).collect()
}

pub mod part1 {
    use super::{parse, Instruction, Rotation};

    pub fn solve(input: &str) -> usize {
        let instructions = parse(input);
        let (mut dx, mut dy) = (-1, 0);
        let (mut x, mut y) = (0, 0);
        for Instruction { rotation, steps } in instructions {
            match rotation {
                Rotation::R => (dx, dy) = (dy, -dx),
                Rotation::L => (dx, dy) = (-dy, dx),
            }
            x += dx * steps;
            y += dy * steps;
        }
        x.unsigned_abs() + y.unsigned_abs()
    }
}

pub mod part2 {
    use std::collections::HashSet;

    use super::{parse, Instruction, Rotation};

    pub fn solve(input: &str) -> usize {
        let instructions = parse(input);
        let (mut dx, mut dy): (isize, _) = (-1, 0);
        let (mut x, mut y) = (0, 0);
        let mut visited = HashSet::new();
        for Instruction { rotation, steps } in instructions {
            match rotation {
                Rotation::R => (dx, dy) = (dy, -dx),
                Rotation::L => (dx, dy) = (-dy, dx),
            }
            for _ in 0..steps {
                if visited.contains(&(x, y)) {
                    break;
                }
                visited.insert((x, y));
                x += dx;
                y += dy;
            }
        }
        x.unsigned_abs() + y.unsigned_abs()
    }
}

pub fn main(test: bool) {
    let test_input = "R2, R2, R2".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_01_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
