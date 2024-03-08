//! https://adventofcode.com/2019/day/15
//! https://adventofcode.com/2019/day/15/input

use std::{
    collections::{hash_map::Entry, HashMap},
    fs::read_to_string,
    time::Instant,
};

use crate::int_code::{IntCode, Interrupt};

type Coord = (isize, isize);

const NEIGHBORS: [(isize, isize); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

#[derive(Copy, Clone, Debug)]
enum Status {
    Wall,
    Empty,
    Oxygen,
}

fn build_maze(mut vm: IntCode) -> (HashMap<Coord, Status>, Coord) {
    let mut maze = HashMap::from([((0, 0), Status::Empty)]);
    let mut direction_stack = vec![1, 2, 2, 1, 3, 4, 4, 3];
    let mut position_stack = vec![(0, 0)];
    let directions = [(0, 0), (0, 1), (0, -1), (-1, 0), (1, 0)];
    let mut oxygen = (0, 0);
    loop {
        match vm.run_until_interrupt() {
            Interrupt::Input => {
                if let Some(direction) = direction_stack.pop() {
                    vm.push_input(direction);
                    let (x, y) = position_stack.last().unwrap();
                    let (dx, dy) = directions[direction as usize];
                    position_stack.push((x + dx, y + dy));
                } else {
                    break;
                }
            }
            Interrupt::Output(value) => {
                let &position = position_stack.last().unwrap();
                match value {
                    0 => {
                        maze.insert(position, Status::Wall);
                        position_stack.pop();
                        direction_stack.pop();
                    }
                    1 => {
                        if let Entry::Vacant(e) = maze.entry(position) {
                            e.insert(Status::Empty);
                            direction_stack.extend([1, 2, 2, 1, 3, 4, 4, 3]);
                        }
                    }
                    2 => {
                        maze.insert(position, Status::Oxygen);
                        oxygen = position;
                    }
                    _ => {}
                }
            }
            Interrupt::Halt => break,
            Interrupt::Error => break,
        }
    }
    (maze, oxygen)
}

pub mod part1 {
    use std::collections::{HashSet, VecDeque};

    use crate::{
        day_15::{build_maze, Status, NEIGHBORS},
        int_code::parse,
    };

    pub fn solve(input: &str) -> usize {
        let vm = parse(input);
        let (maze, _) = build_maze(vm);
        let mut queue = VecDeque::from([((0, 0), 0)]);
        let mut visited = HashSet::new();
        while let Some((coord @ (i, j), dist)) = queue.pop_front() {
            if visited.contains(&coord) {
                continue;
            }
            visited.insert(coord);
            match maze.get(&coord).unwrap() {
                Status::Empty => {
                    for neighbor in NEIGHBORS.map(|(di, dj)| (i + di, j + dj)) {
                        queue.push_back((neighbor, dist + 1));
                    }
                }
                Status::Oxygen => return dist,
                Status::Wall => {}
            }
        }
        unreachable!()
    }
}

pub mod part2 {
    use std::collections::{HashSet, VecDeque};

    use crate::{
        day_15::{build_maze, Status, NEIGHBORS},
        int_code::parse,
    };

    pub fn solve(input: &str) -> usize {
        let vm = parse(input);
        let (maze, oxygen) = build_maze(vm);
        let mut queue = VecDeque::from([(oxygen, 0)]);
        let mut visited = HashSet::new();
        let mut last_distance = 0;
        while let Some((coord @ (i, j), dist)) = queue.pop_front() {
            if visited.contains(&coord) {
                continue;
            }
            last_distance = dist;
            visited.insert(coord);
            match maze.get(&coord).unwrap() {
                Status::Empty | Status::Oxygen => {
                    for neighbor in NEIGHBORS.map(|(di, dj)| (i + di, j + dj)) {
                        queue.push_back((neighbor, dist + 1));
                    }
                }
                Status::Wall => {}
            }
        }
        last_distance
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2019/day_15_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
