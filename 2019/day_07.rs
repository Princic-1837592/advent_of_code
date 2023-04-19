//! https://adventofcode.com/2019/day/7
//! https://adventofcode.com/2019/day/7/input

use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use itertools::Itertools;
    use rayon::prelude::*;

    use crate::int_code::{parse, IntCode};

    fn run_with_phases(vm: &IntCode, phases: Vec<isize>) -> isize {
        let mut output = 0;
        for phase in &phases {
            let mut vm = vm.clone();
            vm.push_input(*phase);
            vm.push_input(output);
            vm.run_until_complete();
            output = vm.last_output().unwrap();
        }
        output
    }

    pub fn solve(input: &str) -> isize {
        let vm = parse(input);
        (0..=4)
            .permutations(5)
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|comb| run_with_phases(&vm, comb))
            .max()
            .unwrap()
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    use itertools::Itertools;
    use rayon::prelude::*;

    use crate::int_code::{parse, IntCode, Interrupt};

    fn run_with_phases(vm: &IntCode, phases: Vec<isize>) -> isize {
        let mut vms: Vec<_> = vec![vm.clone(); phases.len()];
        let mut input_queues: Vec<_> = phases
            .iter()
            .map(|&phase| VecDeque::from([phase]))
            .collect();
        input_queues[0].push_back(0);
        let mut run = true;
        while run {
            run = false;
            for (i, vm) in vms.iter_mut().enumerate() {
                match vm.run_until_interrupt() {
                    Interrupt::Input => {
                        if let Some(value) = input_queues[i].pop_front() {
                            vm.push_input(value);
                            run = true;
                        }
                    }
                    Interrupt::Output(value) => {
                        input_queues[(i + 1) % phases.len()].push_back(value);
                        run = true;
                    }
                    _ => {}
                }
            }
        }
        vms[vms.len() - 1].last_output().unwrap()
    }

    pub fn solve(input: &str) -> isize {
        let vm = parse(input);
        (5..=9)
            .permutations(5)
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|comb| run_with_phases(&vm, comb))
            .max()
            .unwrap()
    }
}

pub fn main(test: bool) {
    let test_input =
        "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_07_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
