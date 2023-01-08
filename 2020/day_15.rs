use std::collections::HashMap;

fn parse(input: &str) -> HashMap<usize, usize> {
    input
        .split(',')
        .enumerate()
        .map(|(i, n)| (n.parse().unwrap(), i + 1))
        .collect()
}

mod part1 {
    use crate::parse;

    pub(crate) fn solve(input: &str) -> usize {
        let mut turns = parse(input);
        let mut number = 0;
        let mut turn = turns.len() + 1;
        while turn < 2020 {
            let previous = turns.get(&number).unwrap_or(&turn);
            let next_number = turn - previous;
            turns.insert(number, turn);
            turn += 1;
            number = next_number;
        }
        number
    }
}

mod part2 {
    use crate::parse;

    pub(crate) fn solve(input: &str) -> usize {
        let mut turns = parse(input);
        let mut number = 0;
        let mut turn = turns.len() + 1;
        while turn < 30000000 {
            let previous = turns.get(&number).unwrap_or(&turn);
            let next_number = turn - previous;
            turns.insert(number, turn);
            turn += 1;
            number = next_number;
        }
        number
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "0,3,6".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_15_input.txt").unwrap()
    };
    println!("{}", part1::solve(&puzzle_input));
    println!("{}", part2::solve(&puzzle_input));
}
