//! https://adventofcode.com/2018/day/18
//! https://adventofcode.com/2018/day/18/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialOrd, PartialEq, Ord)]
enum Acre {
    Ground,
    Trees,
    Lumberyard,
}

type Acres = Vec<Vec<Acre>>;

impl From<char> for Acre {
    fn from(char: char) -> Self {
        match char {
            '.' => Self::Ground,
            '|' => Self::Trees,
            '#' => Self::Lumberyard,
            _ => panic!("Invalid char: {}", char),
        }
    }
}

fn parse(input: &str) -> Acres {
    input
        .lines()
        .map(|line| line.chars().map(Acre::from).collect())
        .collect()
}

fn neighbors(i: usize, j: usize, size: usize) -> impl Iterator<Item = (usize, usize)> {
    (i.saturating_sub(1)..=(i + 1).min(size - 1))
        .flat_map(move |i| (j.saturating_sub(1)..=(j + 1).min(size - 1)).map(move |j| (i, j)))
        .filter(move |neighbor| *neighbor != (i, j))
}

fn minute(acres: &mut Acres, support: &mut Acres) {
    for i in 0..acres.len() {
        for (j, acre) in acres[i].iter().enumerate() {
            let (_, trees, lumberyards) =
                neighbors(i, j, acres.len()).fold((0, 0, 0), |mut acc, (ni, nj)| {
                    match acres[ni][nj] {
                        Acre::Ground => acc.0 += 1,
                        Acre::Trees => acc.1 += 1,
                        Acre::Lumberyard => acc.2 += 1,
                    }
                    acc
                });
            support[i][j] = match acre {
                Acre::Ground if trees >= 3 => Acre::Trees,
                Acre::Trees if lumberyards >= 3 => Acre::Lumberyard,
                Acre::Lumberyard if lumberyards * trees == 0 => Acre::Ground,
                _ => *acre,
            };
        }
    }
    acres
        .iter_mut()
        .enumerate()
        .for_each(|(i, line)| line.copy_from_slice(&support[i]));
}

pub mod part1 {
    use super::{minute, parse, Acre};

    pub fn solve(input: &str) -> usize {
        let mut acres = parse(input);
        let mut support = acres.clone();
        for _ in 0..10 {
            minute(&mut acres, &mut support);
        }
        let (trees, lumberyards) = acres.iter().flatten().fold((0, 0), |mut acc, acre| {
            match acre {
                Acre::Trees => acc.0 += 1,
                Acre::Lumberyard => acc.1 += 1,
                _ => {}
            }
            acc
        });
        trees * lumberyards
    }
}

pub mod part2 {
    use std::collections::{hash_map::Entry, HashMap};

    use super::{minute, parse, Acre};

    pub fn solve(input: &str) -> usize {
        let mut acres = parse(input);
        let mut support = acres.clone();
        let mut states = HashMap::new();
        let mut min = 0;
        while min < 1000000000 {
            match states.entry(acres.clone()) {
                Entry::Occupied(entry) => {
                    let last = entry.get();
                    let interval = min - last;
                    let left = 1000000000 - min;
                    let cycles = left / interval;
                    min += cycles * interval;
                }
                Entry::Vacant(entry) => {
                    entry.insert(min);
                }
            }
            minute(&mut acres, &mut support);
            min += 1;
        }
        let (trees, lumberyards) = acres.iter().flatten().fold((0, 0), |mut acc, acre| {
            match acre {
                Acre::Trees => acc.0 += 1,
                Acre::Lumberyard => acc.1 += 1,
                _ => {}
            }
            acc
        });
        trees * lumberyards
    }
}

pub fn main(test: bool) {
    let test_input = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."
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
