//! https://adventofcode.com/2016/day/24
//! https://adventofcode.com/2016/day/24/input

use std::{collections::VecDeque, fs::read_to_string, time::Instant};

use itertools::Itertools;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Cell {
    Wall,
    Empty,
    Number(usize),
}

impl From<char> for Cell {
    fn from(char: char) -> Self {
        match char {
            '#' => Cell::Wall,
            '.' => Cell::Empty,
            number @ '0'..='9' => Cell::Number(number.to_digit(10).unwrap() as usize),
            _ => panic!("Invalid char: {}", char),
        }
    }
}

type Coord = (usize, usize);
type Cells = Vec<Vec<Cell>>;

fn parse(input: &str) -> (Cells, Vec<Coord>) {
    let result: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(Cell::from).collect())
        .collect();
    let mut numbers = vec![];
    for (i, row) in result.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            match cell {
                Cell::Wall => {}
                Cell::Empty => {}
                Cell::Number(n) => {
                    if n >= numbers.len() {
                        numbers.resize(n + 1, (0, 0));
                    }
                    numbers[n] = (i, j);
                }
            }
        }
    }
    (result, numbers)
}

const NEIGHBORS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn bfs(cells: &Cells, number: usize, numbers: &Vec<Coord>) -> Vec<usize> {
    let mut distances = vec![0; numbers.len()];
    let mut found = 0;
    let mut queue = VecDeque::from([(0, numbers[number])]);
    let mut visited = vec![vec![false; cells[0].len()]; cells.len()];
    while let Some((dist, (i, j))) = queue.pop_front() {
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;
        if let Cell::Number(n) = cells[i][j] {
            distances[n] = dist;
            found += 1;
            if found == numbers.len() {
                break;
            }
        }
        for (di, dj) in NEIGHBORS {
            let (ni, nj) = (i as isize + di, j as isize + dj);
            if ni < 0 || nj < 0 || ni >= cells.len() as isize || nj >= cells[0].len() as isize {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if visited[ni][nj] || cells[ni][nj] == Cell::Wall {
                continue;
            }
            queue.push_back((dist + 1, (ni, nj)));
        }
    }
    distances
}

fn solve_generic(input: &str, go_back: bool) -> usize {
    let (cells, numbers) = parse(input);
    let mut distances = Vec::with_capacity(numbers.len());
    for number in 0..numbers.len() {
        distances.push(bfs(&cells, number, &numbers));
    }
    (1..numbers.len())
        .permutations(numbers.len() - 1)
        .map(|perm| {
            let mut distance = 0;
            let mut node = 0;
            for dest in perm {
                distance += distances[node][dest];
                node = dest;
            }
            if go_back {
                distance += distances[node][0];
            }
            distance
        })
        .min()
        .unwrap()
}

pub mod part1 {
    use crate::day_24::solve_generic;

    pub fn solve(input: &str) -> usize {
        solve_generic(input, false)
    }
}

pub mod part2 {
    use crate::day_24::solve_generic;

    pub fn solve(input: &str) -> usize {
        solve_generic(input, true)
    }
}

pub fn main(test: bool) {
    let test_input = "###########
#0.1.....2#
#.#######.#
#4.......3#
###########"
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
