//! https://adventofcode.com/2018/day/11
//! https://adventofcode.com/2018/day/11/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> isize {
    input.parse().unwrap()
}

fn make_grid(serial_number: isize) -> [[isize; 300]; 300] {
    let mut grid = [[0; 300]; 300];
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            let rack_id = j as isize + 11;
            let mut power_lvl = rack_id * (i + 1) as isize;
            power_lvl += serial_number;
            power_lvl *= rack_id;
            power_lvl = (power_lvl / 100) % 10;
            *cell = power_lvl - 5;
        }
    }
    grid
}

fn find_max_rect(grid: &[[isize; 300]], size: usize) -> (usize, usize, isize) {
    let mut max = (0, 0, 0);
    let mut total: isize = grid
        .iter()
        .take(size - 1)
        .flat_map(|row| row.iter().take(size - 1))
        .sum();
    for (i, row) in grid.iter().enumerate().skip(size - 1) {
        total += row.iter().take(size - 1).sum::<isize>();
        let mut this = total;
        for (j, _) in row.iter().enumerate().skip(size - 1) {
            this += grid
                .iter()
                .skip(i - (size - 1))
                .take(size)
                .map(|row| row[j])
                .sum::<isize>();
            if this > max.2 {
                max = (j - (size - 2), i - (size - 2), this);
            }
            this -= grid
                .iter()
                .skip(i - (size - 1))
                .take(size)
                .map(|row| row[j - (size - 1)])
                .sum::<isize>();
        }
        total -= grid[i - (size - 1)].iter().take(size - 1).sum::<isize>();
    }
    max
}

pub mod part1 {
    use super::{find_max_rect, make_grid, parse};

    pub fn solve(input: &str) -> String {
        let grid = make_grid(parse(input));
        let result = find_max_rect(&grid, 3);
        format!("{},{}", result.0, result.1)
    }
}

pub mod part2 {
    use super::{find_max_rect, make_grid, parse};

    pub fn solve(input: &str) -> String {
        let grid = make_grid(parse(input));
        let (result, size) = (2..=300)
            .map(|size| (find_max_rect(&grid, size), size))
            .max_by_key(|((_, _, value), _)| *value)
            .unwrap();
        format!("{},{},{}", result.0, result.1, size)
    }
}

pub fn main(test: bool) {
    let test_input = "42".to_owned();
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
