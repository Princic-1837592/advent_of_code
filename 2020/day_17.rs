use std::{fs::read_to_string, time::Instant};

pub mod part1 {
    use std::collections::{HashMap, HashSet};

    fn step(
        map: &mut HashSet<(isize, isize, isize)>,
        support: &mut HashMap<(isize, isize, isize), usize>,
    ) {
        support.clear();
        map.iter().for_each(|p| {
            (-1..=1)
                .flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (x, y, z))))
                .filter(|&d| d != (0, 0, 0))
                .for_each(|n| {
                    *support
                        .entry((p.0 + n.0, p.1 + n.1, p.2 + n.2))
                        .or_insert(0) += 1;
                });
        });
        map.retain(|p| {
            let s = support.get(p).unwrap_or(&0);
            *s == 2 || *s == 3
        });
        support
            .iter()
            .filter(|(_, &s)| s == 3)
            .map(|(&p, _)| p)
            .for_each(|p| {
                map.insert(p);
            });
    }

    pub fn solve(input: &str) -> usize {
        let mut map = input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(y, c)| ((x as isize, y as isize, 0), c == '#'))
            })
            .filter(|(_, b)| *b)
            .map(|(a, _)| a)
            .collect();
        let mut support = HashMap::new();
        for _ in 0..6 {
            step(&mut map, &mut support);
        }
        map.len()
    }
}

pub mod part2 {
    use std::collections::{HashMap, HashSet};

    fn step(
        map: &mut HashSet<(isize, isize, isize, isize)>,
        support: &mut HashMap<(isize, isize, isize, isize), usize>,
    ) {
        support.clear();
        map.iter().for_each(|p| {
            (-1..=1)
                .flat_map(|x| {
                    (-1..=1).flat_map(move |y| {
                        (-1..=1).flat_map(move |z| (-1..=1).map(move |w| (x, y, z, w)))
                    })
                })
                .filter(|&d| d != (0, 0, 0, 0))
                .for_each(|n| {
                    *support
                        .entry((p.0 + n.0, p.1 + n.1, p.2 + n.2, p.3 + n.3))
                        .or_insert(0) += 1;
                });
        });
        map.retain(|p| {
            let s = support.get(p).unwrap_or(&0);
            *s == 2 || *s == 3
        });
        support
            .iter()
            .filter(|(_, &s)| s == 3)
            .map(|(&p, _)| p)
            .for_each(|p| {
                map.insert(p);
            });
    }

    pub fn solve(input: &str) -> usize {
        let mut map = input
            .lines()
            .enumerate()
            .flat_map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(y, c)| ((x as isize, y as isize, 0, 0), c == '#'))
            })
            .filter(|(_, b)| *b)
            .map(|(a, _)| a)
            .collect();
        let mut support = HashMap::new();
        for _ in 0..6 {
            step(&mut map, &mut support);
        }
        map.len()
    }
}

pub fn main(test: bool) {
    let test_input = ".#.
..#
###"
    .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2020/day_17_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
