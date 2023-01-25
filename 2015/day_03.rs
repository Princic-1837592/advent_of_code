//! https://adventofcode.com/2015/day/3
//! https://adventofcode.com/2015/day/3/input

use std::{fs::read_to_string, time::Instant};

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
        for i in (0..input.len()).step_by(2) {
            santa = input.chars().nth(i).unwrap();
            robot = input.chars().nth(i + 1).unwrap();
            move_coord(&mut santa_coord, santa);
            move_coord(&mut robot_coord, robot);
            houses.insert(santa_coord);
            houses.insert(robot_coord);
        }
        houses.len()
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
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
