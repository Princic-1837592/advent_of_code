//! https://adventofcode.com/2018/day/14
//! https://adventofcode.com/2018/day/14/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> usize {
    input.parse().unwrap()
}

pub mod part1 {
    use crate::day_14::parse;

    pub fn solve(input: &str) -> String {
        let target = parse(input) + 10;
        let mut recipes = vec![3, 7];
        let (mut first, mut second) = (0, 1);
        while recipes.len() <= target {
            let sum = recipes[first] + recipes[second];
            if sum >= 10 {
                recipes.push(sum / 10);
            }
            recipes.push(sum % 10);
            first = (first + recipes[first] + 1) % recipes.len();
            second = (second + recipes[second] + 1) % recipes.len();
        }
        recipes
            .iter()
            .skip(target - 10)
            .take(10)
            .map(|recipe| (*recipe as u8 + b'0') as char)
            .collect()
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    pub fn solve(input: &str) -> usize {
        let target = VecDeque::from_iter(input.chars().map(|char| (char as u8 - b'0') as usize));
        let mut recipes = vec![3, 7];
        let (mut first, mut second) = (0, 1);
        while recipes.len() <= target.len() {
            let sum = recipes[first] + recipes[second];
            if sum >= 10 {
                recipes.push(sum / 10);
            }
            recipes.push(sum % 10);
            first = (first + recipes[first] + 1) % recipes.len();
            second = (second + recipes[second] + 1) % recipes.len();
        }
        let mut current = VecDeque::with_capacity(target.len());
        for recipe in recipes.iter().take(target.len()) {
            current.push_back(*recipe);
        }
        let mut next_recipe = current.len();
        let offset = recipes.len() - current.len();
        while current != target {
            let sum = recipes[first] + recipes[second];
            if sum >= 10 {
                current.pop_front();
                recipes.push(sum / 10);
                current.push_back(recipes[next_recipe]);
                next_recipe += 1;
            }
            if current == target {
                break;
            }
            current.pop_front();
            recipes.push(sum % 10);
            current.push_back(recipes[next_recipe]);
            next_recipe += 1;
            first = (first + recipes[first] + 1) % recipes.len();
            second = (second + recipes[second] + 1) % recipes.len();
        }
        recipes.len() - target.len() - offset
    }
}

pub fn main(test: bool) {
    let test_input = "59414".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_14_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
