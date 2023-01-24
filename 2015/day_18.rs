//! https://adventofcode.com/2015/day/18

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect()
}

pub fn iter_neighbors(
    coord @ (r, c): (usize, usize),
    height: usize,
    width: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (r.saturating_sub(1)..=(r + 1).min(height - 1))
        .flat_map(move |i| (c.saturating_sub(1)..=(c + 1).min(width - 1)).map(move |j| (i, j)))
        .filter(move |&pos| pos != coord)
}

pub mod part1 {
    use crate::day_18::{iter_neighbors, parse};

    fn step(lights: &mut Vec<Vec<bool>>, support: &mut [Vec<bool>]) {
        for i in 0..lights.len() {
            for j in 0..lights[0].len() {
                let neighbors = iter_neighbors((i, j), lights.len(), lights[0].len())
                    .map(|(i, j)| lights[i][j])
                    .filter(|&on| on)
                    .count();
                support[i][j] = if lights[i][j] {
                    neighbors == 2 || neighbors == 3
                } else {
                    neighbors == 3
                }
            }
        }
        lights.clone_from_slice(support);
    }

    pub fn solve(input: &str) -> usize {
        let mut lights = parse(input);
        let mut support = vec![vec![false; lights[0].len()]; lights.len()];
        for _ in 0..100 {
            step(&mut lights, &mut support);
        }
        lights
            .iter()
            .flat_map(|line| line.iter())
            .filter(|&&light| light)
            .count()
    }
}

pub mod part2 {
    use crate::day_18::{iter_neighbors, parse};

    fn step(lights: &mut Vec<Vec<bool>>, support: &mut [Vec<bool>]) {
        for i in 0..lights.len() {
            for j in 0..lights[0].len() {
                if [
                    (0, 0),
                    (0, lights[0].len() - 1),
                    (lights.len() - 1, 0),
                    (lights.len() - 1, lights[0].len() - 1),
                ]
                .contains(&(i, j))
                {
                    continue;
                }
                let neighbors = iter_neighbors((i, j), lights.len(), lights[0].len())
                    .map(|(i, j)| lights[i][j])
                    .filter(|&on| on)
                    .count();
                support[i][j] = if lights[i][j] {
                    neighbors == 2 || neighbors == 3
                } else {
                    neighbors == 3
                }
            }
        }
        lights.clone_from_slice(support);
    }

    pub fn solve(input: &str) -> usize {
        let mut lights = parse(input);
        let mut support = vec![vec![false; lights[0].len()]; lights.len()];
        for (i, j) in [
            (0, 0),
            (0, lights[0].len() - 1),
            (lights.len() - 1, 0),
            (lights.len() - 1, lights[0].len() - 1),
        ] {
            lights[i][j] = true;
            support[i][j] = true;
        }
        for _ in 0..100 {
            step(&mut lights, &mut support);
        }
        lights
            .iter()
            .flat_map(|line| line.iter())
            .filter(|&&light| light)
            .count()
    }
}

pub fn main(test: bool) {
    let test_input = ".#.#.#
...##.
#....#
..#...
#.#..#
####.."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_18_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
