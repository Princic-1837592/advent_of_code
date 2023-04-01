//! https://adventofcode.com/2019/day/10
//! https://adventofcode.com/2019/day/10/input

use std::{collections::HashSet, fs::read_to_string, time::Instant};

type Coord = (isize, isize);

fn parse(input: &str) -> (HashSet<Coord>, isize, isize) {
    (
        input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, char)| char == '#')
                    .map(move |(j, _)| (i as isize, j as isize))
            })
            .collect(),
        input.lines().count() as isize,
        input.lines().next().unwrap().chars().count() as isize,
    )
}

fn gcd(a: isize, b: isize) -> isize {
    if a == 0 {
        b
    } else {
        gcd(b % a, a)
    }
}

fn reduce(di: isize, dj: isize) -> (isize, isize) {
    match (di, dj) {
        (0, _) => (0, 1),
        (_, 0) => (1, 0),
        (di, dj) => {
            let gcd = gcd(di, dj);
            (di / gcd, dj / gcd)
        }
    }
}

fn find_best(asteroids: &HashSet<Coord>, h: isize, w: isize) -> (Coord, usize) {
    let mut max_detected = ((-1, -1), usize::MIN);
    for &asteroid @ (ai, aj) in asteroids {
        let mut blocked = HashSet::from([asteroid]);
        for other @ (mut oi, mut oj) in asteroids {
            if blocked.contains(other) {
                continue;
            }
            let (mut di, mut dj) = (oi - ai, oj - aj);
            let (si, sj) = (di.signum(), dj.signum());
            (di, dj) = reduce(di.abs(), dj.abs());
            di *= si;
            dj *= sj;
            oi += di;
            oj += dj;
            while oi <= h && oj <= w && oi >= 0 && oj >= 0 {
                if asteroids.contains(&(oi, oj)) {
                    blocked.insert((oi, oj));
                }
                oi += di;
                oj += dj;
            }
        }
        let detected = asteroids.len() - blocked.len();
        if detected > max_detected.1 {
            max_detected = (asteroid, detected);
        }
    }
    max_detected
}

pub mod part1 {
    use crate::day_10::{find_best, parse};

    pub fn solve(input: &str) -> usize {
        let (asteroids, h, w) = parse(input);
        find_best(&asteroids, h, w).1
    }
}

pub mod part2 {
    use crate::day_10::{find_best, parse};

    pub fn solve(input: &str) -> usize {
        let (mut asteroids, h, w) = parse(input);
        let (bi, bj) = find_best(&asteroids, h, w).0;
        asteroids.remove(&(bi, bj));
        let mut asteroids: Vec<_> = asteroids
            .iter()
            .map(|&(ai, aj)| {
                (
                    (ai, aj),
                    ((aj - bj) as f64).atan2((ai - bi) as f64),
                    bi.abs_diff(ai) + bj.abs_diff(aj),
                )
            })
            .collect();
        asteroids.sort_by(|(_, l_angle, l_dist), (_, r_angle, r_dist)| {
            l_angle
                .partial_cmp(r_angle)
                .unwrap()
                .reverse()
                .then(l_dist.cmp(r_dist))
        });
        let mut is_destroyed = vec![false; asteroids.len()];
        let mut last_angle = f64::MIN;
        let mut destroyed = 0;
        let mut i = 0;
        loop {
            if !is_destroyed[i] && asteroids[i].1 != last_angle {
                destroyed += 1;
                if destroyed == 200 {
                    return (asteroids[i].0 .1 * 100 + asteroids[i].0 .0) as usize;
                }
                is_destroyed[i] = true;
                last_angle = asteroids[i].1;
            }
            i += 1;
            if i >= asteroids.len() {
                i = 0;
            }
        }
    }
}

pub fn main(test: bool) {
    let test_input = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_10_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
