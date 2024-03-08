//! https://adventofcode.com/2023/day/10
//! https://adventofcode.com/2023/day/10/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Empty,
    Start,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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
            Direction::S => (i + 1 < h).then_some((i + 1, j)),
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

    fn swap_nw(&self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::E => Direction::S,
            Direction::S => Direction::E,
            Direction::W => Direction::N,
        }
    }

    fn swap_ne(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::N,
            Direction::S => Direction::W,
            Direction::W => Direction::S,
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
    use std::collections::{HashSet, VecDeque};

    use super::{Coord, Direction, Parsed, Pipe, Pipes, DIRECTIONS};

    type Path = Vec<Coord>;

    fn explore(pipes: &Pipes, oi: usize, oj: usize, mut direction: Direction) -> Option<Path> {
        let (mut i, mut j) = (oi, oj);
        let mut visited = Vec::new();
        let (h, w) = (pipes.len(), pipes[0].len());
        while (i, j) != (oi, oj) || visited.is_empty() {
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
            visited.push((i, j));
        }
        Some(visited)
    }

    fn find_path(pipes: &Pipes, (i, j): Coord) -> Path {
        for direction in DIRECTIONS {
            if let Some(borders) = explore(pipes, i, j, direction) {
                return borders;
            }
        }
        unreachable!()
    }

    fn find_inside(pipes: &Pipes, path: &Path, borders: &HashSet<Coord>) -> (usize, Direction) {
        let (h, w) = (pipes.len(), pipes[0].len());
        for (index, (mut i, mut j)) in path
            .iter()
            .enumerate()
            .filter(|(_, (i, j))| pipes[*i][*j] == Pipe::NS)
        {
            'direction: for d in [Direction::E, Direction::W] {
                while let Some(next @ (ni, nj)) = d.transform(i, j, h, w) {
                    if borders.contains(&next) {
                        continue 'direction;
                    }
                    (i, j) = (ni, nj);
                }
                return (index, d.reverse());
            }
        }
        unreachable!()
    }

    fn bfs(start: Coord, visited: &mut HashSet<Coord>, h: usize, w: usize) -> usize {
        let mut queue = VecDeque::from([start]);
        let mut enclosed = 0;
        while let Some(coord @ (i, j)) = queue.pop_front() {
            if visited.contains(&coord) {
                continue;
            }
            visited.insert(coord);
            enclosed += 1;
            for next in DIRECTIONS.iter().flat_map(|d| d.transform(i, j, h, w)) {
                queue.push_back(next);
            }
        }
        enclosed
    }

    pub fn solve((mut pipes, start): Parsed) -> usize {
        let path = find_path(&pipes, start);
        pipes[path[path.len() - 1].0][path[path.len() - 1].1] = match (
            (
                path[0].0 as isize - path[path.len() - 1].0 as isize,
                path[0].1 as isize - path[path.len() - 1].1 as isize,
            ),
            (
                path[path.len() - 2].0 as isize - path[path.len() - 1].0 as isize,
                path[path.len() - 2].1 as isize - path[path.len() - 1].1 as isize,
            ),
        ) {
            ((0, _), (0, _)) => Pipe::EW,
            ((_, 0), (_, 0)) => Pipe::NS,
            ((0, 1), (1, 0)) | ((1, 0), (0, 1)) => Pipe::SE,
            ((0, 1), (-1, 0)) | ((-1, 0), (0, 1)) => Pipe::NE,
            ((0, -1), (1, 0)) | ((1, 0), (0, -1)) => Pipe::SW,
            ((0, -1), (-1, 0)) | ((-1, 0), (0, -1)) => Pipe::NW,
            _ => {
                unimplemented!()
            }
        };
        let borders = path.iter().copied().collect();
        let (index, mut inside) = find_inside(&pipes, &path, &borders);
        let mut visited = borders.clone();
        let (h, w) = (pipes.len(), pipes[0].len());
        let mut enclosed = 0;
        for ((i, j), pipe) in (index..path.len()).chain(0..index).map(|i| {
            let (pi, pj) = path[i];
            (path[i], pipes[pi][pj])
        }) {
            enclosed += bfs(inside.transform(i, j, h, w).unwrap(), &mut visited, h, w);
            match pipe {
                Pipe::NS | Pipe::EW => {}
                Pipe::NE | Pipe::SW => {
                    inside = inside.swap_ne();
                    enclosed += bfs(inside.transform(i, j, h, w).unwrap(), &mut visited, h, w);
                }
                Pipe::NW | Pipe::SE => {
                    inside = inside.swap_nw();
                    enclosed += bfs(inside.transform(i, j, h, w).unwrap(), &mut visited, h, w);
                }
                Pipe::Empty | Pipe::Start => unreachable!(),
            }
        }
        enclosed
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_10_input.txt").unwrap()
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
