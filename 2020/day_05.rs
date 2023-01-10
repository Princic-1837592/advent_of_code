use std::time::Instant;

fn find_seat_id(seat: &&str) -> usize {
    let (mut front, mut back) = (0, 127);
    let (mut left, mut right) = (0, 7);
    for c in seat.chars() {
        match c {
            'F' => back = (front + back) / 2,
            'B' => front = (front + back) / 2 + 1,
            'L' => right = (left + right) / 2,
            _r => left = (left + right) / 2 + 1,
        }
    }
    front * 8 + left
}

mod part1 {
    use rayon::prelude::*;

    use crate::find_seat_id;

    pub(crate) fn solve(input: &str) -> usize {
        let seats: Vec<_> = input.lines().collect();
        seats.par_iter().map(find_seat_id).max().unwrap()
    }
}

mod part2 {
    use std::{cmp::Ordering, usize};

    use rayon::prelude::*;

    use crate::find_seat_id;

    pub(crate) fn solve(input: &str) -> usize {
        let mut seats: Vec<_> = input
            .lines()
            .collect::<Vec<_>>()
            .par_iter()
            .map(find_seat_id)
            .collect();
        seats.sort_unstable();
        let (min, _max) = (seats[0], seats[seats.len() - 1]);
        let (mut left, mut right) = (0, seats.len() - 1);
        while left < right {
            let mid = (left + right) / 2;
            let mid_value = mid + min;
            match seats[mid].cmp(&mid_value) {
                Ordering::Equal | Ordering::Less => left = mid,
                Ordering::Greater => {
                    if seats[mid - 1] == seats[mid] - 2 {
                        return seats[mid] - 1;
                    }
                    right = mid
                }
            }
        }
        panic!("Couldn't find the solution!")
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "BBFFBBFRLL".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_05_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
}
