//! https://adventofcode.com/2015/day/22
//! https://adventofcode.com/2015/day/22/input

use std::{
    fs::read_to_string,
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug)]
struct Spell {
    cost: usize,
    duration: usize,
}

impl Spell {
    fn start(&self, wizard: &mut Character, boss: &mut Character) {
        match self.cost {
            53 => boss.hp -= 4,
            73 => {
                boss.hp -= 2;
                wizard.hp += 2;
            }
            113 => wizard.armor += 7,
            _ => {}
        }
    }

    fn effect(&self, wizard: &mut Character, boss: &mut Character) {
        match self.cost {
            173 => boss.hp -= 3,
            229 => wizard.mana += 101,
            _ => {}
        }
    }

    fn end(&self, wizard: &mut Character, _boss: &mut Character) {
        if self.cost == 113 {
            wizard.armor -= 7
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Character {
    hp: isize,
    damage: usize,
    armor: usize,
    mana: usize,
}

const SPELLS: [Spell; 5] = [
    Spell {
        cost: 53,
        duration: 0,
    },
    Spell {
        cost: 73,
        duration: 0,
    },
    Spell {
        cost: 113,
        duration: 6,
    },
    Spell {
        cost: 173,
        duration: 6,
    },
    Spell {
        cost: 229,
        duration: 5,
    },
];

type Parsed = (isize, usize);

fn parse(input: &str) -> Parsed {
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
    (boss_hp, boss_damage)
}

fn fight(
    mut player: Character,
    mut boss: Character,
    is_player_turn: bool,
    mut timers: [usize; 5],
    spent: usize,
    mut min_spent: usize,
    part2: bool,
) -> usize {
    if spent >= min_spent {
        return usize::MAX;
    }
    if part2 && is_player_turn {
        player.hp -= 1;
    }
    if player.hp <= 0 {
        return usize::MAX;
    }
    for (i, timer) in timers.iter_mut().enumerate() {
        match *timer {
            1 => {
                *timer = 0;
                SPELLS[i].effect(&mut player, &mut boss);
                SPELLS[i].end(&mut player, &mut boss);
            }
            0 => {}
            _ => {
                *timer -= 1;
                SPELLS[i].effect(&mut player, &mut boss);
            }
        }
    }
    if boss.hp <= 0 {
        return spent;
    }
    if is_player_turn {
        let mut has_cast = false;
        for (i, spell) in SPELLS.iter().enumerate() {
            if player.mana >= spell.cost && timers[i] == 0 {
                let (mut player, mut boss) = (player, boss);
                player.mana -= spell.cost;
                spell.start(&mut player, &mut boss);
                has_cast = true;
                timers[i] = spell.duration;
                let result = fight(
                    player,
                    boss,
                    false,
                    timers,
                    spent + spell.cost,
                    min_spent,
                    part2,
                );
                if result < min_spent {
                    min_spent = result;
                }
                timers[i] = 0;
            }
        }
        if has_cast {
            min_spent
        } else {
            usize::MAX
        }
    } else {
        player.hp -= (boss.damage - player.armor).max(1) as isize;
        fight(player, boss, true, timers, spent, min_spent, part2)
    }
}

pub mod part1 {
    use super::{fight, parse, Character};

    pub fn solve(input: &str, hp: isize, mana: usize) -> usize {
        let boss = parse(input);
        fight(
            Character {
                hp,
                mana,
                ..Default::default()
            },
            Character {
                hp: boss.0,
                damage: boss.1,
                ..Default::default()
            },
            true,
            [0; 5],
            0,
            usize::MAX,
            false,
        )
    }
}

pub mod part2 {
    use super::{fight, parse, Character};

    pub fn solve(input: &str, hp: isize, mana: usize) -> usize {
        let boss = parse(input);
        fight(
            Character {
                hp,
                mana,
                ..Default::default()
            },
            Character {
                hp: boss.0,
                damage: boss.1,
                ..Default::default()
            },
            true,
            [0; 5],
            0,
            usize::MAX,
            true,
        )
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "Hit Points: 14
Damage: 8"
        .to_owned();
    let (puzzle_input, hp, mana) = if test {
        (test_input, 10, 250)
    } else {
        (read_to_string("../inputs/2015/day_22_input.txt").unwrap(), 50, 500)
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let result = part1::solve(&puzzle_input, hp, mana);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(&puzzle_input, hp, mana);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    println!("Total {:?}", total);
    total
}
