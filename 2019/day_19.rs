//! https://adventofcode.com/2019/day/19
//! https://adventofcode.com/2019/day/19/input

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use itertools::Itertools;
    use rayon::prelude::*;

    use crate::int_code::parse;

    pub fn solve(input: &str) -> usize {
        let vm = parse(input);
        (0..50)
            .cartesian_product(0..50)
            .collect::<Vec<_>>()
            .into_par_iter()
            .filter(|&(x, y)| {
                let mut vm = vm.clone();
                vm.push_input(x);
                vm.push_input(y);
                vm.run_until_complete();
                vm.last_output().unwrap_or(0) == 1
            })
            .count()
    }
}

pub mod part2 {
    use itertools::Itertools;
    use rayon::prelude::*;

    use crate::int_code::{parse, IntCode};

    type Coord = (i64, i64);

    fn next(vm: IntCode, (mut x, mut y): Coord, check: i64, dx: i64) -> Coord {
        y += 1;
        while {
            let mut vm = vm.clone();
            vm.push_input(x);
            vm.push_input(y);
            vm.run_until_complete();
            vm.last_output().unwrap()
        } == check
        {
            x += 1;
        }
        (x + dx, y)
    }

    fn next_left(vm: IntCode, coord: Coord) -> Coord {
        next(vm, coord, 0, 0)
    }

    fn next_right(vm: IntCode, coord: Coord) -> Coord {
        next(vm, coord, 1, -1)
    }

    fn find_first(vm: IntCode) -> (Coord, Coord) {
        for y in 0.. {
            let beam: Vec<_> = (0..100)
                .into_par_iter()
                .filter_map(|x| {
                    let mut vm = vm.clone();
                    vm.push_input(x);
                    vm.push_input(y);
                    vm.run_until_complete();
                    (vm.last_output().unwrap_or(0) == 1).then_some((x, y))
                })
                .collect();
            if beam.len() >= 5 {
                return (beam[0], *beam.last().unwrap());
            }
        }
        unreachable!()
    }

    pub fn solve(input: &str) -> i64 {
        let vm = parse(input);
        let (mut left, mut right) = find_first(vm.clone());
        for _ in 0..99 {
            left = next_left(vm.clone(), left)
        }
        while right.0 - left.0 + 1 < 100 {
            right = next_right(vm.clone(), right);
            left = next_left(vm.clone(), left);
        }
        left.0 * 10000 + right.1
    }

    #[allow(unused)]
    fn to_string(vm: IntCode, size: i64) -> String {
        let beam: Vec<_> = (0..size)
            .cartesian_product(0..size)
            .collect::<Vec<_>>()
            .into_par_iter()
            .filter(|&(x, y)| {
                let mut vm = vm.clone();
                vm.push_input(x);
                vm.push_input(y);
                vm.run_until_complete();
                vm.last_output().unwrap_or(0) == 1
            })
            .collect();
        let min_row = beam.iter().map(|&(_, y)| y).min().unwrap();
        let min_col = beam.iter().map(|&(x, _)| x).min().unwrap();
        let max_row = beam.iter().map(|&(_, y)| y).max().unwrap();
        let max_col = beam.iter().map(|&(x, _)| x).max().unwrap();
        let (h, w) = (max_row - min_row + 1, max_col - min_col + 1);
        let mut result = vec![vec![' '; w as usize]; h as usize];
        for (i, row) in result.iter_mut().enumerate() {
            for (j, panel) in row.iter_mut().enumerate() {
                if beam.contains(&(j as i64 + min_col, i as i64 + min_row)) {
                    *panel = '#';
                }
            }
        }
        result.iter().map(|row| row.iter().join("")).join("\n")
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2019/day_19_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
