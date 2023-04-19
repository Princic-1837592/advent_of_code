//! https://adventofcode.com/2019/day/23
//! https://adventofcode.com/2019/day/23/input

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use std::collections::VecDeque;

    use crate::int_code::{parse, Interrupt};

    pub fn solve(input: &str) -> isize {
        let mut computers = vec![parse(input); 50];
        let mut input_queues: Vec<_> = (0..50).map(|address| VecDeque::from([address])).collect();
        loop {
            for (i, computer) in computers.iter_mut().enumerate() {
                match computer.run_until_interrupt() {
                    Interrupt::Input => {
                        computer.push_input(input_queues[i].pop_front().unwrap_or(-1));
                    }
                    Interrupt::Output(address) => {
                        computer.run_until_interrupt();
                        let x = computer.last_output().unwrap();
                        computer.run_until_interrupt();
                        let y = computer.last_output().unwrap();
                        let address = address as usize;
                        if address == 255 {
                            return y;
                        }
                        input_queues[address].push_back(x);
                        input_queues[address].push_back(y);
                    }
                    _ => {}
                }
            }
        }
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    use crate::int_code::{parse, Interrupt};

    pub fn solve(input: &str) -> isize {
        let mut computers = vec![parse(input); 50];
        let mut input_queues: Vec<_> = (0..50).map(|address| VecDeque::from([address])).collect();
        let mut nat = (isize::MIN, isize::MIN);
        let mut last_y_sent = 0;
        loop {
            let mut idle = true;
            for (i, computer) in computers.iter_mut().enumerate() {
                match computer.run_until_interrupt() {
                    Interrupt::Input => {
                        computer.push_input(if let Some(input) = input_queues[i].pop_front() {
                            idle = false;
                            input
                        } else {
                            -1
                        });
                    }
                    Interrupt::Output(address) => {
                        computer.run_until_interrupt();
                        let x = computer.last_output().unwrap();
                        computer.run_until_interrupt();
                        let y = computer.last_output().unwrap();
                        let address = address as usize;
                        if address == 255 {
                            nat = (x, y);
                        } else {
                            input_queues[address].push_back(x);
                            input_queues[address].push_back(y);
                        }
                        idle = false;
                    }
                    Interrupt::Halt => {}
                    Interrupt::Error => {}
                }
            }
            if idle && input_queues.iter().all(|queue| queue.is_empty()) {
                if nat.1 == last_y_sent {
                    return last_y_sent;
                }
                input_queues[0].push_back(nat.0);
                input_queues[0].push_back(nat.1);
                last_y_sent = nat.1;
            }
        }
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_23_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
