//! https://adventofcode.com/2023/day/21
//! https://adventofcode.com/2023/day/21/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Plot,
    Rock,
}

type Coord = (usize, usize);

type Parsed = (Vec<Vec<Tile>>, Coord);

fn parse(input: &str) -> Parsed {
    let (mut si, mut sj) = (0, 0);
    let mut result = vec![
        vec![Tile::Rock; input.lines().next().unwrap().chars().count()];
        input.lines().count()
    ];
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            result[i][j] = match char {
                '.' => Tile::Plot,
                '#' => Tile::Rock,
                'S' => {
                    (si, sj) = (i, j);
                    Tile::Plot
                }
                _ => unreachable!(),
            };
        }
    }
    (result, (si, sj))
}

pub mod part1 {
    use std::collections::VecDeque;

    use utils::coords::iter_cross_near;

    use super::{Parsed, Tile};

    pub fn solve((garden, start): Parsed) -> usize {
        let (h, w) = (garden.len(), garden[0].len());
        let mut queue = VecDeque::from([(start, 0)]);
        let mut seen = vec![vec![usize::MAX; w]; h];
        let mut result = 0;
        while let Some(((i, j), steps)) = queue.pop_front() {
            if steps > 64 || seen[i][j] != usize::MAX {
                continue;
            }
            seen[i][j] = steps;
            if steps % 2 == 0 {
                result += 1;
            }
            for (ni, nj) in
                iter_cross_near(i as isize, j as isize).map(|(i, j)| (i as usize, j as usize))
            {
                if ni < h && nj < w {
                    if let Tile::Plot = garden[ni][nj] {
                        queue.push_back(((ni, nj), steps + 1));
                    }
                }
            }
        }
        result
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    use utils::coords::iter_cross_near;

    use super::{Parsed, Tile};

    pub fn solve((garden, start): Parsed) -> usize {
        let (h, w) = (garden.len(), garden[0].len());
        let mut queue = VecDeque::from([(start, 0)]);
        let mut seen = vec![vec![usize::MAX; w]; h];
        while let Some(((i, j), steps)) = queue.pop_front() {
            if seen[i][j] != usize::MAX {
                continue;
            }
            seen[i][j] = steps;
            for (ni, nj) in
                iter_cross_near(i as isize, j as isize).map(|(i, j)| (i as usize, j as usize))
            {
                if ni < h && nj < w {
                    if let Tile::Plot = garden[ni][nj] {
                        queue.push_back(((ni, nj), steps + 1));
                    }
                }
            }
        }
        let visited = seen.iter().flatten().filter(|&&v| v != usize::MAX);
        let even_corners = visited.clone().filter(|v| **v % 2 == 0 && **v > 65).count();
        let odd_corners = visited.clone().filter(|v| **v % 2 == 1 && **v > 65).count();
        let even_full = visited.clone().filter(|v| **v % 2 == 0).count();
        let odd_full = visited.clone().filter(|v| **v % 2 == 1).count();
        let n = (26501365 - (w / 2)) / h;
        ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners
            + n * even_corners
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2023/day_21_input.txt").unwrap()
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
