//! https://adventofcode.com/2019/day/11
//! https://adventofcode.com/2019/day/11/input

use std::{collections::HashSet, fs::read_to_string, time::Instant};

use crate::int_code::{parse, Interrupt};

fn paint(input: &str, painted: &mut HashSet<(i32, i32)>, white: &mut HashSet<(i32, i32)>) {
    let mut vm = parse(input);
    let (mut i, mut j, mut di, mut dj) = (0, 0, -1, 0);
    let mut first_output = true;
    loop {
        match vm.run_until_interrupt() {
            Interrupt::Input => vm.push_input(if white.contains(&(i, j)) { 1 } else { 0 }),
            Interrupt::Output(value) => {
                first_output = if first_output {
                    if value == 0 {
                        white.remove(&(i, j));
                    } else {
                        white.insert((i, j));
                    }
                    painted.insert((i, j));
                    false
                } else {
                    if value == 0 {
                        (di, dj) = (-dj, di);
                    } else {
                        (di, dj) = (dj, -di);
                    }
                    (i, j) = (i + di, j + dj);
                    true
                }
            }
            Interrupt::Halt => break,
            Interrupt::Error => break,
        }
    }
}

pub mod part1 {
    use std::collections::HashSet;

    use crate::day_11::paint;

    pub fn solve(input: &str) -> usize {
        let mut painted = HashSet::new();
        let mut white = HashSet::new();
        paint(input, &mut painted, &mut white);
        painted.len()
    }
}

pub mod part2 {
    use std::collections::HashSet;

    use itertools::Itertools;

    use crate::day_11::paint;

    pub fn solve(input: &str) -> String {
        let mut painted = HashSet::new();
        let mut white = HashSet::from([(0, 0)]);
        paint(input, &mut painted, &mut white);
        let min_row = white.iter().map(|&(i, _)| i).min().unwrap();
        let min_col = white.iter().map(|&(_, j)| j).min().unwrap();
        let max_row = white.iter().map(|&(i, _)| i).max().unwrap();
        let max_col = white.iter().map(|&(_, j)| j).max().unwrap();
        let (h, w) = (max_row - min_row + 1, max_col - min_col + 1);
        let mut result = vec![vec![' '; w as usize]; h as usize];
        for (i, row) in result.iter_mut().enumerate() {
            for (j, panel) in row.iter_mut().enumerate() {
                if white.contains(&(i as i32 + min_row, j as i32 + min_col)) {
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
        read_to_string("inputs/day_11_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
