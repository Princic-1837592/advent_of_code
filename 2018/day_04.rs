//! https://adventofcode.com/2018/day/4
//! https://adventofcode.com/2018/day/4/input

use std::{fs::read_to_string, time::Instant};

use regex::Regex;

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum Event {
    Begins(usize),
    FallsAsleep,
    WakesUp,
}

impl From<&str> for Event {
    fn from(string: &str) -> Self {
        let mut parts = string.split(' ');
        match parts.next().unwrap() {
            "Guard" => Self::Begins(parts.next().unwrap()[1..].parse().unwrap()),
            "falls" => Self::FallsAsleep,
            "wakes" => Self::WakesUp,
            _ => panic!("Invalid event: {}", string),
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
struct Record {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
    event: Event,
}

impl From<&str> for Record {
    fn from(string: &str) -> Self {
        let pattern = Regex::new(
            r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)] (Guard #\d+ begins shift|falls asleep|wakes up)",
        )
        .unwrap();
        let captures = pattern.captures(string).unwrap();
        Self {
            year: captures.get(1).unwrap().as_str().parse().unwrap(),
            month: captures.get(2).unwrap().as_str().parse().unwrap(),
            day: captures.get(3).unwrap().as_str().parse().unwrap(),
            hour: captures.get(4).unwrap().as_str().parse().unwrap(),
            minute: captures.get(5).unwrap().as_str().parse().unwrap(),
            event: Event::from(captures.get(6).unwrap().as_str()),
        }
    }
}

fn parse(input: &str) -> Vec<Record> {
    let mut result: Vec<_> = input.lines().map(Record::from).collect();
    result.sort();
    result
}

fn sum_records(records: Vec<Record>) -> Vec<[usize; 60]> {
    let mut states = vec![
        [0; 60];
        records
            .iter()
            .filter_map(|record| if let Event::Begins(guard) = record.event {
                Some(guard)
            } else {
                None
            })
            .max()
            .unwrap()
            + 1
    ];
    let mut active_guard = None;
    let mut fell_asleep = 0;
    for record in records {
        match record.event {
            Event::Begins(guard) => active_guard = Some(guard),
            Event::FallsAsleep => fell_asleep = record.minute,
            Event::WakesUp => {
                for minute in fell_asleep..record.minute {
                    states[active_guard.unwrap()][minute] += 1;
                }
            }
        }
    }
    states
}

pub mod part1 {
    use crate::day_04::{parse, sum_records};

    pub fn solve(input: &str) -> usize {
        let records = parse(input);
        let states = sum_records(records);
        let (id, most_sleepy_guard) = states
            .iter()
            .enumerate()
            .max_by_key(|(_, minutes)| minutes.iter().filter(|minute| **minute != 0).count())
            .unwrap();
        most_sleepy_guard
            .iter()
            .enumerate()
            .max_by_key(|(_, times)| *times)
            .unwrap()
            .0
            * id
    }
}

pub mod part2 {
    use crate::day_04::{parse, sum_records};

    pub fn solve(input: &str) -> usize {
        let records = parse(input);
        let states = sum_records(records);
        let (id, minute) = states
            .iter()
            .enumerate()
            .map(|(i, minutes)| {
                (
                    i,
                    minutes
                        .iter()
                        .enumerate()
                        .max_by_key(|(_, times)| *times)
                        .unwrap(),
                )
            })
            .max_by_key(|(_, (_, times))| *times)
            .map(|(i, (minute, _))| (i, minute))
            .unwrap();
        id * minute
    }
}

pub fn main(test: bool) {
    let test_input = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 00:25] wakes up
[1518-11-03 00:24] falls asleep
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-02 00:50] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:29] wakes up
[1518-11-04 00:46] wakes up
[1518-11-05 00:55] wakes up
[1518-11-04 00:36] falls asleep
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_04_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
