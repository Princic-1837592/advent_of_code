//! https://adventofcode.com/2018/day/6
//! https://adventofcode.com/2018/day/6/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(", ").collect();
            (parts[1].parse().unwrap(), parts[0].parse().unwrap())
        })
        .collect()
}

pub mod part1 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let points = parse(input);
        let (max_i, max_j) = points
            .iter()
            .fold((usize::MIN, usize::MIN), |(max_i, max_j), (x, y)| {
                (max_i.max(*x), max_j.max(*y))
            });
        let mut distances = vec![vec![vec![usize::MAX; points.len()]; max_j + 1]; max_i + 1];
        for (i, row) in distances.iter_mut().enumerate() {
            for (j, point) in row.iter_mut().enumerate() {
                for (p, (x, y)) in points.iter().enumerate() {
                    point[p] = (i as isize - *x as isize).unsigned_abs()
                        + (j as isize - *y as isize).unsigned_abs();
                }
            }
        }
        let mut nearest = vec![vec![None; max_j + 1]; max_i + 1];
        for (i, row) in distances.iter_mut().enumerate() {
            for (j, point) in row.iter_mut().enumerate() {
                let (p, min_distance) = point
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, distance)| **distance)
                    .unwrap();
                if point
                    .iter()
                    .filter(|distance| *distance == min_distance)
                    .count()
                    == 1
                {
                    nearest[i][j] = Some(p);
                }
            }
        }
        let mut valid = vec![true; points.len()];
        for j in 0..nearest[0].len() {
            if let Some(p) = nearest[0][j] {
                valid[p] = false;
            }
            if let Some(p) = nearest[nearest.len() - 1][j] {
                valid[p] = false;
            }
        }
        for i in 0..nearest.len() {
            if let Some(p) = nearest[i][0] {
                valid[p] = false;
            }
            if let Some(p) = nearest[i][nearest[0].len() - 1] {
                valid[p] = false;
            }
        }
        let mut largest = vec![0_usize; points.len()];
        for row in nearest {
            for p in row.into_iter().flatten() {
                largest[p] += 1;
            }
        }
        *largest
            .iter()
            .enumerate()
            .filter(|(i, _)| valid[*i])
            .max_by_key(|(_, area)| **area)
            .unwrap()
            .1
    }
}

pub mod part2 {
    use super::parse;

    pub fn solve(input: &str, max_distance: usize) -> usize {
        let points = parse(input);
        let (max_i, max_j) = points
            .iter()
            .fold((usize::MIN, usize::MIN), |(max_i, max_j), (x, y)| {
                (max_i.max(*x), max_j.max(*y))
            });
        let mut distances = vec![vec![0; max_j + 1]; max_i + 1];
        for (i, row) in distances.iter_mut().enumerate() {
            for (j, point) in row.iter_mut().enumerate() {
                for (x, y) in points.iter() {
                    *point += (i as isize - *x as isize).unsigned_abs()
                        + (j as isize - *y as isize).unsigned_abs();
                }
            }
        }
        distances
            .iter()
            .flat_map(|row| row.iter())
            .filter(|distance| **distance < max_distance)
            .count()
    }
}

pub fn main(test: bool) {
    let test_input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9"
        .to_owned();
    let (puzzle_input, max_distance) = if test {
        (test_input, 32)
    } else {
        (read_to_string("inputs/day_06_input.txt").unwrap(), 10000)
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input, max_distance));
    println!("Run in {:?}", start.elapsed());
}
