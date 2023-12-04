//! https://adventofcode.com/2019/day/24
//! https://adventofcode.com/2019/day/24/input

use std::{fs::read_to_string, time::Instant};

const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

#[derive(Copy, Clone, Debug)]
enum State {
    Bug,
    Empty,
}

fn parse(input: &str) -> [[State; 5]; 5] {
    let mut grid = [[State::Empty; 5]; 5];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[y][x] = match c {
                '#' => State::Bug,
                '.' => State::Empty,
                _ => panic!("Invalid character"),
            };
        }
    }
    grid
}

pub mod part1 {
    use std::{collections::HashSet, mem::swap};

    use super::{parse, State, NEIGHBORS};

    fn biodiversity_rating(eris: &[[State; 5]; 5]) -> usize {
        let mut po2 = 1;
        let mut rating = 0;
        for row in eris {
            for state in row {
                if let State::Bug = state {
                    rating += po2;
                }
                po2 *= 2;
            }
        }
        rating
    }

    fn count_surrounding_bugs(eris: &[[State; 5]; 5], i: usize, j: usize) -> usize {
        let mut count = 0;
        for (di, dj) in NEIGHBORS.iter() {
            let i = (i as isize + di) as usize;
            let j = (j as isize + dj) as usize;
            if i >= eris.len() || j >= eris[i].len() {
                continue;
            }
            if let State::Bug = eris[i][j] {
                count += 1;
            }
        }
        count
    }

    fn step(eris: &mut [[State; 5]; 5], support: &mut [[State; 5]; 5]) {
        for i in 0..eris.len() {
            for j in 0..eris[i].len() {
                let surrounding = count_surrounding_bugs(eris, i, j);
                match eris[i][j] {
                    State::Bug => {
                        support[i][j] = if surrounding != 1 {
                            State::Empty
                        } else {
                            State::Bug
                        }
                    }
                    State::Empty => {
                        support[i][j] = if surrounding == 1 || surrounding == 2 {
                            State::Bug
                        } else {
                            State::Empty
                        }
                    }
                }
            }
        }
        swap(eris, support);
    }

    pub fn solve(input: &str) -> usize {
        let mut eris = parse(input);
        let mut seen = HashSet::new();
        let mut support = [[State::Empty; 5]; 5];
        loop {
            let rating = biodiversity_rating(&eris);
            if seen.contains(&rating) {
                return rating;
            }
            seen.insert(rating);
            step(&mut eris, &mut support);
        }
    }
}

pub mod part2 {
    use std::{collections::HashSet, mem::swap};

    use super::{parse, State, NEIGHBORS};

    type Coord = (isize, usize, usize);

    fn near((level, i, j): Coord) -> Vec<Coord> {
        match (i, j) {
            (1, 1) | (1, 3) | (3, 1) | (3, 3) => NEIGHBORS
                .iter()
                .map(|(di, dj)| {
                    (
                        level,
                        (i as isize + di) as usize,
                        (j as isize + dj) as usize,
                    )
                })
                .collect(),
            (0, 0) => vec![
                (level, 0, 1),
                (level, 1, 0),
                (level - 1, 2, 1),
                (level - 1, 1, 2),
            ],
            (0, 4) => vec![
                (level, 0, 3),
                (level, 1, 4),
                (level - 1, 2, 3),
                (level - 1, 1, 2),
            ],
            (4, 0) => vec![
                (level, 3, 0),
                (level, 4, 1),
                (level - 1, 2, 1),
                (level - 1, 3, 2),
            ],
            (4, 4) => vec![
                (level, 3, 4),
                (level, 4, 3),
                (level - 1, 2, 3),
                (level - 1, 3, 2),
            ],
            (1, 0) | (2, 0) | (3, 0) => vec![
                (level, i - 1, j),
                (level, i, j + 1),
                (level, i + 1, j),
                (level - 1, 2, 1),
            ],
            (1, 4) | (2, 4) | (3, 4) => vec![
                (level, i - 1, j),
                (level, i, j - 1),
                (level, i + 1, j),
                (level - 1, 2, 3),
            ],
            (0, 1) | (0, 2) | (0, 3) => vec![
                (level, i, j - 1),
                (level, i + 1, j),
                (level, i, j + 1),
                (level - 1, 1, 2),
            ],
            (4, 1) | (4, 2) | (4, 3) => vec![
                (level, i, j - 1),
                (level, i - 1, j),
                (level, i, j + 1),
                (level - 1, 3, 2),
            ],
            (1, 2) => vec![
                (level, i, j - 1),
                (level, i - 1, j),
                (level, i, j + 1),
                (level + 1, 0, 0),
                (level + 1, 0, 1),
                (level + 1, 0, 2),
                (level + 1, 0, 3),
                (level + 1, 0, 4),
            ],
            (2, 1) => vec![
                (level, i - 1, j),
                (level, i, j - 1),
                (level, i + 1, j),
                (level + 1, 0, 0),
                (level + 1, 1, 0),
                (level + 1, 2, 0),
                (level + 1, 3, 0),
                (level + 1, 4, 0),
            ],
            (2, 3) => vec![
                (level, i - 1, j),
                (level, i, j + 1),
                (level, i + 1, j),
                (level + 1, 0, 4),
                (level + 1, 1, 4),
                (level + 1, 2, 4),
                (level + 1, 3, 4),
                (level + 1, 4, 4),
            ],
            (3, 2) => vec![
                (level, i, j - 1),
                (level, i + 1, j),
                (level, i, j + 1),
                (level + 1, 4, 0),
                (level + 1, 4, 1),
                (level + 1, 4, 2),
                (level + 1, 4, 3),
                (level + 1, 4, 4),
            ],
            _ => unreachable!(),
        }
    }

    fn count_surrounding_bugs(bugs: &HashSet<Coord>, coord: Coord) -> usize {
        near(coord)
            .iter()
            .filter(|coord| bugs.contains(coord))
            .count()
    }

    fn step(bugs: &mut HashSet<Coord>) {
        let mut supports = bugs.clone();
        for &coord in bugs.iter() {
            for near @ (level, ni, nj) in near(coord) {
                let surrounding = count_surrounding_bugs(bugs, near);
                if surrounding == 1 || surrounding == 2 {
                    supports.insert((level, ni, nj));
                }
            }
        }
        for &coord @ (level, i, j) in bugs.iter() {
            let surrounding = count_surrounding_bugs(bugs, coord);
            if surrounding != 1 {
                supports.remove(&(level, i, j));
            }
        }
        swap(bugs, &mut supports);
    }

    pub fn solve(input: &str) -> usize {
        let eris = parse(input);
        let mut bugs = HashSet::new();
        for (i, row) in eris.iter().enumerate() {
            for (j, state) in row.iter().enumerate() {
                if let State::Bug = state {
                    bugs.insert((0, i, j));
                }
            }
        }
        for _ in 0..200 {
            step(&mut bugs);
        }
        bugs.len()
    }
}

pub fn main(test: bool) {
    let test_input = "....#
#..#.
#..##
..#..
#...."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_24_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
