//! https://adventofcode.com/2018/day/3
//! https://adventofcode.com/2018/day/3/input

use std::{fs::read_to_string, time::Instant};

use regex::Regex;

#[derive(Copy, Clone, Debug, Default)]
struct Rectangle {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

fn parse(input: &str) -> Vec<Rectangle> {
    let pattern = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            Rectangle {
                id: captures[1].parse().unwrap(),
                left: captures[2].parse().unwrap(),
                top: captures[3].parse().unwrap(),
                width: captures[4].parse().unwrap(),
                height: captures[5].parse().unwrap(),
            }
        })
        .collect()
}

pub mod part1 {
    use std::collections::HashMap;

    use crate::day_03::parse;

    pub fn solve(input: &str) -> usize {
        let rectangles = parse(input);
        let mut fabric = HashMap::new();
        for rectangle in rectangles {
            for i in rectangle.top..rectangle.top + rectangle.height {
                for j in rectangle.left..rectangle.left + rectangle.width {
                    *fabric.entry((i, j)).or_insert(0) += 1;
                }
            }
        }
        fabric.values().filter(|&&v| v >= 2).count()
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use crate::day_03::parse;

    pub fn solve(input: &str) -> usize {
        let rectangles = parse(input);
        let mut fabric = HashMap::new();
        for rectangle in &rectangles {
            for i in rectangle.top..rectangle.top + rectangle.height {
                for j in rectangle.left..rectangle.left + rectangle.width {
                    *fabric.entry((i, j)).or_insert(0) += 1;
                }
            }
        }
        for rectangle in rectangles {
            let mut overlaps = false;
            for i in rectangle.top..rectangle.top + rectangle.height {
                for j in rectangle.left..rectangle.left + rectangle.width {
                    if let Some(n) = fabric.get(&(i, j)) {
                        if *n > 1 {
                            overlaps = true;
                        }
                    }
                }
                if overlaps {
                    break;
                }
            }
            if !overlaps {
                return rectangle.id;
            }
        }
        0
    }
}

pub fn main(test: bool) {
    let test_input = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_03_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
