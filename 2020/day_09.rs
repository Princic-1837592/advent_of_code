use std::cmp::Ordering;
use std::time::Instant;

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|n| n.parse().unwrap()).collect()
}

fn find_invalid(numbers: &[usize], size: usize) -> usize {
    let mut previous: Vec<_> = numbers.iter().take(size).copied().collect();
    previous.sort();
    numbers
        .iter()
        .skip(size)
        .enumerate()
        .filter_map(|(i, &n)| {
            let result = if has_pair(&previous, n, size) {
                None
            } else {
                Some(n)
            };
            previous.remove(previous.binary_search(&numbers[i]).unwrap());
            previous.insert(previous.binary_search(&n).unwrap_or_else(|e| e), n);
            result
        })
        .take(1)
        .next()
        .unwrap()
}

fn has_pair(previous: &[usize], number: usize, size: usize) -> bool {
    let (mut left, mut right) = (0, size - 1);
    while left < right {
        match number.cmp(&(previous[left] + previous[right])) {
            Ordering::Greater => left += 1,
            Ordering::Equal => return true,
            Ordering::Less => right -= 1,
        }
    }
    false
}

mod part1 {
    use crate::{find_invalid, parse};

    pub(crate) fn solve(input: &str, size: usize) -> usize {
        let numbers = parse(input);
        find_invalid(&numbers, size)
    }
}

mod part2 {
    use std::cmp::Ordering;

    use crate::{find_invalid, parse};

    pub(crate) fn solve(input: &str, size: usize) -> usize {
        let numbers = parse(input);
        let invalid = find_invalid(&numbers, size);
        let (mut left, mut right) = (0, 1);
        let mut tot = numbers[left] + numbers[right];
        loop {
            match tot.cmp(&invalid) {
                Ordering::Less => {
                    right += 1;
                    tot += numbers[right];
                }
                Ordering::Equal => break,
                Ordering::Greater => {
                    tot -= numbers[left];
                    left += 1;
                }
            }
        }
        let interval: Vec<_> = numbers.iter().skip(left).take(right - left + 1).collect();
        *interval.iter().min().unwrap() + *interval.iter().max().unwrap()
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576"
    .to_owned();
    let (puzzle_input, size) = if test {
        (test_input, 5)
    } else {
        (
            std::fs::read_to_string("inputs/day_09_input.txt").unwrap(),
            25,
        )
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
}
}
