//! https://adventofcode.com/2015/day/3
//! https://adventofcode.com/2015/day/3/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

pub mod part1 {
    use std::collections::HashSet;

    pub fn solve(input: &str) -> usize {
        let mut houses = HashSet::new();
        let mut coord = (0, 0);
        houses.insert(coord);
        for c in input.chars() {
            match c {
                '^' => coord.1 += 1,
                'v' => coord.1 -= 1,
                '>' => coord.0 += 1,
                '<' => coord.0 -= 1,
                _ => (),
            }
            houses.insert(coord);
        }
        houses.len()
    }
}

pub mod part2 {
    use std::collections::HashSet;

    fn move_coord(coord: &mut (i32, i32), direction: char) {
        match direction {
            '^' => coord.1 += 1,
            'v' => coord.1 -= 1,
            '>' => coord.0 += 1,
            '<' => coord.0 -= 1,
            _ => (),
        }
    }

    pub fn solve(input: &str) -> usize {
        let mut houses = HashSet::new();
        let mut santa_coord = (0, 0);
        let mut robot_coord = (0, 0);
        houses.insert(santa_coord);
        houses.insert(robot_coord);
        let (mut santa, mut robot);
        let mut chars = input.chars();
        for _ in (0..input.len()).step_by(2) {
            santa = chars.next().unwrap();
            robot = chars.next().unwrap();
            move_coord(&mut santa_coord, santa);
            move_coord(&mut robot_coord, robot);
            houses.insert(santa_coord);
            houses.insert(robot_coord);
        }
        houses.len()
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2015/day_03_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let result = part1::solve(&puzzle_input);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(&puzzle_input);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
