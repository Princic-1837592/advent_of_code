//! https://adventofcode.com/2019/day/6
//! https://adventofcode.com/2019/day/6/input

use std::{
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    fs::read_to_string,
    time::Instant,
};

type Orbits<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse(input: &str) -> Orbits {
    let mut direct = Orbits::new();
    for (origin, target) in input.lines().map(|line| {
        let mut parts = line.split(')');
        (parts.next().unwrap(), parts.next().unwrap())
    }) {
        match direct.entry(origin) {
            Entry::Occupied(mut entry) => {
                entry.get_mut().push(target);
            }
            Entry::Vacant(entry) => {
                entry.insert(vec![target]);
            }
        }
        if let Entry::Vacant(entry) = direct.entry(target) {
            entry.insert(vec![]);
        }
    }
    direct
}

fn bfs(orbits: Orbits, from: &str, to: &str) -> usize {
    let mut queue = VecDeque::from([(0, from)]);
    let mut visited = HashSet::new();
    let mut total_distance = 0;
    while let Some((distance, object)) = queue.pop_back() {
        if object == to {
            return distance;
        }
        visited.insert(object);
        total_distance += distance;
        let new_distance = distance + 1;
        for new_object in orbits.get(object).unwrap() {
            if !visited.contains(new_object) {
                queue.push_back((new_distance, new_object));
            }
        }
    }
    total_distance
}

pub mod part1 {
    use super::{bfs, parse};

    pub fn solve(input: &str) -> usize {
        let orbits = parse(input);
        bfs(orbits, "COM", "")
    }
}

pub mod part2 {
    use super::{bfs, parse, Orbits};

    fn reverse(orbits: Orbits) -> Orbits {
        let mut reversed = orbits.clone();
        for (src, dests) in orbits {
            for dest in dests {
                reversed.get_mut(dest).unwrap().push(src);
            }
        }
        reversed
    }

    pub fn solve(input: &str) -> usize {
        let mut orbits = parse(input);
        orbits = reverse(orbits);
        bfs(orbits, "YOU", "SAN") - 2
    }
}

pub fn main(test: bool) {
    let test_input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2019/day_06_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
