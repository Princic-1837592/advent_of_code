//! https://adventofcode.com/2023/day/15
//! https://adventofcode.com/2023/day/15/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

type Parsed<'a> = Vec<Vec<usize>>;

fn parse(input: &str) -> Parsed {
    input
        .trim()
        .split(',')
        .map(|part| part.chars().map(|char| char as usize).collect())
        .collect()
}

fn hash(chars: &[usize]) -> usize {
    chars.iter().fold(0, |acc, char| ((acc + char) * 17) % 256)
}

pub mod part1 {
    use super::{hash, Parsed};

    pub fn solve(parsed: Parsed) -> usize {
        parsed.iter().map(|part| hash(part)).sum()
    }
}

pub mod part2 {
    use super::{hash, Parsed};

    #[derive(Copy, Clone, Debug)]
    struct Lens {
        id: usize,
        focal_length: usize,
    }

    fn compute_id(lens: &[usize]) -> usize {
        lens.iter()
            .zip(0..)
            .map(|(&c, e)| c * 26_usize.pow(e))
            .sum()
    }

    pub fn solve(steps: Parsed) -> usize {
        let mut boxes = vec![Vec::<Lens>::new(); 256];
        for step in steps {
            if step[step.len() - 1] == '-' as usize {
                let hash = hash(&step[..step.len() - 1]);
                let id = compute_id(&step[..step.len() - 1]);
                boxes[hash].retain(|lens| lens.id != id);
            } else {
                let hash = hash(&step[..step.len() - 2]);
                let id = compute_id(&step[..step.len() - 2]);
                let number = step[step.len() - 1] - '0' as usize;
                let lens = Lens {
                    id,
                    focal_length: number,
                };
                if let Some(l) = boxes[hash].iter().position(|lens| lens.id == id) {
                    boxes[hash][l].focal_length = number;
                } else {
                    boxes[hash].push(lens);
                }
            }
        }
        boxes
            .into_iter()
            .enumerate()
            .flat_map(|(h, hash)| {
                hash.into_iter()
                    .enumerate()
                    .map(move |(i, lens)| (h + 1) * (i + 1) * lens.focal_length)
            })
            .sum()
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_15_input.txt").unwrap()
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
