use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use std::cmp::Ordering;

    pub fn solve(input: &str) -> usize {
        let mut entries: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
        entries.sort();
        let mut left = 0;
        let mut right = entries.len() - 1;
        while left < right {
            let sum = entries[left] + entries[right];
            match sum.cmp(&2020) {
                Ordering::Less => left += 1,
                Ordering::Equal => return entries[left] * entries[right],
                Ordering::Greater => right -= 1,
            }
        }
        panic!("No solution found");
    }
}

pub mod part2 {
    use std::cmp::Ordering;

    pub fn solve(input: &str) -> usize {
        let mut entries: Vec<usize> = input.lines().map(|l| l.parse().unwrap()).collect();
        entries.sort();
        for (i, entry) in entries.iter().enumerate() {
            let mut left = i + 1;
            let mut right = entries.len() - 1;
            while left < right {
                let sum = entry + entries[left] + entries[right];
                match sum.cmp(&2020) {
                    Ordering::Less => left += 1,
                    Ordering::Equal => return entries[i] * entries[left] * entries[right],
                    Ordering::Greater => right -= 1,
                }
            }
        }
        panic!("No solution found");
    }
}

pub fn main(test: bool) {
    let test_input = "1721
979
366
299
675
1456"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2020/day_01_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
