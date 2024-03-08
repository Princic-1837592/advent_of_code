//! https://adventofcode.com/2017/day/24
//! https://adventofcode.com/2017/day/24/input

use std::{fs::read_to_string, time::Instant};

type Port = (usize, usize);

fn parse(input: &str) -> Vec<Port> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split('/');
            (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

pub mod part1 {
    use std::collections::HashSet;

    use super::{parse, Port};

    fn explore(ports: &[Port]) -> usize {
        fn internal(
            ports: &[Port],
            used: usize,
            last: usize,
            strength: usize,
            seen: &mut HashSet<usize>,
        ) -> usize {
            if seen.contains(&used) {
                return 0;
            }
            seen.insert(used);
            let mut max = strength;
            let mut mask = 1 << ports.len();
            for port in ports {
                mask >>= 1;
                if used & mask == 0 && port.0 == last || port.1 == last {
                    let new_port = port.0 + port.1 - last;
                    max = max.max(internal(
                        ports,
                        used | mask,
                        new_port,
                        strength + port.0 + port.1,
                        seen,
                    ));
                }
            }
            max
        }
        internal(ports, 0, 0, 0, &mut HashSet::new())
    }

    pub fn solve(input: &str) -> usize {
        let ports = parse(input);
        explore(&ports)
    }
}

pub mod part2 {
    use std::collections::HashSet;

    use super::{parse, Port};

    fn explore(ports: &[Port]) -> usize {
        fn internal(
            ports: &[Port],
            used: usize,
            last: usize,
            value @ (length, strength): (usize, usize),
            seen: &mut HashSet<usize>,
        ) -> (usize, usize) {
            if seen.contains(&used) {
                return (0, 0);
            }
            seen.insert(used);
            let mut local_max = value;
            let mut mask = 1 << ports.len();
            for port in ports {
                mask >>= 1;
                if used & mask == 0 && port.0 == last || port.1 == last {
                    let new_port = port.0 + port.1 - last;
                    local_max = local_max.max(internal(
                        ports,
                        used | mask,
                        new_port,
                        (length + 1, strength + port.0 + port.1),
                        seen,
                    ));
                }
            }
            local_max
        }
        internal(ports, 0, 0, (0, 0), &mut HashSet::new()).1
    }

    pub fn solve(input: &str) -> usize {
        let ports = parse(input);
        explore(&ports)
    }
}

pub fn main(test: bool) {
    let test_input = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2017/day_24_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
