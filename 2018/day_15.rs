//! https://adventofcode.com/2018/day/15
//! https://adventofcode.com/2018/day/15/input

use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap, HashSet, VecDeque},
    fs::read_to_string,
    rc::Rc,
    time::Instant,
};

use itertools::Itertools;

const ADJACENT: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

#[derive(Copy, Clone, Debug)]
struct Unit {
    elf: bool,
    hp: isize,
    position: (usize, usize),
    damage: isize,
}

impl Eq for Unit {}

impl PartialEq<Self> for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.position.eq(&other.position)
    }
}

impl PartialOrd<Self> for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}

#[derive(Copy, Clone, Debug)]
enum TurnResult {
    NoTarget,
    GoblinDied,
    ElfDied,
    NothingSpecial,
}

#[derive(Copy, Clone, Debug)]
enum RoundResult {
    NoWinner,
    ElvesWin,
    GoblinsWin,
}

fn parse(input: &str) -> (Vec<Vec<bool>>, Vec<Unit>) {
    let mut walls =
        vec![vec![false; input.lines().next().unwrap().chars().count()]; input.lines().count()];
    let mut units = Vec::new();
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            walls[i][j] = char == '#';
            if char == 'G' {
                units.push(Unit {
                    elf: false,
                    hp: 200,
                    position: (i, j),
                    damage: 3,
                })
            } else if char == 'E' {
                units.push(Unit {
                    elf: true,
                    hp: 200,
                    position: (i, j),
                    damage: 3,
                })
            }
        }
    }
    (walls, units)
}

fn next_move(
    walls: &[Vec<bool>],
    position: (usize, usize),
    candidates: HashSet<(usize, usize)>,
    units: &[Rc<RefCell<Unit>>],
) -> (usize, usize) {
    let mut queue = VecDeque::from([(position, 0)]);
    let mut paths = HashMap::from([(position, (0, None))]);
    let mut seen = HashSet::new();
    let occupied: HashSet<_> = units
        .iter()
        .filter_map(|unit| (unit.borrow().hp > 0).then_some(unit.borrow().position))
        .collect();
    while let Some((node @ (i, j), distance)) = queue.pop_front() {
        for next @ (ni, nj) in ADJACENT
            .iter()
            .map(|(di, dj)| ((i as isize + di) as usize, (j as isize + dj) as usize))
        {
            if walls[ni][nj] || occupied.contains(&next) {
                continue;
            }
            let path = (distance + 1, Some(node));
            match paths.entry(next) {
                Entry::Vacant(entry) => {
                    entry.insert(path);
                }
                Entry::Occupied(mut entry) if *entry.get() > path => {
                    entry.insert(path);
                }
                _ => {}
            }
            if seen.contains(&next) {
                continue;
            }
            if !queue.iter().any(|(position, _)| next == *position) {
                queue.push_back((next, distance + 1));
            }
        }
        seen.insert(node);
    }
    let maybe_min = paths
        .iter()
        .filter_map(|(position, (distance, _))| {
            candidates
                .contains(position)
                .then_some((distance, position))
        })
        .min();
    if maybe_min.is_none() {
        return position;
    }
    let mut closest = *maybe_min.unwrap().1;
    while paths.get(&closest).unwrap().0 > 1 {
        closest = paths.get(&closest).unwrap().1.unwrap();
    }
    closest
}

fn turn(
    walls: &mut [Vec<bool>],
    this: Rc<RefCell<Unit>>,
    units: &[Rc<RefCell<Unit>>],
) -> TurnResult {
    let targets: Vec<_> = units
        .iter()
        .filter(|target| target.borrow().elf != this.borrow().elf && target.borrow().hp > 0)
        .collect();
    if targets.is_empty() {
        return TurnResult::NoTarget;
    }
    let occupied: HashSet<_> = units
        .iter()
        .filter_map(|unit| {
            (unit.borrow().hp > 0 && *unit != this).then_some(unit.borrow().position)
        })
        .collect();
    let in_range: HashSet<_> = ADJACENT
        .iter()
        .cartesian_product(targets.iter().map(|target| target.borrow().position))
        .filter_map(|((di, dj), (ti, tj))| {
            let adjacent @ (ai, aj) = ((ti as isize + di) as usize, (tj as isize + dj) as usize);
            (!walls[ai][aj] && !occupied.contains(&adjacent)).then_some(adjacent)
        })
        .collect();
    if !in_range.contains(&this.borrow().position) {
        let next_move = next_move(walls, this.borrow().position, in_range, units);
        this.borrow_mut().position = next_move;
    }
    let adjacent: HashSet<_> = ADJACENT
        .iter()
        .map(|(di, dj)| {
            (
                (this.borrow().position.0 as isize + di) as usize,
                (this.borrow().position.1 as isize + dj) as usize,
            )
        })
        .collect();
    let opponents: Vec<_> = targets
        .iter()
        .filter(|unit| adjacent.contains(&unit.borrow().position))
        .collect();
    if !opponents.is_empty() {
        let target = opponents
            .iter()
            .min_by_key(|opponent| (opponent.borrow().hp, opponent.borrow().position))
            .unwrap();
        target.borrow_mut().hp -= this.borrow().damage;
        if target.borrow().hp <= 0 {
            return if target.borrow().elf {
                TurnResult::ElfDied
            } else {
                TurnResult::GoblinDied
            };
        }
    }
    TurnResult::NothingSpecial
}

pub mod part1 {
    use std::{cell::RefCell, rc::Rc};

    use crate::day_15::{parse, turn, RoundResult, TurnResult, Unit};

    fn round(
        walls: &mut [Vec<bool>],
        #[allow(clippy::ptr_arg)] units: &mut Vec<Rc<RefCell<Unit>>>,
    ) -> RoundResult {
        units.sort();
        for unit in units.clone() {
            #[allow(clippy::collapsible_if)]
            if unit.borrow().hp > 0 {
                if matches!(turn(walls, unit.clone(), units), TurnResult::NoTarget) {
                    return if unit.borrow().elf {
                        RoundResult::ElvesWin
                    } else {
                        RoundResult::GoblinsWin
                    };
                }
            }
        }
        RoundResult::NoWinner
    }

    pub fn solve(input: &str) -> usize {
        let (mut walls, units) = parse(input);
        let mut units: Vec<_> = units
            .into_iter()
            .map(|unit| Rc::new(RefCell::new(unit)))
            .collect();
        let mut rounds = 0;
        while let RoundResult::NoWinner = round(&mut walls, &mut units) {
            rounds += 1;
        }
        let hp = units
            .iter()
            .filter_map(|unit| (unit.borrow().hp > 0).then_some(unit.borrow().hp as usize))
            .sum::<usize>();
        rounds * hp
    }
}

pub mod part2 {
    use std::{cell::RefCell, rc::Rc};

    use crate::day_15::{parse, turn, RoundResult, TurnResult, Unit};

    fn round(
        walls: &mut [Vec<bool>],
        #[allow(clippy::ptr_arg)] units: &mut Vec<Rc<RefCell<Unit>>>,
    ) -> RoundResult {
        units.sort();
        for unit in units.clone() {
            if unit.borrow().hp > 0 {
                match turn(walls, unit.clone(), units) {
                    TurnResult::NoTarget => {
                        return if unit.borrow().elf {
                            RoundResult::ElvesWin
                        } else {
                            RoundResult::GoblinsWin
                        }
                    }
                    TurnResult::ElfDied => return RoundResult::GoblinsWin,
                    _ => {}
                }
            }
        }
        RoundResult::NoWinner
    }

    pub fn solve(input: &str) -> usize {
        let (mut walls, units) = parse(input);
        let mut damage = 4;
        loop {
            let mut units: Vec<_> = units
                .iter()
                .map(|unit| {
                    Rc::new(RefCell::new(Unit {
                        hp: 200,
                        damage: if unit.elf { damage } else { 3 },
                        ..*unit
                    }))
                })
                .collect();
            let mut rounds = 0;
            loop {
                let result = round(&mut walls, &mut units);
                match result {
                    RoundResult::NoWinner => {
                        rounds += 1;
                    }
                    RoundResult::ElvesWin => {
                        return rounds
                            * units
                                .iter()
                                .filter_map(|unit| {
                                    (unit.borrow().hp > 0).then_some(unit.borrow().hp as usize)
                                })
                                .sum::<usize>()
                    }
                    RoundResult::GoblinsWin => break,
                }
            }
            damage += 1;
        }
    }
}

pub fn main(test: bool) {
    //     let test_input = "#######
    // #.G...#
    // #...EG#
    // #.#.#G#
    // #..G#E#
    // #.....#
    // #######"
    //         .to_owned();
    //     let test_input = "#######
    // #G..#E#
    // #E#E.E#
    // #G.##.#
    // #...#E#
    // #...E.#
    // #######"
    //         .to_owned();
    //     let test_input = "#######
    // #E..EG#
    // #.#G.E#
    // #E.##E#
    // #G..#.#
    // #..E#.#
    // #######"
    //         .to_owned();
    //     let test_input = "#######
    // #E.G#.#
    // #.#G..#
    // #G.#.G#
    // #G..#.#
    // #...E.#
    // #######"
    //         .to_owned();
    //     let test_input = "#######
    // #.E...#
    // #.#..G#
    // #.###.#
    // #E#G#G#
    // #...#G#
    // #######"
    //         .to_owned();
    let test_input = "#########
    #G......#
    #.E.#...#
    #..##..G#
    #...##..#
    #...#...#
    #.G...G.#
    #.....G.#
    #########"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_15_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
