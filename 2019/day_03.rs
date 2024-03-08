//! https://adventofcode.com/2019/day/3
//! https://adventofcode.com/2019/day/3/input

use std::{fs::read_to_string, time::Instant};

type Wire = Vec<(isize, isize)>;

fn parse(input: &str) -> (Wire, Wire) {
    let mut wires = input.lines().map(|line| {
        line.split(',').map(|d| {
            let direction = d.chars().next().unwrap();
            let distance = d[1..].parse::<isize>().unwrap();
            match direction {
                'R' => (distance, 0),
                'L' => (-distance, 0),
                'D' => (0, -distance),
                'U' => (0, distance),
                _ => panic!("Invalid direction: {}", direction),
            }
        })
    });
    (
        wires.next().unwrap().collect(),
        wires.next().unwrap().collect(),
    )
}

pub mod part1 {
    use std::collections::HashSet;

    use super::parse;

    pub fn solve(input: &str) -> usize {
        let (first, second) = parse(input);
        let mut first_points = HashSet::with_capacity(first.len());
        let (mut x, mut y) = (0, 0);
        for (dx, dy) in first {
            let target_x = x + dx;
            while x != target_x {
                x += dx.signum();
                first_points.insert((x, y));
            }
            let target_y = y + dy;
            while y != target_y {
                y += dy.signum();
                first_points.insert((x, y));
            }
        }
        let (mut min_x, mut min_y) = (isize::MAX, isize::MAX);
        (x, y) = (0, 0);
        for (dx, dy) in second {
            let target_x = x + dx;
            while x != target_x {
                x += dx.signum();
                if first_points.contains(&(x, y))
                    && x.unsigned_abs() + y.unsigned_abs()
                        < min_x.unsigned_abs() + min_y.unsigned_abs()
                {
                    (min_x, min_y) = (x, y);
                }
            }
            let target_y = y + dy;
            while y != target_y {
                y += dy.signum();
                if first_points.contains(&(x, y))
                    && x.unsigned_abs() + y.unsigned_abs()
                        < min_x.unsigned_abs() + min_y.unsigned_abs()
                {
                    (min_x, min_y) = (x, y);
                }
            }
        }
        min_x.unsigned_abs() + min_y.unsigned_abs()
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use super::parse;

    pub fn solve(input: &str) -> usize {
        let (first, second) = parse(input);
        let mut first_points = HashMap::with_capacity(first.len());
        let (mut x, mut y) = (0, 0);
        let mut steps = 0;
        for (dx, dy) in first {
            let target_x = x + dx;
            while x != target_x {
                x += dx.signum();
                steps += 1;
                first_points.insert((x, y), steps);
            }
            let target_y = y + dy;
            while y != target_y {
                y += dy.signum();
                steps += 1;
                first_points.insert((x, y), steps);
            }
        }
        let mut min_steps = usize::MAX;
        (x, y) = (0, 0);
        steps = 0;
        for (dx, dy) in second {
            let target_x = x + dx;
            while x != target_x {
                x += dx.signum();
                steps += 1;
                if let Some(first_steps) = first_points.get(&(x, y)) {
                    if steps + first_steps < min_steps {
                        min_steps = steps + first_steps;
                    }
                }
            }
            let target_y = y + dy;
            while y != target_y {
                y += dy.signum();
                steps += 1;
                if let Some(first_steps) = first_points.get(&(x, y)) {
                    if steps + first_steps < min_steps {
                        min_steps = steps + first_steps;
                    }
                }
            }
        }
        min_steps
    }
}

pub fn main(test: bool) {
    let test_input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2019/day_03_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
