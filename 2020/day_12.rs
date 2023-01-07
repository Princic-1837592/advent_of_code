#[derive(Debug, Copy, Clone)]
enum Instruction {
    Move(isize, isize, isize),
    Forward(isize),
    Right,
    Left,
    Flip,
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let n = string.chars().skip(1).collect::<String>().parse().unwrap();
        match string.chars().next().unwrap() {
            'N' => Instruction::Move(-1, 0, n),
            'E' => Instruction::Move(0, 1, n),
            'S' => Instruction::Move(1, 0, n),
            'W' => Instruction::Move(0, -1, n),
            'R' => match n {
                90 => Instruction::Right,
                180 => Instruction::Flip,
                _ => Instruction::Left,
            },
            'L' => match n {
                90 => Instruction::Left,
                180 => Instruction::Flip,
                _ => Instruction::Right,
            },
            _ => Instruction::Forward(n),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input.lines().map(Instruction::from).collect()
}

mod part1 {
    use crate::{parse, Instruction};

    pub(crate) fn solve(input: &str) -> usize {
        let instructions = parse(input);
        let mut position = (0, 0);
        let mut direction = (0, 1);
        for instr in instructions {
            match instr {
                Instruction::Move(x, y, n) => position = (position.0 + x * n, position.1 + y * n),
                Instruction::Forward(n) => {
                    position = (position.0 + direction.0 * n, position.1 + direction.1 * n)
                }
                Instruction::Right => direction = (direction.1, -direction.0),
                Instruction::Left => direction = (-direction.1, direction.0),
                Instruction::Flip => direction = (-direction.0, -direction.1),
            }
        }
        (position.0.abs() + position.1.abs()) as usize
    }
}

mod part2 {
    use crate::{parse, Instruction};

    pub(crate) fn solve(input: &str) -> usize {
        let instructions = parse(input);
        let mut ship = (0, 0);
        let mut waypoint = (-1, 10);
        for instr in instructions {
            match instr {
                Instruction::Move(x, y, n) => waypoint = (waypoint.0 + x * n, waypoint.1 + y * n),
                Instruction::Forward(n) => {
                    ship = (ship.0 + waypoint.0 * n, ship.1 + waypoint.1 * n)
                }
                Instruction::Right => waypoint = (waypoint.1, -waypoint.0),
                Instruction::Left => waypoint = (-waypoint.1, waypoint.0),
                Instruction::Flip => waypoint = (-waypoint.0, -waypoint.1),
            }
        }
        (ship.0.abs() + ship.1.abs()) as usize
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "F10
N3
F7
R90
F11"
    .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_12_input.txt").unwrap()
    };
    println!("{}", part1::solve(&puzzle_input));
    println!("{}", part2::solve(&puzzle_input));
}
