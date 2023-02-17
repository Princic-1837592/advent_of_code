//! https://adventofcode.com/2017/day/3
//! https://adventofcode.com/2017/day/3/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> isize {
    input.parse().unwrap()
}

pub mod part1 {
    use crate::day_03::parse;

    pub fn solve(input: &str) -> usize {
        let target = parse(input);
        let mut accumulated = 1;
        let mut size = 1;
        while accumulated < target {
            size += 2;
            accumulated += size * 4 - 4;
        }
        let layer = size / 2;
        let last = (size - 2) * (size - 2);
        let (mut x, mut y) = (layer, layer - 1);
        let mut n = last + 1;
        while y >= -layer {
            if n == target {
                return x.unsigned_abs() + y.unsigned_abs();
            }
            y -= 1;
            n += 1;
        }
        y += 1;
        x -= 1;
        while x >= -layer {
            if n == target {
                return x.unsigned_abs() + y.unsigned_abs();
            }
            x -= 1;
            n += 1;
        }
        x += 1;
        y += 1;
        while y <= layer {
            if n == target {
                return x.unsigned_abs() + y.unsigned_abs();
            }
            y += 1;
            n += 1;
        }
        y -= 1;
        x += 1;
        while x < layer {
            if n == target {
                return x.unsigned_abs() + y.unsigned_abs();
            }
            x += 1;
            n += 1;
        }
        unreachable!()
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use crate::day_03::parse;

    const NEIGHBORS: [(isize, isize); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    fn sum_neighbors((x, y): (isize, isize), cells: &HashMap<(isize, isize), isize>) -> isize {
        let mut sum = 0;
        for neighbor in NEIGHBORS.map(|(nx, ny)| (x + nx, y + ny)) {
            if let Some(&n) = cells.get(&neighbor) {
                sum += n;
            }
        }
        sum
    }

    pub fn solve(input: &str) -> isize {
        let target = parse(input);
        let mut cells = HashMap::from([((0, 0), 1)]);
        let (mut x, mut y) = (1, 0);
        let mut layer = 1;
        let (mut dx, mut dy) = (0, -1);
        loop {
            let mut out = false;
            if y < -layer {
                y += 1;
                out = true;
            }
            if x < -layer {
                x += 1;
                out = true;
            }
            if y > layer {
                y -= 1;
                out = true;
            }
            if x > layer {
                layer += 1;
                out = true;
            }
            if out {
                (dx, dy) = (dy, -dx);
            }
            let n = sum_neighbors((x, y), &cells);
            if n > target {
                return n;
            }
            cells.insert((x, y), n);
            x += dx;
            y += dy;
        }
    }
}

pub fn main(test: bool) {
    let test_input = "747".to_owned();
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
