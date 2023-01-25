//! https://adventofcode.com/2015/day/21
//! https://adventofcode.com/2015/day/21/input

use std::{fs::read_to_string, time::Instant};

const WEAPONS: [(usize, isize, isize); 5] =
    [(8, 4, 0), (10, 5, 0), (25, 6, 0), (40, 7, 0), (74, 8, 0)];
const ARMOR: [(usize, isize, isize); 5] =
    [(13, 0, 1), (31, 0, 2), (53, 0, 3), (75, 0, 4), (102, 0, 5)];
const RINGS: [(usize, isize, isize); 6] = [
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

fn parse(input: &str) -> (isize, isize, isize) {
    let mut lines = input.lines();
    let boss_hp = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let boss_damage = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let boss_armor = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    (boss_hp, boss_damage, boss_armor)
}

pub mod part1 {
    use itertools::Itertools;

    use crate::day_21::{parse, ARMOR, RINGS, WEAPONS};

    pub fn solve(input: &str) -> usize {
        let boss = parse(input);
        (0..WEAPONS.len())
            .cartesian_product((0..1).map(|_| 0..=ARMOR.len()).multi_cartesian_product())
            .cartesian_product((0..2).map(|_| 0..=RINGS.len()).multi_cartesian_product())
            .map(|((weapon, armor), rings)| {
                let weapon = WEAPONS[weapon];
                let armor = if armor[0] != 0 {
                    Some(ARMOR[armor[0] - 1])
                } else {
                    None
                };
                let rings: Vec<_> = rings
                    .iter()
                    .filter(|&&take| take > 0)
                    .map(|&ring| RINGS[ring - 1])
                    .collect();
                (weapon, armor, rings)
            })
            .map(|(weapon, armor, rings)| {
                let armor = armor.unwrap_or((0, 0, 0));
                (
                    weapon.0 + armor.0 + rings.iter().map(|ring| ring.0).sum::<usize>(),
                    weapon.1 + armor.1 + rings.iter().map(|ring| ring.1).sum::<isize>(),
                    weapon.2 + armor.2 + rings.iter().map(|ring| ring.2).sum::<isize>(),
                )
            })
            .filter(|(_, dmg, armor)| {
                let player_turns = (boss.0 as f32 / (dmg - boss.2).max(1) as f32).ceil() as usize;
                let boss_turns = (100_f32 / (boss.1 - armor).max(1) as f32).ceil() as usize;
                player_turns <= boss_turns
            })
            .min_by_key(|&(cost, ..)| cost)
            .unwrap()
            .0
    }
}

pub mod part2 {
    use itertools::Itertools;

    use crate::day_21::{parse, ARMOR, RINGS, WEAPONS};

    pub fn solve(input: &str) -> usize {
        let boss = parse(input);
        (0..WEAPONS.len())
            .cartesian_product((0..1).map(|_| 0..=ARMOR.len()).multi_cartesian_product())
            .cartesian_product((0..2).map(|_| 0..=RINGS.len()).multi_cartesian_product())
            .map(|((weapon, armor), rings)| {
                let weapon = WEAPONS[weapon];
                let armor = if armor[0] != 0 {
                    Some(ARMOR[armor[0] - 1])
                } else {
                    None
                };
                let rings: Vec<_> = rings
                    .iter()
                    .filter(|&&take| take > 0)
                    .map(|&ring| RINGS[ring - 1])
                    .collect();
                (weapon, armor, rings)
            })
            .map(|(weapon, armor, rings)| {
                let armor = armor.unwrap_or((0, 0, 0));
                (
                    weapon.0 + armor.0 + rings.iter().map(|ring| ring.0).sum::<usize>(),
                    weapon.1 + armor.1 + rings.iter().map(|ring| ring.1).sum::<isize>(),
                    weapon.2 + armor.2 + rings.iter().map(|ring| ring.2).sum::<isize>(),
                )
            })
            .filter(|(_, dmg, armor)| {
                let player_turns = (boss.0 as f32 / (dmg - boss.2).max(1) as f32).ceil() as usize;
                let boss_turns = (100_f32 / (boss.1 - armor).max(1) as f32).ceil() as usize;
                player_turns > boss_turns
            })
            .max_by_key(|&(cost, ..)| cost)
            .unwrap()
            .0
    }
}

pub fn main(test: bool) {
    let test_input = "".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_21_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
