//! https://adventofcode.com/2018/day/23
//! https://adventofcode.com/2018/day/23/input

use std::{fs::read_to_string, time::Instant};

use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Nanobot {
    x: isize,
    y: isize,
    z: isize,
    radius: isize,
}

fn parse(input: &str) -> Vec<Nanobot> {
    let pattern = Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(-?\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = pattern.captures(line).unwrap();
            Nanobot {
                x: captures.get(1).unwrap().as_str().parse().unwrap(),
                y: captures.get(2).unwrap().as_str().parse().unwrap(),
                z: captures.get(3).unwrap().as_str().parse().unwrap(),
                radius: captures.get(4).unwrap().as_str().parse().unwrap(),
            }
        })
        .collect()
}

pub mod part1 {
    use super::parse;

    pub fn solve(input: &str) -> usize {
        let nanobots = parse(input);
        let strongest = nanobots
            .iter()
            .max_by_key(|nanobot| nanobot.radius)
            .unwrap();
        nanobots
            .iter()
            .filter(|nanobot| {
                (nanobot.x - strongest.x).abs()
                    + (nanobot.y - strongest.y).abs()
                    + (nanobot.z - strongest.z).abs()
                    <= strongest.radius
            })
            .count()
    }
}

pub mod part2 {
    use std::{cmp::Ordering, collections::BinaryHeap};

    use super::{parse, Nanobot};

    #[derive(Clone, Debug)]
    struct Space {
        x: isize,
        y: isize,
        z: isize,
        size: isize,
        distance: isize,
        in_range: Vec<usize>,
    }

    impl Space {
        fn new(nanobots: &Vec<Nanobot>) -> Self {
            let (min_x, max_x, min_y, max_y, min_z, max_z) = nanobots.iter().fold(
                (
                    isize::MAX,
                    isize::MIN,
                    isize::MAX,
                    isize::MIN,
                    isize::MAX,
                    isize::MIN,
                ),
                |(min_x, max_x, min_y, max_y, min_z, max_z), nanobot| {
                    (
                        min_x.min(nanobot.x),
                        max_x.max(nanobot.x),
                        min_y.min(nanobot.y),
                        max_y.max(nanobot.y),
                        min_z.min(nanobot.z),
                        max_z.max(nanobot.z),
                    )
                },
            );
            let size = (max_x - min_x).max(max_y - min_y).max(max_z - min_z);
            let mut power_of_two = 1;
            while power_of_two < size {
                power_of_two *= 2;
            }
            Space {
                x: min_x,
                y: min_y,
                z: min_z,
                size: power_of_two,
                distance: Self::from_origin(size, min_x, min_y, min_z),
                in_range: (0..nanobots.len()).collect(),
            }
        }

        fn from(
            space: &Self,
            x: isize,
            y: isize,
            z: isize,
            size: isize,
            nanobots: &[Nanobot],
        ) -> Self {
            Space {
                x,
                y,
                z,
                size,
                distance: Self::from_origin(size, x, y, z),
                in_range: space
                    .in_range
                    .iter()
                    .filter(|i| {
                        Self::distance(x, y, z, size, &nanobots[**i]) <= nanobots[**i].radius
                    })
                    .cloned()
                    .collect(),
            }
        }

        fn from_origin(size: isize, x: isize, y: isize, z: isize) -> isize {
            let (min_x, max_x, min_y, max_y, min_z, max_z) =
                (x, x + size, y, y + size, z, z + size);
            if min_x <= 0 && 0 <= max_x && min_y <= 0 && 0 <= max_y && min_z <= 0 && 0 <= max_z {
                0
            } else {
                let corners = [
                    (min_x, min_y, min_z),
                    (min_x, min_y, max_z),
                    (min_x, max_y, min_z),
                    (min_x, max_y, max_z),
                    (max_x, min_y, min_z),
                    (max_x, min_y, max_z),
                    (max_x, max_y, min_z),
                    (max_x, max_y, max_z),
                ];
                corners
                    .iter()
                    .map(|(x, y, z)| x.abs() + y.abs() + z.abs())
                    .min()
                    .unwrap()
            }
        }

        fn distance(x: isize, y: isize, z: isize, size: isize, nanobot: &Nanobot) -> isize {
            fn range_dist(x: isize, left: isize, right: isize) -> isize {
                if x < left {
                    left - x
                } else if x > right {
                    x - right
                } else {
                    0
                }
            }
            let x1 = x;
            let x2 = x + size;
            let y1 = y;
            let y2 = y + size;
            let z1 = z;
            let z2 = z + size;
            range_dist(nanobot.x, x1, x2)
                + range_dist(nanobot.y, y1, y2)
                + range_dist(nanobot.z, z1, z2)
        }
    }

    impl Eq for Space {}

    impl PartialEq<Self> for Space {
        fn eq(&self, other: &Self) -> bool {
            (self.x, self.y, self.z, self.size) == (other.x, other.y, other.z, other.size)
        }
    }

    impl PartialOrd<Self> for Space {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(
                self.in_range
                    .len()
                    .cmp(&other.in_range.len())
                    .then(self.distance.cmp(&other.distance).reverse())
                    .then(self.size.cmp(&other.size).reverse()),
            )
        }
    }

    impl Ord for Space {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    fn find_best(nanobots: Vec<Nanobot>) -> Space {
        let first = Space::new(&nanobots);
        let mut heap = BinaryHeap::from([first]);
        while let Some(space) = heap.pop() {
            if space.size == 0 {
                return space;
            }
            let new_size = space.size / 2;
            for (dx, dy, dz) in [
                (0, 0, 0),
                (0, 0, new_size),
                (0, new_size, 0),
                (0, new_size, new_size),
                (new_size, 0, 0),
                (new_size, 0, new_size),
                (new_size, new_size, 0),
                (new_size, new_size, new_size),
            ] {
                heap.push(Space::from(
                    &space,
                    space.x + dx,
                    space.y + dy,
                    space.z + dz,
                    new_size,
                    &nanobots,
                ));
            }
        }
        unreachable!()
    }

    pub fn solve(input: &str) -> isize {
        let nanobots = parse(input);
        let result = find_best(nanobots);
        result.distance
    }
}

pub fn main(test: bool) {
    let test_input = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"
        .to_owned();
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
