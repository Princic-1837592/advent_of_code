//! https://adventofcode.com/2015/day/16

use std::time::Instant;

struct Aunt {
    number: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

fn parse(input: &str) -> Vec<Aunt> {
    input
        .lines()
        .map(|line| {
            let mut parts = line
                .split(|c| c == ':' || c == ',' || c == ' ')
                .filter(|part| !part.is_empty());
            let number = parts.nth(1).unwrap().parse().unwrap();
            let mut sue = Aunt {
                number,
                children: None,
                cats: None,
                samoyeds: None,
                pomeranians: None,
                akitas: None,
                vizslas: None,
                goldfish: None,
                trees: None,
                cars: None,
                perfumes: None,
            };
            while let Some(name) = parts.next() {
                let value = parts
                    .next()
                    .unwrap()
                    .parse()
                    .expect(&*format!("Invalid digit {} for {}", name, line));
                match name {
                    "children" => sue.children = Some(value),
                    "cats" => sue.cats = Some(value),
                    "samoyeds" => sue.samoyeds = Some(value),
                    "pomeranians" => sue.pomeranians = Some(value),
                    "akitas" => sue.akitas = Some(value),
                    "vizslas" => sue.vizslas = Some(value),
                    "goldfish" => sue.goldfish = Some(value),
                    "trees" => sue.trees = Some(value),
                    "cars" => sue.cars = Some(value),
                    "perfumes" => sue.perfumes = Some(value),
                    _ => panic!("Unknown name: {}", name),
                }
            }
            sue
        })
        .collect()
}

pub mod part1 {
    use crate::day_16::parse;

    pub fn solve(input: &str) -> usize {
        let aunts = parse(input);
        aunts
            .iter()
            .filter(|aunt| {
                (aunt.children.is_none() || matches!(aunt.children, Some(3)))
                    && (aunt.cats.is_none() || matches!(aunt.cats, Some(7)))
                    && (aunt.samoyeds.is_none() || matches!(aunt.samoyeds, Some(2)))
                    && (aunt.pomeranians.is_none() || matches!(aunt.pomeranians, Some(3)))
                    && (aunt.akitas.is_none() || matches!(aunt.akitas, Some(0)))
                    && (aunt.vizslas.is_none() || matches!(aunt.vizslas, Some(0)))
                    && (aunt.goldfish.is_none() || matches!(aunt.goldfish, Some(2)))
                    && (aunt.trees.is_none() || matches!(aunt.trees, Some(3)))
                    && (aunt.cars.is_none() || matches!(aunt.cars, Some(2)))
                    && (aunt.perfumes.is_none() || matches!(aunt.perfumes, Some(1)))
            })
            .map(|aunt| aunt.number)
            .next()
            .unwrap()
    }
}

pub mod part2 {
    use crate::day_16::parse;

    pub fn solve(input: &str) -> usize {
        let aunts = parse(input);
        aunts
            .iter()
            .filter(|aunt| {
                (aunt.children.is_none() || matches!(aunt.children, Some(3)))
                    && (aunt.cats.is_none() || matches!(aunt.cats, Some(8..)))
                    && (aunt.samoyeds.is_none() || matches!(aunt.samoyeds, Some(2)))
                    && (aunt.pomeranians.is_none() || matches!(aunt.pomeranians, Some(0..=2)))
                    && (aunt.akitas.is_none() || matches!(aunt.akitas, Some(0)))
                    && (aunt.vizslas.is_none() || matches!(aunt.vizslas, Some(0)))
                    && (aunt.goldfish.is_none() || matches!(aunt.goldfish, Some(0..=1)))
                    && (aunt.trees.is_none() || matches!(aunt.trees, Some(4..)))
                    && (aunt.cars.is_none() || matches!(aunt.cars, Some(2)))
                    && (aunt.perfumes.is_none() || matches!(aunt.perfumes, Some(1)))
            })
            .map(|aunt| aunt.number)
            .next()
            .unwrap()
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_16_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
