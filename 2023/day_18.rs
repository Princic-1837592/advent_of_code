//! https://adventofcode.com/2023/day/18
//! https://adventofcode.com/2023/day/18/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone)]
enum Direction {
    U,
    R,
    D,
    L,
}

impl Direction {
    fn to_tuple(self) -> (isize, isize) {
        match self {
            Direction::U => (-1, 0),
            Direction::R => (0, 1),
            Direction::D => (1, 0),
            Direction::L => (0, -1),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Self::U,
            'R' => Self::R,
            'D' => Self::D,
            'L' => Self::L,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
pub struct Dig {
    direction: Direction,
    distance: usize,
    color: usize,
}

impl From<&str> for Dig {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        Self {
            direction: Direction::from(parts.next().unwrap().chars().next().unwrap()),
            distance: parts.next().unwrap().parse().unwrap(),
            color: {
                let color = parts.next().unwrap();
                usize::from_str_radix(&color[2..color.len() - 1], 16).unwrap()
            },
        }
    }
}

type Parsed = Vec<Dig>;

fn parse(input: &str) -> Parsed {
    input.lines().map(Dig::from).collect()
}

pub fn solve(instructions: Parsed) -> usize {
    let (mut i, mut j) = (0, 0);
    let mut corners = vec![];
    let mut perimeter = 0;
    for Dig {
        direction,
        distance,
        ..
    } in instructions
    {
        let (di, dj) = direction.to_tuple();
        i += di * distance as isize;
        j += dj * distance as isize;
        corners.push((i, j));
        perimeter += distance;
    }
    let mut area = 0;
    for i in 0..corners.len() {
        area += corners[i].1 * corners[(i + 1) % corners.len()].0
            - corners[(i + 1) % corners.len()].1 * corners[i].0;
    }
    area /= 2;
    area as usize + perimeter / 2 + 1
}

pub mod part1 {
    use super::Parsed;

    pub fn solve(instructions: Parsed) -> usize {
        super::solve(instructions)
    }
}

pub mod part2 {
    use super::{Dig, Direction, Parsed};

    pub fn solve(instructions: Parsed) -> usize {
        let instructions: Vec<_> = instructions
            .iter()
            .map(|&Dig { color, .. }| Dig {
                direction: match color % 16 {
                    0 => Direction::R,
                    1 => Direction::D,
                    2 => Direction::L,
                    3 => Direction::U,
                    _ => unreachable!(),
                },
                distance: color / 16,
                color: 0,
            })
            .collect();
        super::solve(instructions)
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_18_input.txt").unwrap()
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
