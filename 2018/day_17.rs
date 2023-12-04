//! https://adventofcode.com/2018/day/17
//! https://adventofcode.com/2018/day/17/input

use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
};

type Coord = (usize, usize);

#[derive(Copy, Clone, Debug)]
enum WaterState {
    Still = '~' as isize,
    Flowing = '|' as isize,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> (HashSet<Coord>, usize, usize) {
    let mut result = HashSet::new();
    for line in input.lines() {
        let mut parts = line.split(", ");
        let mut left = parts.next().unwrap().split('=');
        let mut right = parts.next().unwrap().split('=').nth(1).unwrap().split("..");
        let from = right.next().unwrap().parse().unwrap();
        let to = right.next().unwrap().parse().unwrap();
        if left.next().unwrap().starts_with('x') {
            let mut coord = (left.next().unwrap().parse().unwrap(), 0);
            for c in from..=to {
                coord.1 = c;
                result.insert(coord);
            }
        } else {
            let mut coord = (0, left.next().unwrap().parse().unwrap());
            for c in from..=to {
                coord.0 = c;
                result.insert(coord);
            }
        }
    }
    let min_y = *result.iter().map(|(_, y)| y).min().unwrap();
    let max_y = *result.iter().map(|(_, y)| y).max().unwrap();
    (result, min_y, max_y)
}

#[allow(unused)]
fn to_string(clay: &HashSet<Coord>, explored: &HashMap<Coord, WaterState>) -> String {
    let (min_x, max_x, min_y, max_y) = clay.iter().fold(
        (usize::MAX, 0, usize::MAX, 0),
        |(min_x, max_x, min_y, max_y), &(x, y)| {
            (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
        },
    );
    let (min_x, max_x, min_y, max_y) = explored.keys().fold(
        (min_x, max_x, min_y, max_y),
        |(min_x, max_x, min_y, max_y), &(x, y)| {
            (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
        },
    );
    let mut matrix = vec![vec!['.'; max_x - min_x + 1]; max_y - min_y + 1];
    for (x, y) in clay {
        matrix[*y - min_y][x - min_x] = '#';
    }
    for ((x, y), state) in explored {
        matrix[*y - min_y][x - min_x] = *state as u8 as char;
    }
    matrix[0][500 - min_x] = '+';
    matrix
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn dfs(
    clay: &HashSet<Coord>,
    coord @ (x, y): Coord,
    max_y: usize,
    direction: Direction,
    explored: &mut HashMap<Coord, WaterState>,
) -> WaterState {
    if clay.contains(&coord) {
        return WaterState::Still;
    }
    if let Some(state) = explored.get(&coord) {
        return *state;
    }
    if y > max_y {
        return WaterState::Flowing;
    }
    let down = dfs(clay, (x, y + 1), max_y, Direction::Down, explored);
    let result = match down {
        WaterState::Still => match direction {
            Direction::Down => {
                let left = dfs(clay, (x - 1, y), max_y, Direction::Left, explored);
                let right = dfs(clay, (x + 1, y), max_y, Direction::Right, explored);
                match (left, right) {
                    (WaterState::Still, WaterState::Still) => WaterState::Still,
                    (WaterState::Flowing, WaterState::Still) => {
                        let mut right = (x + 1, y);
                        while let Entry::Occupied(mut e) = explored.entry(right) {
                            e.insert(WaterState::Flowing);
                            right.0 += 1;
                        }
                        WaterState::Flowing
                    }
                    (WaterState::Still, WaterState::Flowing) => {
                        let mut left = (x - 1, y);
                        while let Entry::Occupied(mut e) = explored.entry(left) {
                            e.insert(WaterState::Flowing);
                            left.0 -= 1;
                        }
                        WaterState::Flowing
                    }
                    _ => WaterState::Flowing,
                }
            }
            Direction::Left => dfs(clay, (x - 1, y), max_y, Direction::Left, explored),
            Direction::Right => dfs(clay, (x + 1, y), max_y, Direction::Right, explored),
        },
        WaterState::Flowing => WaterState::Flowing,
    };
    explored.insert(coord, result);
    result
}

pub mod part1 {
    use std::collections::HashMap;

    use super::{dfs, parse, Direction};

    pub fn solve(input: &str) -> usize {
        let (clay, min_y, max_y) = parse(input);
        let mut explored = HashMap::new();
        dfs(&clay, (500, 0), max_y, Direction::Down, &mut explored);
        explored.len() - min_y
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use super::{dfs, parse, Direction, WaterState};

    pub fn solve(input: &str) -> usize {
        let (clay, _, max_y) = parse(input);
        let mut explored = HashMap::new();
        dfs(&clay, (500, 0), max_y, Direction::Down, &mut explored);
        explored
            .values()
            .filter(|state| matches!(state, WaterState::Still))
            .count()
    }
}

pub fn main(test: bool) {
    let test_input = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_17_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
