//! https://adventofcode.com/2017/day/22
//! https://adventofcode.com/2017/day/22/input

use std::{collections::HashSet, fs::read_to_string, time::Instant};

type Coord = (isize, isize);

fn parse(input: &str) -> (HashSet<Coord>, Coord) {
    let mut result = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, node) in line.chars().enumerate() {
            if node == '#' {
                result.insert((i as isize, j as isize));
            }
        }
    }
    (
        result,
        (
            (input.lines().count() / 2) as isize,
            (input.lines().next().unwrap().chars().count() / 2) as isize,
        ),
    )
}

pub mod part1 {
    use crate::day_22::parse;

    pub fn solve(input: &str) -> usize {
        let (mut infected, (mut vi, mut vj)) = parse(input);
        let (mut di, mut dj) = (-1, 0);
        let mut infections = 0;
        for _ in 0..10000 {
            if infected.contains(&(vi, vj)) {
                (di, dj) = (dj, -di);
                infected.remove(&(vi, vj));
            } else {
                (di, dj) = (-dj, di);
                infected.insert((vi, vj));
                infections += 1;
            }
            (vi, vj) = (vi + di, vj + dj);
        }
        infections
    }
}

pub mod part2 {
    use std::collections::{hash_map::Entry, HashMap};

    use crate::day_22::parse;

    #[derive(Copy, Clone, Debug)]
    enum State {
        Weakened,
        Infected,
        Flagged,
    }

    pub fn solve(input: &str) -> usize {
        let (infected, (mut vi, mut vj)) = parse(input);
        let mut infected: HashMap<_, _> = infected
            .iter()
            .map(|&coord| (coord, State::Infected))
            .collect();
        let (mut di, mut dj) = (-1, 0);
        let mut infections = 0;
        for _ in 0..10_000_000 {
            match infected.entry((vi, vj)) {
                Entry::Occupied(mut entry) => match entry.get() {
                    State::Weakened => {
                        entry.insert(State::Infected);
                        infections += 1;
                    }
                    State::Infected => {
                        (di, dj) = (dj, -di);
                        entry.insert(State::Flagged);
                    }
                    State::Flagged => {
                        (di, dj) = (-di, -dj);
                        entry.remove();
                    }
                },
                Entry::Vacant(clean) => {
                    (di, dj) = (-dj, di);
                    clean.insert(State::Weakened);
                }
            }
            (vi, vj) = (vi + di, vj + dj);
        }
        infections
    }
}

pub fn main(test: bool) {
    let test_input = "..#
#..
..."
    .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_22_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
