//! https://adventofcode.com/2017/day/10
//! https://adventofcode.com/2017/day/10/input

use std::{fs::read_to_string, time::Instant};

fn reverse(numbers: &mut [usize], from: usize, length: usize) {
    for i in 0..length / 2 {
        numbers.swap(
            (from + i) % numbers.len(),
            (from + length - i - 1) % numbers.len(),
        );
    }
}

fn round(
    numbers: &mut [usize],
    lengths: &[usize],
    mut cp: usize,
    mut skip_size: usize,
) -> (usize, usize) {
    for &length in lengths {
        reverse(numbers, cp, length);
        cp += length + skip_size;
        skip_size += 1;
    }
    (cp, skip_size)
}

pub mod part1 {
    use crate::day_10::round;

    fn parse(input: &str) -> Vec<usize> {
        input.split(',').map(|n| n.parse().unwrap()).collect()
    }

    pub fn solve(input: &str) -> usize {
        let lengths = parse(input);
        let mut numbers: Vec<_> = (0..256).collect();
        round(&mut numbers, &lengths, 0, 0);
        numbers[0] * numbers[1]
    }
}

pub mod part2 {
    use crate::day_10::round;

    fn parse(input: &str) -> Vec<usize> {
        input
            .chars()
            .map(|char| char as u8 as usize)
            .chain([17, 31, 73, 47, 23])
            .collect()
    }

    pub fn solve(input: &str) -> String {
        let lengths = parse(input);
        let mut numbers: Vec<_> = (0..256).collect();
        let (mut cp, mut skip_size) = (0, 0);
        for _ in 0..64 {
            (cp, skip_size) = round(&mut numbers, &lengths, cp, skip_size);
            cp %= numbers.len();
            skip_size %= numbers.len();
        }
        let mut xors = vec![0; 16];
        for i in 0..16 {
            let mut xor = 0;
            for j in 0..16 {
                xor ^= numbers[i * 16 + j];
            }
            xors[i] = xor;
        }
        let mut result = String::new();
        for xor in xors {
            result.push_str(&*format!("{:0>2x}", xor));
        }
        result
    }
}

pub fn main(test: bool) {
    let test_input = "1,2,4".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_10_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
