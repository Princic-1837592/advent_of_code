//! https://adventofcode.com/2018/day/24
//! https://adventofcode.com/2018/day/24/input

use std::{fs::read_to_string, time::Instant};

use regex::Regex;

use crate::LINE_ENDING;

type Groups = Vec<Group>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Damage {
    Fire,
    Radiation,
    Bludgeoning,
    Slashing,
    Cold,
}

impl From<&str> for Damage {
    fn from(string: &str) -> Self {
        match string.chars().next().unwrap() {
            'f' => Damage::Fire,
            'r' => Damage::Radiation,
            'b' => Damage::Bludgeoning,
            's' => Damage::Slashing,
            'c' => Damage::Cold,
            _ => panic!("Wrong attack type: {}", string),
        }
    }
}

#[derive(Clone, Debug)]
struct Group {
    units: isize,
    hp: isize,
    damage: isize,
    initiative: isize,
    damage_type: Damage,
    immunities: Vec<Damage>,
    weaknesses: Vec<Damage>,
    infection: bool,
}

impl From<(&str, bool)> for Group {
    fn from((string, infection): (&str, bool)) -> Self {
        let group = Regex::new(r"(\d+) units each with (\d+) hit points(?: \((.+?)\))? with an attack that does (\d+) ([a-z]+) damage at initiative (\d+)").unwrap();
        let captures = group.captures(string).unwrap();
        let mut result = Group {
            units: captures.get(1).unwrap().as_str().parse().unwrap(),
            hp: captures.get(2).unwrap().as_str().parse().unwrap(),
            damage: captures.get(4).unwrap().as_str().parse().unwrap(),
            initiative: captures.get(6).unwrap().as_str().parse().unwrap(),
            damage_type: captures.get(5).unwrap().as_str().into(),
            immunities: vec![],
            weaknesses: vec![],
            infection,
        };
        if let Some(modifiers) = captures.get(3) {
            for modifiers in modifiers.as_str().split("; ") {
                let mut parts = modifiers.split(" to ");
                let w_or_i = parts.next().unwrap().chars().next().unwrap();
                let damages = parts.next().unwrap().split(", ").map(Damage::from);
                match w_or_i {
                    'w' => &mut result.weaknesses,
                    'i' => &mut result.immunities,
                    _ => panic!(),
                }
                .extend(damages);
            }
        }
        result
    }
}

fn parse(input: &str) -> Groups {
    let separator = LINE_ENDING.repeat(2);
    let mut armies = input.split(&separator);

    armies
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| (line, false).into())
        .chain(
            armies
                .next()
                .unwrap()
                .lines()
                .skip(1)
                .map(|line| (line, true).into()),
        )
        .collect()
}

fn target_selection(groups: &Groups) -> Vec<Option<usize>> {
    let mut effective_power: Vec<_> = (0..groups.len()).collect();
    effective_power.sort_by_key(|group| {
        (
            groups[*group].units * groups[*group].damage,
            groups[*group].initiative,
        )
    });
    effective_power.reverse();
    let mut targets = vec![None; groups.len()];
    let mut selected = vec![false; groups.len()];
    for i in effective_power {
        let group = &groups[i];
        let mut best_target = None;
        let mut priority = (0, 0, 0);
        for (t, target) in groups
            .iter()
            .enumerate()
            .filter(|(t, target)| target.infection != group.infection && !selected[*t])
        {
            let target = (
                group.damage
                    * group.units
                    * if target.immunities.contains(&group.damage_type) {
                        0
                    } else if target.weaknesses.contains(&group.damage_type) {
                        2
                    } else {
                        1
                    },
                target.damage * target.units,
                target.initiative,
            );
            if target.0 > 0 && target > priority {
                priority = target;
                best_target = Some(t);
            }
        }
        if let Some(best_target) = best_target {
            selected[best_target] = true;
            targets[i] = Some(best_target);
        }
    }
    targets
}

fn attack(groups: &mut Groups, targets: Vec<Option<usize>>) -> bool {
    let mut dealt_damage = true;
    for g in (0..groups.len()).filter(|g| targets[*g].is_some()) {
        let group = groups[g].clone();
        if group.units <= 0 {
            continue;
        }
        let target = &mut groups[targets[g].unwrap()];
        let damage = group.damage
            * group.units
            * if target.immunities.contains(&group.damage_type) {
                0
            } else if target.weaknesses.contains(&group.damage_type) {
                2
            } else {
                1
            };
        if damage / target.hp > 0 {
            dealt_damage = true;
            target.units -= damage / target.hp;
        }
    }
    dealt_damage
}

fn round(groups: &mut Groups) {
    let targets = target_selection(groups);
    if !attack(groups, targets) {
        groups.retain(|group| group.infection);
    } else {
        groups.retain(|group| group.units > 0);
    }
}

fn fight(groups: &mut Groups) {
    while let (true, true) = groups.iter().fold(
        (false, false),
        |(mut immune_system, mut infection), group| {
            if !group.infection {
                immune_system = true
            }
            if group.infection {
                infection = true
            }
            (immune_system, infection)
        },
    ) {
        round(groups);
    }
}

pub mod part1 {
    use super::{fight, parse};

    pub fn solve(input: &str) -> isize {
        let mut groups = parse(input);
        groups.sort_by_key(|group| group.initiative);
        groups.reverse();
        fight(&mut groups);
        groups.iter().map(|group| group.units).sum()
    }
}

pub mod part2 {
    use super::{fight, parse};

    pub fn solve(input: &str) -> isize {
        let mut groups = parse(input);
        groups.sort_by_key(|group| group.initiative);
        groups.reverse();
        let mut boost = 1;
        loop {
            let mut groups = groups.clone();
            groups
                .iter_mut()
                .filter(|group| !group.infection)
                .for_each(|group| group.damage += boost);
            fight(&mut groups);
            if !groups[0].infection {
                return groups.iter().map(|group| group.units).sum();
            }
            boost += 1;
        }
    }
}

pub fn main(test: bool) {
    let test_input = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"
        .to_owned()
        .replace('\n',"\r\n");
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_24_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
