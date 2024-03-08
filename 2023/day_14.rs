//! https://adventofcode.com/2023/day/14
//! https://adventofcode.com/2023/day/14/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Rock {
    Round,
    Cube,
    Empty,
}

impl From<char> for Rock {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Cube,
            '.' => Self::Empty,
            'O' => Self::Round,
            _ => unreachable!(),
        }
    }
}

type Parsed = Vec<Vec<Rock>>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| line.chars().map(Rock::from).collect())
        .collect()
}

pub mod part1 {
    use super::{Parsed, Rock};

    pub fn solve(mut rocks: Parsed) -> usize {
        let mut load = 0;
        let h = rocks.len();
        for j in 0..rocks[0].len() {
            let mut last_free: Option<usize> = None;
            for i in 0..rocks.len() {
                match rocks[i][j] {
                    Rock::Round => {
                        if let Some(free) = last_free {
                            rocks[free][j] = Rock::Round;
                            rocks[i][j] = Rock::Empty;
                            last_free = Some(free + 1);
                            load += h - free;
                        } else {
                            load += h - i;
                        }
                    }
                    Rock::Cube => last_free = None,
                    Rock::Empty if last_free.is_none() => last_free = Some(i),
                    _ => {}
                }
            }
        }
        load
    }
}

pub mod part2 {
    use std::collections::{hash_map::Entry, HashMap};

    use utils::matrix::rotate_right;

    use super::{Parsed, Rock};

    fn move_north(rocks: &mut Parsed) {
        for j in 0..rocks[0].len() {
            let mut last_free: Option<usize> = None;
            for i in 0..rocks.len() {
                match rocks[i][j] {
                    Rock::Round => {
                        if let Some(free) = last_free {
                            rocks[free][j] = Rock::Round;
                            rocks[i][j] = Rock::Empty;
                            last_free = Some(free + 1);
                        }
                    }
                    Rock::Cube => last_free = None,
                    Rock::Empty if last_free.is_none() => last_free = Some(i),
                    _ => {}
                }
            }
        }
    }

    fn compute_load(rocks: &Parsed) -> usize {
        let h = rocks.len();
        let mut load = 0;
        for (i, row) in rocks.iter().enumerate() {
            for &rock in row {
                if rock == Rock::Round {
                    load += h - i;
                }
            }
        }
        load
    }

    pub fn solve(mut rocks: Parsed) -> usize {
        let mut seen = HashMap::new();
        let mut c = 0;
        let target = 1_000_000_000;
        while c < target {
            match seen.entry(rocks.clone()) {
                Entry::Occupied(entry) => {
                    let last = entry.get();
                    let interval = c - last;
                    let left = target - c;
                    let cycles = left / interval;
                    c += cycles * interval;
                }
                Entry::Vacant(entry) => {
                    entry.insert(c);
                }
            }
            move_north(&mut rocks);
            rocks = rotate_right(&rocks);
            move_north(&mut rocks);
            rocks = rotate_right(&rocks);
            move_north(&mut rocks);
            rocks = rotate_right(&rocks);
            move_north(&mut rocks);
            rocks = rotate_right(&rocks);
            c += 1;
        }
        compute_load(&rocks)
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_14_input.txt").unwrap()
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
