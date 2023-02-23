//! https://adventofcode.com/2016/day/3
//! https://adventofcode.com/2016/day/3/input

use std::{fs::read_to_string, time::Instant};

type Triangle = (usize, usize, usize);
fn generic_solve(triangles: Vec<Triangle>) -> usize {
    triangles
        .iter()
        .filter(|triangle| {
            triangle.0 + triangle.1 > triangle.2
                && triangle.2 + triangle.1 > triangle.0
                && triangle.0 + triangle.2 > triangle.1
        })
        .count()
}

pub mod part1 {
    use crate::day_03::{generic_solve, Triangle};

    fn parse(input: &str) -> Vec<Triangle> {
        input
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                (
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                )
            })
            .collect()
    }

    pub fn solve(input: &str) -> usize {
        generic_solve(parse(input))
    }
}

pub mod part2 {
    use crate::day_03::{generic_solve, Triangle};

    fn parse(input: &str) -> Vec<Triangle> {
        let numbers: Vec<Vec<usize>> = input
            .lines()
            .map(|line| line.split_whitespace().flat_map(str::parse).collect())
            .collect();
        let mut result = vec![];
        for i in (0..numbers.len()).step_by(3) {
            for j in 0..3 {
                result.push((numbers[i][j], numbers[i + 1][j], numbers[i + 2][j]))
            }
        }
        result
    }

    pub fn solve(input: &str) -> usize {
        generic_solve(parse(input))
    }
}

pub fn main(test: bool) {
    let test_input = "101 301 501
102 302 502
103 303 503
201 401 601
202 402 602
203 403 603"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_03_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
