//! https://adventofcode.com/2023/day/11
//! https://adventofcode.com/2023/day/11/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

type Coord = (usize, usize);

type Parsed = (Vec<Coord>, usize, usize);

fn parse(input: &str) -> Parsed {
    let (h, w) = (
        input.lines().count(),
        input.lines().next().unwrap().chars().count(),
    );
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                l.chars()
                    .enumerate()
                    .flat_map(move |(j, c)| (c == '#').then_some((i, j)))
            })
            .collect(),
        h,
        w,
    )
}

fn expand(stars: &mut [Coord], h: usize, w: usize, factor: usize) {
    let mut rows = vec![false; h];
    let mut columns = vec![false; w];
    for &(i, j) in stars.iter() {
        rows[i] = true;
        columns[j] = true;
    }
    let mut rows_shift = vec![0; h];
    let mut c = 0;
    for (i, found) in rows.into_iter().enumerate() {
        if !found {
            c += factor - 1;
        }
        rows_shift[i] = c;
    }
    let mut columns_shift = vec![0; w];
    let mut c = 0;
    for (j, found) in columns.into_iter().enumerate() {
        if !found {
            c += factor - 1;
        }
        columns_shift[j] = c;
    }
    stars.iter_mut().for_each(|(i, j)| {
        *i += rows_shift[*i];
        *j += columns_shift[*j]
    })
}

fn solve(mut stars: Vec<Coord>, h: usize, w: usize, factor: usize) -> usize {
    expand(&mut stars, h, w, factor);
    stars
        .iter()
        .enumerate()
        .flat_map(|(s1, &(i1, j1))| {
            stars
                .iter()
                .skip(s1 + 1)
                .map(move |&(i2, j2)| i1.abs_diff(i2) + j1.abs_diff(j2))
        })
        .sum::<usize>()
}

pub mod part1 {
    use super::Parsed;

    pub fn solve((stars, h, w): Parsed) -> usize {
        super::solve(stars, h, w, 2)
    }
}

pub mod part2 {
    use super::Parsed;

    pub fn solve((stars, h, w): Parsed) -> usize {
        super::solve(stars, h, w, 1_000_000)
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_11_input.txt").unwrap()
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
