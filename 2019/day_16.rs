//! https://adventofcode.com/2019/day/16
//! https://adventofcode.com/2019/day/16/input

use std::{fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<isize> {
    input
        .chars()
        .map(|char| char.to_digit(10).unwrap() as isize)
        .collect()
}

pub mod part1 {
    use crate::day_16::parse;

    const PATTERN: [isize; 4] = [0, 1, 0, -1];

    #[derive(Copy, Clone, Debug)]
    struct Pattern {
        position: usize,
        consumed: usize,
        current: usize,
    }

    impl Pattern {
        fn new(position: usize) -> Self {
            Self {
                position: position + 1,
                consumed: 0,
                current: 0,
            }
        }
    }

    impl Iterator for Pattern {
        type Item = isize;

        fn next(&mut self) -> Option<Self::Item> {
            self.consumed += 1;
            if self.consumed % self.position == 0 {
                self.current = (self.current + 1) % 4;
            }
            let current = self.current;
            Some(PATTERN[current])
        }
    }

    fn phase(list: &mut Vec<isize>, support: &mut Vec<isize>) {
        for (i, s) in support.iter_mut().enumerate() {
            *s = (list
                .iter()
                .zip(Pattern::new(i))
                .map(|(e, c)| e * c)
                .sum::<isize>()
                % 10)
                .abs();
        }
        list.clone_from(support);
    }

    pub fn solve(input: &str) -> isize {
        let mut list = parse(input);
        let mut support = list.clone();
        for _ in 0..100 {
            phase(&mut list, &mut support);
        }
        list.iter()
            .take(8)
            .fold((0, 10_000_000), |(acc, coefficient), n| {
                (acc + n * coefficient, coefficient / 10)
            })
            .0
    }
}

pub mod part2 {
    use crate::day_16::parse;

    pub fn solve(input: &str) -> isize {
        let list = parse(input);
        let offset = list
            .iter()
            .take(7)
            .fold((0, 1_000_000), |(acc, coefficient), n| {
                (acc + n * coefficient, coefficient / 10)
            })
            .0 as usize;
        let len = list.len() * 10_000;
        let mut slice = Vec::with_capacity(len - offset);
        for i in offset..len {
            slice.push(list[i % list.len()]);
        }
        let mut sums = vec![0; slice.len() + 1];
        for _phase in 0..100 {
            let mut total = 0;
            for (n, i) in slice.iter().zip(1..) {
                total += n;
                sums[i] = total;
            }
            for i in 0..slice.len() {
                slice[i] = (total - sums[i]) % 10;
            }
        }
        slice
            .iter()
            .take(8)
            .fold((0, 10_000_000), |(acc, coefficient), n| {
                (acc + n * coefficient, coefficient / 10)
            })
            .0
    }
}

pub fn main(test: bool) {
    let test_input = "03036732577212944063491565474664".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_16_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
