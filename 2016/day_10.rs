//! https://adventofcode.com/2016/day/10
//! https://adventofcode.com/2016/day/10/input

use std::{fs::read_to_string, time::Instant};

#[derive(Copy, Clone, Debug)]
enum Destination {
    Bot(usize),
    Output(usize),
}

impl Default for Destination {
    fn default() -> Self {
        Destination::Output(0)
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Bot {
    low: Destination,
    high: Destination,
}

fn parse(input: &str) -> (Vec<Option<usize>>, Vec<Bot>, usize) {
    let values: Vec<_> = input
        .lines()
        .filter(|line| line.starts_with('v'))
        .map(|line| {
            let mut parts = line.split_whitespace();
            let value = parts.nth(1).unwrap().parse::<usize>().unwrap();
            let bot = parts.last().unwrap().parse().unwrap();
            (value, bot)
        })
        .collect();
    let bots: Vec<_> = input
        .lines()
        .filter(|line| line.starts_with('b'))
        .map(|line| {
            let mut parts = line.split_whitespace();
            let bot = parts.nth(1).unwrap().parse::<usize>().unwrap();
            let low = if parts.nth(3).unwrap().starts_with('o') {
                Destination::Output(parts.next().unwrap().parse().unwrap())
            } else {
                Destination::Bot(parts.next().unwrap().parse().unwrap())
            };
            let high = if parts.nth(3).unwrap().starts_with('o') {
                Destination::Output(parts.next().unwrap().parse().unwrap())
            } else {
                Destination::Bot(parts.next().unwrap().parse().unwrap())
            };
            (bot, low, high)
        })
        .collect();
    let mut result_values = vec![None; values.iter().max_by_key(|(v, _)| v).unwrap().0 + 1];
    for (v, b) in values {
        result_values[v] = Some(b);
    }
    let mut result_bots =
        vec![Bot::default(); bots.iter().max_by_key(|(b, _, _)| b).unwrap().0 + 1];
    for (b, low, high) in bots {
        result_bots[b] = Bot { low, high };
    }
    let outputs = result_bots
        .iter()
        .filter_map(|&bot| match bot {
            Bot {
                low: Destination::Output(low),
                high: Destination::Output(high),
            } => Some(low.max(high)),
            Bot {
                low: Destination::Output(o),
                ..
            }
            | Bot {
                high: Destination::Output(o),
                ..
            } => Some(o),
            _ => None,
        })
        .max()
        .unwrap();
    (result_values, result_bots, outputs)
}

pub mod part1 {
    use std::collections::VecDeque;

    use super::{parse, Destination};

    pub fn solve(input: &str) -> usize {
        let (values, bots, _) = parse(input);
        let mut states = vec![vec![]; bots.len()];
        let mut queue = VecDeque::new();
        for (value, bot) in values
            .iter()
            .enumerate()
            .filter_map(|(value, &bot)| bot.is_some().then(|| (value, bot.unwrap())))
        {
            states[bot].push(value);
            if states[bot].len() == 2 {
                queue.push_back(bot);
            }
        }
        while let Some(bot) = queue.pop_front() {
            if states[bot].contains(&61) && states[bot].contains(&17) {
                return bot;
            }
            states[bot].sort();
            let (low, high) = (states[bot][0], states[bot][1]);
            if let Destination::Bot(destination) = bots[bot].low {
                states[destination].push(low);
                if states[destination].len() == 2 {
                    queue.push_back(destination);
                }
            }
            if let Destination::Bot(destination) = bots[bot].high {
                states[destination].push(high);
                if states[destination].len() == 2 {
                    queue.push_back(destination);
                }
            }
        }
        unreachable!()
    }
}

pub mod part2 {
    use std::collections::VecDeque;

    use super::{parse, Destination};

    pub fn solve(input: &str) -> usize {
        let (values, bots, outputs) = parse(input);
        let mut states = vec![vec![]; bots.len()];
        let mut outputs = vec![0; outputs + 1];
        let mut queue = VecDeque::new();
        for (value, bot) in values
            .iter()
            .enumerate()
            .filter_map(|(value, &bot)| bot.is_some().then(|| (value, bot.unwrap())))
        {
            states[bot].push(value);
            if states[bot].len() == 2 {
                queue.push_back(bot);
            }
        }
        while let Some(bot) = queue.pop_front() {
            if outputs[0] * outputs[1] * outputs[2] != 0 {
                return outputs[0] * outputs[1] * outputs[2];
            }
            states[bot].sort();
            let (low, high) = (states[bot][0], states[bot][1]);
            match bots[bot].low {
                Destination::Bot(destination) => {
                    states[destination].push(low);
                    if states[destination].len() == 2 {
                        queue.push_back(destination);
                    }
                }
                Destination::Output(output) => outputs[output] = low,
            }
            match bots[bot].high {
                Destination::Bot(destination) => {
                    states[destination].push(high);
                    if states[destination].len() == 2 {
                        queue.push_back(destination);
                    }
                }
                Destination::Output(output) => outputs[output] = high,
            }
        }
        unreachable!()
    }
}

pub fn main(test: bool) {
    let test_input = "value 5 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 3 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2"
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
