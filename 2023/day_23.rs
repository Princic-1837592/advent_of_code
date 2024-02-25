//! https://adventofcode.com/2023/day/23
//! https://adventofcode.com/2023/day/23/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Path,
    Forest,
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Forest,
            '.' => Self::Path,
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

type Parsed = Vec<Vec<Cell>>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| l.chars().map(Cell::from).collect())
        .collect()
}

pub mod part1 {
    use std::collections::VecDeque;

    use super::{Cell, Parsed};

    pub fn solve(map: Parsed) -> usize {
        let (h, w) = (map.len(), map[0].len());
        let mut queue = VecDeque::from([((0, 0), (0, 1), 1)]);
        let mut seen = vec![vec![0; w]; h];
        let mut result = 0;
        while let Some((from, coord @ (i, j), steps)) = queue.pop_front() {
            if steps <= seen[i][j] {
                continue;
            }
            seen[i][j] = steps;
            if coord == (h - 1, w - 2) {
                result = result.max(steps);
            }
            match map[i][j] {
                Cell::Path => {
                    let to @ (ni, nj) = (i.wrapping_sub(1), j);
                    if ni < h && !matches!(map[ni][nj], Cell::Forest | Cell::Down) && to != from {
                        queue.push_back((coord, to, steps + 1));
                    }
                    let to @ (ni, nj) = (i + 1, j);
                    if ni < h && !matches!(map[ni][nj], Cell::Forest | Cell::Up) && to != from {
                        queue.push_back((coord, to, steps + 1));
                    }
                    let to @ (ni, nj) = (i, j.wrapping_sub(1));
                    if ni < h && !matches!(map[ni][nj], Cell::Forest | Cell::Right) && to != from {
                        queue.push_back((coord, to, steps + 1));
                    }
                    let to @ (ni, nj) = (i, j + 1);
                    if ni < h && !matches!(map[ni][nj], Cell::Forest | Cell::Left) && to != from {
                        queue.push_back((coord, to, steps + 1));
                    }
                }
                Cell::Forest => unreachable!(),
                Cell::Up => queue.push_back((coord, (i - 1, j), steps + 1)),
                Cell::Right => queue.push_back((coord, (i, j + 1), steps + 1)),
                Cell::Down => queue.push_back((coord, (i + 1, j), steps + 1)),
                Cell::Left => queue.push_back((coord, (i, j - 1), steps + 1)),
            }
        }
        result - 1
    }
}

pub mod part2 {
    use utils::coords::iter_cross_near;

    use super::{Cell, Parsed};

    pub fn solve(map: Parsed) -> usize {
        let (h, w) = (map.len(), map[0].len());
        let mut stack = Vec::from([(false, (0, 1), 0), (true, (0, 1), 1)]);
        let mut seen = vec![vec![false; w]; h];
        let mut result = 0;
        while let Some((enter, coord @ (i, j), steps)) = stack.pop() {
            if enter {
                if seen[i][j] {
                    continue;
                }
                seen[i][j] = true;
                if coord == (h - 1, w - 2) {
                    result = result.max(steps);
                }
                for to @ (ni, nj) in iter_cross_near(i as isize, j as isize)
                    .map(|(ni, nj)| (ni as usize, nj as usize))
                {
                    if ni < map.len()
                        && nj < map[0].len()
                        && map[ni][nj] != Cell::Forest
                        && !seen[ni][nj]
                    {
                        stack.push((false, to, 0));
                        stack.push((true, to, steps + 1));
                    }
                }
            } else {
                seen[i][j] = false;
            }
        }
        result - 1
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_23_input.txt").unwrap()
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
