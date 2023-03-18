//! https://adventofcode.com/2016/day/17
//! https://adventofcode.com/2016/day/17/input

use std::{fs::read_to_string, time::Instant};

const DIRECTIONS: [(char, (isize, isize)); 4] =
    [('U', (0, -1)), ('D', (0, 1)), ('L', (-1, 0)), ('R', (1, 0))];

pub mod part1 {
    use std::collections::VecDeque;

    use md5::compute;

    use crate::day_17::DIRECTIONS;

    pub fn solve(input: &str) -> String {
        let mut queue = VecDeque::from([((0, 0), input.to_owned())]);
        while let Some((coord @ (x, y), path)) = queue.pop_front() {
            if coord == (3, 3) {
                return path.chars().skip(input.len()).collect();
            }
            let hash: Vec<_> = format!("{:?}", compute(&path)).chars().take(4).collect();
            for (i, (direction, (dx, dy))) in DIRECTIONS.iter().enumerate() {
                if ('b'..='f').contains(&hash[i])
                    && (0..=3).contains(&(x + dx))
                    && (0..=3).contains(&(y + dy))
                {
                    queue.push_back(((x + dx, y + dy), format!("{}{}", path, direction)));
                }
            }
        }
        unreachable!()
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    use md5::compute;

    use crate::day_17::DIRECTIONS;

    pub fn solve(input: &str) -> usize {
        let mut longest = 0;
        let mut queue = VecDeque::from([((0, 0), input.to_owned())]);
        while let Some((coord @ (x, y), path)) = queue.pop_front() {
            if coord == (3, 3) {
                longest = longest.max(path.len());
                continue;
            }
            let hash: Vec<_> = format!("{:?}", compute(&path)).chars().take(4).collect();
            for (i, (direction, (dx, dy))) in DIRECTIONS.iter().enumerate() {
                if ('b'..='f').contains(&hash[i])
                    && (0..=3).contains(&(x + dx))
                    && (0..=3).contains(&(y + dy))
                {
                    queue.push_back(((x + dx, y + dy), format!("{}{}", path, direction)));
                }
            }
        }
        longest - input.len()
    }
}

pub fn main(test: bool) {
    let test_input = "ulqzkmiv".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_17_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
