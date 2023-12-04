//! https://adventofcode.com/2017/day/11
//! https://adventofcode.com/2017/day/11/input

use std::{fs::read_to_string, time::Instant};

enum Direction {
    N,
    S,
    Ne,
    Sw,
    Nw,
    Se,
}

impl Direction {
    fn to_coord(&self) -> (isize, isize, isize) {
        match self {
            Direction::N => (0, 1, -1),
            Direction::Ne => (-1, 1, 0),
            Direction::Se => (-1, 0, 1),
            Direction::S => (0, -1, 1),
            Direction::Sw => (1, -1, 0),
            Direction::Nw => (1, 0, -1),
        }
    }
}

impl From<&str> for Direction {
    fn from(string: &str) -> Self {
        match string {
            "n" => Self::N,
            "ne" => Self::Ne,
            "nw" => Self::Nw,
            "s" => Self::S,
            "se" => Self::Se,
            "sw" => Self::Sw,
            _ => panic!("Invalid direction: {}", string),
        }
    }
}

fn parse(input: &str) -> Vec<Direction> {
    input.split(',').map(Direction::from).collect()
}

pub mod part1 {
    use super::{parse, Direction};

    pub fn solve(input: &str) -> usize {
        let directions = parse(input);
        let (mut x, mut y, mut z) = (0, 0, 0);
        for (dx, dy, dz) in directions.iter().map(Direction::to_coord) {
            x += dx;
            y += dy;
            z += dz;
        }
        (x.unsigned_abs() + y.unsigned_abs() + z.unsigned_abs()) / 2
    }
}

pub mod part2 {
    use super::{parse, Direction};

    pub fn solve(input: &str) -> usize {
        let directions = parse(input);
        let (mut x, mut y, mut z) = (0, 0, 0);
        let mut max_distance = 0;
        for (dx, dy, dz) in directions.iter().map(Direction::to_coord) {
            x += dx;
            y += dy;
            z += dz;
            max_distance = max_distance.max((x.abs() + y.abs() + z.abs()) / 2)
        }
        max_distance as usize
    }
}

pub fn main(test: bool) {
    let test_input = "ne,ne,sw,sw".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_11_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
