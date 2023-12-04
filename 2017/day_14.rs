//! https://adventofcode.com/2017/day/14
//! https://adventofcode.com/2017/day/14/input

use std::{fs::read_to_string, time::Instant};

use crate::day_10::part2::hash;

fn parse(input: &str) -> [[usize; 128]; 128] {
    let mut grid = [[0; 128]; 128];
    for (i, row) in grid.iter_mut().enumerate() {
        let hash = hash(&format!("{}-{}", input, i));
        for (j, cell) in row.iter_mut().enumerate() {
            if hash[j / 8] & (0b10000000 >> (j % 8)) != 0 {
                *cell = 1;
            };
        }
    }
    grid
}

pub mod part1 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let grid = parse(input);
        grid.iter().map(|row| row.iter().sum::<usize>()).sum()
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    use super::parse;

    pub fn solve(input: &str) -> usize {
        let grid = parse(input);
        let mut regions = 0;
        let mut visited = [[false; 128]; 128];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    visited[i][j] = true;
                } else if !visited[i][j] {
                    regions += 1;
                    let mut queue = VecDeque::from([(i, j)]);
                    while let Some((i, j)) = queue.pop_front() {
                        visited[i][j] = true;
                        for neighbor @ (ni, nj) in [
                            (i.wrapping_sub(1), j),
                            (i, j.wrapping_sub(1)),
                            (i + 1, j),
                            (i, j + 1),
                        ] {
                            if ni < 128 && nj < 128 && grid[ni][nj] == 1 && !visited[ni][nj] {
                                queue.push_back(neighbor);
                            }
                        }
                    }
                }
            }
        }
        regions
    }
}

pub fn main(test: bool) {
    let test_input = "flqrgnkx".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_14_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
