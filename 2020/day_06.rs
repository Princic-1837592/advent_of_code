use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use std::collections::HashSet;

    use crate::LINE_ENDING;

    pub fn solve(input: &str) -> usize {
        input
            .split(&LINE_ENDING.repeat(2))
            .map(|g| {
                g.lines()
                    .flat_map(|l| l.chars())
                    .collect::<HashSet<_>>()
                    .len()
            })
            .sum()
    }
}

pub mod part2 {
    use crate::LINE_ENDING;

    pub fn solve(input: &str) -> usize {
        input
            .split(&LINE_ENDING.repeat(2))
            .map(|g| {
                let mut people = g.lines().map(|l| l.chars().collect::<Vec<_>>());
                let mut first = people.next().unwrap();
                people.for_each(|p| first.retain(|e| p.contains(e)));
                first.len()
            })
            .sum()
    }
}

pub fn main(test: bool) {
    let test_input = "abc

a
b
c

ab
ac

a
a
a
a

b"
    .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2020/day_06_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
