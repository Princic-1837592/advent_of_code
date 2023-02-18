//! https://adventofcode.com/2017/day/11
//! https://adventofcode.com/2017/day/11/input

use std::{fs::read_to_string, time::Instant};

const DIRECTIONS: [Direction; 6] = [
    Direction::N,
    Direction::S,
    Direction::Ne,
    Direction::Sw,
    Direction::Nw,
    Direction::Se,
];

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
    use std::collections::{hash_map::Entry, HashMap, VecDeque};

    use crate::day_11::{parse, Direction, DIRECTIONS};

    pub fn solve(input: &str) -> usize {
        let directions = parse(input);
        let (mut x, mut y, mut z) = (0, 0, 0);
        for (dx, dy, dz) in directions.iter().map(Direction::to_coord) {
            x += dx;
            y += dy;
            z += dz;
        }
        let target = (x, y, z);
        let mut queue = VecDeque::from([(0, 0, 0)]);
        let mut distances = HashMap::from([((0, 0, 0), 0)]);
        while let Some(hex @ (x, y, z)) = queue.pop_front() {
            if hex == target {
                break;
            }
            let distance = *distances.get(&hex).unwrap();
            for (dx, dy, dz) in DIRECTIONS.iter().map(Direction::to_coord) {
                let next = (x + dx, y + dy, z + dz);
                if let Entry::Vacant(e) = distances.entry(next) {
                    e.insert(distance + 1);
                    queue.push_back(next);
                }
            }
        }
        *distances.get(&target).unwrap()
    }
}

pub mod part2 {
    use std::collections::{hash_map::Entry, HashMap, HashSet, VecDeque};

    use crate::day_11::{parse, Direction, DIRECTIONS};

    pub fn solve(input: &str) -> usize {
        let directions = parse(input);
        let (mut x, mut y, mut z) = (0, 0, 0);
        let mut visited = HashSet::new();
        for (dx, dy, dz) in directions.iter().map(Direction::to_coord) {
            x += dx;
            y += dy;
            z += dz;
            visited.insert((x, y, z));
        }
        let mut visited_clone = visited.clone();
        let mut queue = VecDeque::from([(0, 0, 0)]);
        let mut distances = HashMap::from([((0, 0, 0), 0)]);
        while let Some(hex @ (x, y, z)) = queue.pop_front() {
            if visited_clone.contains(&hex) {
                visited_clone.remove(&hex);
            }
            if visited_clone.is_empty() {
                break;
            }
            let distance = *distances.get(&hex).unwrap();
            for (dx, dy, dz) in DIRECTIONS.iter().map(Direction::to_coord) {
                let next = (x + dx, y + dy, z + dz);
                if let Entry::Vacant(e) = distances.entry(next) {
                    e.insert(distance + 1);
                    queue.push_back(next);
                }
            }
        }
        *distances
            .iter()
            .filter(|(hex, _)| visited.contains(hex))
            .max_by_key(|(_, &d)| d)
            .unwrap()
            .1
    }
}

pub fn main(test: bool) {
    let test_input = "ne,ne,s,s".to_owned();
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
