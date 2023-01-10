use std::{str::Chars, time::Instant};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    pub fn apply(self, result: usize, operand: usize) -> usize {
        match self {
            Operation::Add => result + operand,
            Operation::Mul => result * operand,
        }
    }
}

fn evaluate(chars: &mut Chars) -> usize {
    let mut operation = Operation::Add;
    let mut result = 0;
    while let Some(char) = chars.next() {
        match char {
            c @ '0'..='9' => result = operation.apply(result, c.to_digit(10).unwrap() as usize),
            '+' => operation = Operation::Add,
            '*' => operation = Operation::Mul,
            '(' => result = operation.apply(result, evaluate(chars)),
            ')' => return result,
            _space => {}
        }
    }
    result
}

mod part1 {
    use crate::evaluate;

    pub(crate) fn solve(input: &str) -> usize {
        input.lines().map(|line| evaluate(&mut line.chars())).sum()
    }
}

mod part2 {
    use crate::evaluate;

    fn apply_parentheses(expression: &str) -> String {
        let mut result = String::from("((");
        result.extend(expression.chars().map(|c| match c {
            '+' => ") + (".to_string(),
            '*' => ")) * ((".to_string(),
            '(' => "(((".to_string(),
            ')' => ")))".to_string(),
            _ => c.to_string(),
        }));
        result.push_str("))");
        result
    }

    pub(crate) fn solve(input: &str) -> usize {
        input
            .lines()
            .map(|line| evaluate(&mut apply_parentheses(line).chars()))
            .sum()
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "1 + (2 * 3) + (4 * (5 + 6))".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_18_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
}
