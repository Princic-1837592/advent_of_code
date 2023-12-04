//! https://adventofcode.com/2017/day/21
//! https://adventofcode.com/2017/day/21/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
enum Rotation {
    None,
    Right,
    Double,
    Left,
}

#[derive(Copy, Clone, Debug)]
struct Transformation {
    rotation: Rotation,
    flipped: bool,
}

impl Transformation {
    fn apply_2(&self, mut rule: usize) -> usize {
        // ab
        // cd
        // -> rotate ->
        // ca
        // db
        // -> flip ->
        // ac
        // bd
        for _ in 0..self.rotation as u8 {
            let mut rotated = 0;
            rotated |= (rule & 0b10_00) >> 1; // 01_00
            rotated |= (rule & 0b01_00) >> 2; // 00_01
            rotated |= (rule & 0b00_10) << 2; // 10_00
            rotated |= (rule & 0b00_01) << 1; // 00_10
            rule = rotated;
        }
        if self.flipped {
            let mut flipped = 0;
            flipped |= (rule & 0b10_00) >> 1;
            flipped |= (rule & 0b01_00) << 1;
            flipped |= (rule & 0b00_10) >> 1;
            flipped |= (rule & 0b00_01) << 1;
            rule = flipped;
        }
        rule
    }

    #[allow(clippy::unusual_byte_groupings)]
    #[allow(clippy::identity_op)]
    fn apply_3(&self, mut rule: usize) -> usize {
        // abc
        // def
        // ghi
        // -> rotate ->
        // gda
        // heb
        // ifc
        // -> flip ->
        // adg
        // beh
        // cfi
        for _ in 0..self.rotation as u8 {
            let mut rotated = 0;
            rotated |= (rule & 0b100_000_000) >> 2;
            rotated |= (rule & 0b010_000_000) >> 4;
            rotated |= (rule & 0b001_000_000) >> 6;
            rotated |= (rule & 0b000_100_000) << 2;
            rotated |= (rule & 0b000_010_000) >> 0;
            rotated |= (rule & 0b000_001_000) >> 2;
            rotated |= (rule & 0b000_000_100) << 6;
            rotated |= (rule & 0b000_000_010) << 4;
            rotated |= (rule & 0b000_000_001) << 2;
            rule = rotated;
        }
        if self.flipped {
            let mut flipped = 0;
            flipped |= (rule & 0b100_000_000) >> 2;
            flipped |= (rule & 0b010_000_000) >> 0;
            flipped |= (rule & 0b001_000_000) << 2;
            flipped |= (rule & 0b000_100_000) >> 2;
            flipped |= (rule & 0b000_010_000) >> 0;
            flipped |= (rule & 0b000_001_000) << 2;
            flipped |= (rule & 0b000_000_100) >> 2;
            flipped |= (rule & 0b000_000_010) >> 0;
            flipped |= (rule & 0b000_000_001) << 2;
            rule = flipped;
        }
        rule
    }
}

const TRANSFORMATIONS: [Transformation; 8] = [
    Transformation {
        rotation: Rotation::None,
        flipped: false,
    },
    Transformation {
        rotation: Rotation::Left,
        flipped: false,
    },
    Transformation {
        rotation: Rotation::Double,
        flipped: false,
    },
    Transformation {
        rotation: Rotation::Right,
        flipped: false,
    },
    Transformation {
        rotation: Rotation::None,
        flipped: true,
    },
    Transformation {
        rotation: Rotation::Left,
        flipped: true,
    },
    Transformation {
        rotation: Rotation::Double,
        flipped: true,
    },
    Transformation {
        rotation: Rotation::Right,
        flipped: true,
    },
];

fn parse(input: &str) -> [Vec<usize>; 2] {
    fn to_number(rule: &str) -> usize {
        rule.chars()
            .filter_map(|char| match char {
                '.' => Some(0),
                '#' => Some(1),
                _ => None,
            })
            .fold(0, |acc, bit| (acc << 1) | bit)
    }
    let mut result = [vec![0; 1 << 4], vec![0; 1 << 9]];
    for line in input.lines() {
        let mut parts = line.split(" => ");
        let (left, right) = (parts.next().unwrap(), parts.next().unwrap());
        let from = to_number(left);
        let to = to_number(right);
        for transformation in TRANSFORMATIONS {
            if left.len() == 5 {
                result[0][transformation.apply_2(from)] = to;
            } else {
                result[1][transformation.apply_3(from)] = to;
            }
        }
    }
    result
}

fn expand(rules: [Vec<usize>; 2], iterations: usize) -> Vec<Vec<usize>> {
    let mut image = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1]];
    for _ in 0..iterations {
        let sub_size = 2 + image.len() % 2;
        let new_size = image.len() + image.len() / sub_size;
        let mut new_image = vec![vec![0; new_size]; new_size];
        for gi in 0..image.len() / sub_size {
            for gj in 0..image[0].len() / sub_size {
                let mut from = 0;
                for i in 0..sub_size {
                    for j in 0..sub_size {
                        from = (from << 1) | image[gi * sub_size + i][gj * sub_size + j];
                    }
                }
                let to = rules[sub_size % 2][from];
                let sub_size = sub_size + 1;
                let mut mask = 1 << (sub_size * sub_size);
                for i in 0..sub_size {
                    for j in 0..sub_size {
                        mask >>= 1;
                        new_image[gi * sub_size + i][gj * sub_size + j] =
                            if to & mask == 0 { 0 } else { 1 };
                    }
                }
            }
        }
        image = new_image;
    }
    image
}

fn count(image: Vec<Vec<usize>>) -> usize {
    image.iter().map(|line| line.iter().sum::<usize>()).sum()
}

fn generic_solve(input: &str, iterations: usize) -> usize {
    let rules = parse(input);
    let image = expand(rules, iterations);
    count(image)
}

pub mod part1 {
    use super::generic_solve;

    pub fn solve(input: &str) -> usize {
        generic_solve(input, 5)
    }
}

pub mod part2 {
    use super::generic_solve;

    pub fn solve(input: &str) -> usize {
        generic_solve(input, 18)
    }
}

pub fn main(test: bool) {
    let test_input = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_21_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
