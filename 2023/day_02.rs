//! https://adventofcode.com/2023/day/2
//! https://adventofcode.com/2023/day/2/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

use regex::Regex;

#[derive(Copy, Clone, Default, Debug)]
pub struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

type Game = Vec<Cubes>;

type Parsed = Vec<Game>;

fn parse(input: &str) -> Parsed {
    let red = Regex::new(r"(\d+) red").unwrap();
    let green = Regex::new(r"(\d+) green").unwrap();
    let blue = Regex::new(r"(\d+) blue").unwrap();
    input
        .lines()
        .map(|line| {
            line.split(';')
                .map(|s| {
                    let mut cubes = Cubes::default();
                    if let Some(red) = red.captures(s) {
                        cubes.red = red[1].parse().unwrap();
                    }
                    if let Some(green) = green.captures(s) {
                        cubes.green = green[1].parse().unwrap();
                    }
                    if let Some(blue) = blue.captures(s) {
                        cubes.blue = blue[1].parse().unwrap();
                    }
                    cubes
                })
                .collect()
        })
        .collect()
}

pub mod part1 {
    use super::Parsed;

    pub fn solve(games: Parsed) -> usize {
        games
            .iter()
            .enumerate()
            .filter_map(|(id, game)| {
                game.iter()
                    .all(|cubes| cubes.red <= 12 && cubes.green <= 13 && cubes.blue <= 14)
                    .then_some(id + 1)
            })
            .sum()
    }
}

pub mod part2 {
    use super::{Cubes, Parsed};

    pub(crate) fn solve(games: Parsed) -> usize {
        games
            .iter()
            .map(|game| {
                let max = game.iter().fold(Cubes::default(), |mut acc, cubes| {
                    acc.red = acc.red.max(cubes.red);
                    acc.green = acc.green.max(cubes.green);
                    acc.blue = acc.blue.max(cubes.blue);
                    acc
                });
                max.red * max.green * max.blue
            })
            .sum()
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_02_input.txt").unwrap()
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

    println!("Total {:?}", total);
    total
}
