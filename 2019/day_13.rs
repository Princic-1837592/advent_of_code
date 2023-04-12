//! https://adventofcode.com/2019/day/13
//! https://adventofcode.com/2019/day/13/input

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use crate::int_code::parse;

    pub fn solve(input: &str) -> usize {
        let mut vm = parse(input);
        vm.run_until_complete();
        vm.get_output()
            .iter()
            .enumerate()
            .filter(|&(i, &o)| i % 3 == 2 && o == 2)
            .count()
    }
}

pub mod part2 {
    use crate::int_code::{parse, Interrupt};

    pub fn solve(input: &str) -> isize {
        let mut vm = parse(&"2".chars().chain(input.chars().skip(1)).collect::<String>());
        let (mut ball, mut pad): (isize, isize) = (0, 0);
        let mut outputs = [0; 3];
        let mut output = 0;
        let mut points = 0;
        loop {
            match vm.run_until_interrupt() {
                Interrupt::Input => vm.push_input((ball - pad).signum() as isize),
                Interrupt::Output(value) => {
                    outputs[output] = value;
                    output += 1;
                    if output == 3 {
                        let [x, y, tile_id] = outputs;
                        if (x, y) == (-1, 0) {
                            points = tile_id;
                        } else if tile_id == 3 {
                            pad = x;
                        } else if tile_id == 4 {
                            ball = x;
                        }
                        output = 0
                    }
                }
                Interrupt::Halt => break,
                Interrupt::Error => break,
            }
        }
        points
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_13_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
