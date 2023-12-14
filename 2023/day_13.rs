//! https://adventofcode.com/2023/day/13
//! https://adventofcode.com/2023/day/13/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

use utils::matrix::rotate_left;

type Pattern = Vec<Vec<bool>>;

type Parsed = Vec<Pattern>;

fn parse(input: &str) -> Parsed {
    let separator = "\n".repeat(2);
    input
        .replace("\r\n", "\n")
        .split(&separator)
        .map(|pattern| {
            pattern
                .lines()
                .map(|line| line.chars().map(|char| char == '#').collect())
                .collect()
        })
        .collect()
}

fn find_vertical(pattern: &Pattern) -> Option<usize> {
    'column: for c in 1..pattern[0].len() {
        for row in pattern {
            for dj in 0..c.min(pattern[0].len() - c) {
                if row[c - dj - 1] != row[c + dj] {
                    continue 'column;
                }
            }
        }
        return Some(c);
    }
    None
}

fn find_reflection(pattern: &Pattern) -> usize {
    find_vertical(pattern).unwrap_or_else(|| {
        let pattern = rotate_left(pattern);
        find_vertical(&pattern).unwrap() * 100
    })
}

pub mod part1 {
    use super::{find_reflection, Parsed};

    pub fn solve(patterns: Parsed) -> usize {
        patterns.iter().map(find_reflection).sum()
    }
}

pub mod part2 {
    use std::collections::HashSet;

    use utils::matrix::rotate_left;

    use super::{Parsed, Pattern};

    fn find_verticals(pattern: &Pattern) -> HashSet<usize> {
        let mut result = HashSet::new();
        'column: for c in 1..pattern[0].len() {
            let mut diff = false;
            for row in pattern {
                for dj in 0..c.min(pattern[0].len() - c) {
                    if row[c - dj - 1] != row[c + dj] {
                        if diff {
                            continue 'column;
                        }
                        diff = true;
                    }
                }
            }
            result.insert(c);
        }
        result
    }

    fn find_reflections(pattern: &Pattern) -> HashSet<usize> {
        let mut result = find_verticals(pattern);
        let pattern = rotate_left(pattern);
        result.extend(find_verticals(&pattern).iter().map(|v| v * 100));
        result
    }

    fn find_reflection(pattern: Pattern) -> usize {
        let original = super::find_reflection(&pattern);
        let mut new = find_reflections(&pattern);
        new.remove(&original);
        if let Some(&result) = new.iter().next() {
            return result;
        }
        original
    }

    pub fn solve(patterns: Parsed) -> usize {
        patterns.into_iter().map(find_reflection).sum()
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_13_input.txt").unwrap()
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
