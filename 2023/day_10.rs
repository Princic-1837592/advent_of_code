//! https://adventofcode.com/2023/day/10
//! https://adventofcode.com/2023/day/10/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug)]
pub enum Pipe {
    NS,    // |
    EW,    // -
    NE,    // L
    NW,    // J
    SW,    // 7
    SE,    // F
    Empty, // .
    Start, // S
}

impl Pipe {
    fn open_west(&self) -> bool {
        match self {
            Pipe::NS | Pipe::NE | Pipe::SE | Pipe::Empty => false,
            Pipe::EW | Pipe::SW | Pipe::NW | Pipe::Start => true,
        }
    }

    fn open_east(&self) -> bool {
        match self {
            Pipe::EW | Pipe::NE | Pipe::SE | Pipe::Start => true,
            Pipe::NS | Pipe::SW | Pipe::NW | Pipe::Empty => false,
        }
    }

    fn open_north(&self) -> bool {
        match self {
            Pipe::NS | Pipe::NE | Pipe::NW | Pipe::Start => true,
            Pipe::EW | Pipe::SW | Pipe::SE | Pipe::Empty => false,
        }
    }

    fn open_south(&self) -> bool {
        match self {
            Pipe::EW | Pipe::NE | Pipe::NW | Pipe::Empty => false,
            Pipe::NS | Pipe::SW | Pipe::SE | Pipe::Start => true,
        }
    }

    fn open(&self, direction: Direction) -> bool {
        match direction {
            Direction::N => self.open_north(),
            Direction::E => self.open_east(),
            Direction::S => self.open_south(),
            Direction::W => self.open_west(),
        }
    }

    fn other(&self, direction: Direction) -> Direction {
        match (self, direction) {
            (Pipe::NS, Direction::N) => Direction::S,
            (Pipe::NS, Direction::S) => Direction::N,
            (Pipe::EW, Direction::W) => Direction::E,
            (Pipe::EW, Direction::E) => Direction::W,
            (Pipe::NE, Direction::E) => Direction::N,
            (Pipe::NE, Direction::N) => Direction::E,
            (Pipe::NW, Direction::N) => Direction::W,
            (Pipe::NW, Direction::W) => Direction::N,
            (Pipe::SW, Direction::W) => Direction::S,
            (Pipe::SW, Direction::S) => Direction::W,
            (Pipe::SE, Direction::E) => Direction::S,
            (Pipe::SE, Direction::S) => Direction::E,
            (Pipe::Empty, _) => unreachable!(),
            (Pipe::Start, _) => Direction::N,
            _ => unreachable!(),
        }
    }
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::NS,
            '-' => Self::EW,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            'S' => Self::Start,
            '.' => Self::Empty,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn transform(&self, i: usize, j: usize, h: usize, w: usize) -> Option<Coord> {
        match self {
            Direction::N => i.checked_sub(1).map(|i| (i, j)),
            Direction::E => (j + 1 < w).then_some((i, j + 1)),
            Direction::S => (i + 1 < w).then_some((i + 1, j)),
            Direction::W => j.checked_sub(1).map(|j| (i, j)),
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [Direction::S, Direction::N, Direction::W, Direction::E];

type Pipes = Vec<Vec<Pipe>>;

type Coord = (usize, usize);

type Parsed = (Pipes, Coord);

fn parse(input: &str) -> Parsed {
    let result: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(Pipe::from).collect())
        .collect();
    for (i, row) in result.iter().enumerate() {
        for (j, pipe) in row.iter().enumerate() {
            if let Pipe::Start = pipe {
                return (result, (i, j));
            }
        }
    }
    unreachable!()
}

pub mod part1 {
    use super::{Direction, Parsed, Pipes, DIRECTIONS};

    fn explore(pipes: &Pipes, oi: usize, oj: usize, mut direction: Direction) -> Option<usize> {
        let (mut i, mut j) = (oi, oj);
        let mut length = 0;
        let (h, w) = (pipes.len(), pipes[0].len());
        while (i, j) != (oi, oj) || length == 0 {
            let (ni, nj) = match direction.transform(i, j, h, w) {
                None => return None,
                Some(n) => n,
            };
            let nd = direction.reverse();
            if !pipes[ni][nj].open(nd) {
                return None;
            }
            (i, j) = (ni, nj);
            direction = pipes[i][j].other(nd);
            length += 1;
        }
        Some(length)
    }

    pub fn solve((pipes, (i, j)): Parsed) -> usize {
        for direction in DIRECTIONS {
            if let Some(length) = explore(&pipes, i, j, direction) {
                return length / 2;
            }
        }
        unreachable!()
    }
}

pub mod part2 {
    use super::Parsed;

    pub fn solve(_parsed: Parsed) -> usize {
        0
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_10_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    if verbose {
        println!("Parsed in {:?}", elapsed);
        total += elapsed;
    }

    let start = Instant::now();
    let result = part1::solve(parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    if verbose {
        println!("Total {:?}", total);
    }
    total
}
