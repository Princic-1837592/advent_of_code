//! https://adventofcode.com/2023/day/17
//! https://adventofcode.com/2023/day/17/input

use std::{
    collections::BinaryHeap,
    fs::read_to_string,
    time::{Duration, Instant},
};

use utils::{coords::Direction, new, IntoEnumIterator};

type Parsed = Vec<Vec<usize>>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| l.chars().map(|c| c as usize - '0' as usize).collect())
        .collect()
}

type Coord = (usize, usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, new)]
struct State {
    position: Coord,
    direction: Direction,
    steps: usize,
    heat_loss: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone)]
struct Visited {
    visited: bool,
    heat_loss: usize,
}

// https://github.com/ranjeethmahankali/adventofcode/blob/b2cac5e9ca03d2ef6d18a5b81eecfb3c9e0f5b32/2023/src/day_17.rs#L125
fn solve_generic<const MIN: usize, const MAX: usize>(map: Parsed) -> usize {
    let (rows, cols) = (map.len(), map[0].len());
    let mut queue = BinaryHeap::<State>::new();
    let mut history = vec![
        vec![
            vec![
                [Visited {
                    visited: false,
                    heat_loss: usize::MAX
                }; 4];
                MAX + 1
            ];
            cols
        ];
        rows
    ];
    queue.push(State {
        position: (0, 0),
        direction: Direction::S,
        steps: 0,
        heat_loss: 0,
    });
    queue.push(State {
        position: (0, 0),
        direction: Direction::E,
        steps: 0,
        heat_loss: 0,
    });
    while let Some(State {
        position: (i, j),
        direction,
        steps,
        heat_loss,
    }) = queue.pop()
    {
        history[i][j][steps][direction as usize].visited = true;
        for d in Direction::iter() {
            let (same_dir, opp_dir) = (direction == d, direction.opposite() == d);
            if (steps < MIN && !same_dir)
                || (steps >= MAX && same_dir)
                || opp_dir
                || match d {
                    Direction::N => i == 0,
                    Direction::E => j == cols - 1,
                    Direction::S => i == rows - 1,
                    Direction::W => j == 0,
                }
            {
                continue;
            }
            let next_pos @ (ni, nj) = match d {
                Direction::N => (i - 1, j),
                Direction::E => (i, j + 1),
                Direction::S => (i + 1, j),
                Direction::W => (i, j - 1),
            };
            let next_steps = if same_dir { steps + 1 } else { 1 };
            let next_heat_loss = heat_loss + map[ni][nj];
            let Visited {
                visited,
                heat_loss: prev_heat_loss,
            } = history[ni][nj][next_steps][d as usize];
            if visited || next_heat_loss >= prev_heat_loss {
                continue;
            }
            history[ni][nj][next_steps][d as usize].heat_loss = next_heat_loss;
            queue.push(State::new(next_pos, d, next_steps, next_heat_loss))
        }
    }
    history[rows - 1][cols - 1]
        .iter()
        .flatten()
        .map(|visited| visited.heat_loss)
        .min()
        .unwrap()
}

pub mod part1 {
    use super::{solve_generic, Parsed};

    pub fn solve(map: Parsed) -> usize {
        solve_generic::<1, 3>(map)
    }
}

pub mod part2 {
    use super::{solve_generic, Parsed};

    pub fn solve(map: Parsed) -> usize {
        solve_generic::<4, 10>(map)
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_17_input.txt").unwrap()
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
