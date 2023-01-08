use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Copy)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            _f => Seat::Floor,
        }
    }
}

impl Display for Seat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Seat::Floor => write!(f, "."),
            Seat::Empty => write!(f, "L"),
            Seat::Occupied => write!(f, "#"),
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Seat>> {
    input
        .lines()
        .map(|l| l.chars().map(Seat::from).collect())
        .collect()
}

fn count(seats: Vec<Vec<Seat>>) -> usize {
    seats
        .iter()
        .map(|l| l.iter().filter(|s| matches!(s, Seat::Occupied)).count())
        .sum()
}

mod part1 {
    use crate::{count, parse, Seat};

    fn count_occupied_neighbors(seats: &[Vec<Seat>], i: usize, j: usize) -> usize {
        let mut count = 0;
        for k in i.saturating_sub(1)..=(i.saturating_add(1).min(seats.len() - 1)) {
            for l in j.saturating_sub(1)..=(j.saturating_add(1).min(seats[0].len() - 1)) {
                if (k != i || l != j) && matches!(seats[k][l], Seat::Occupied) {
                    count += 1;
                }
            }
        }
        count
    }

    fn step(seats: Vec<Vec<Seat>>) -> (bool, Vec<Vec<Seat>>) {
        let mut support_vec = seats.clone();
        let mut changed = false;
        for i in 0..support_vec.len() {
            for j in 0..support_vec[0].len() {
                match seats[i][j] {
                    Seat::Empty if count_occupied_neighbors(&seats, i, j) == 0 => {
                        support_vec[i][j] = Seat::Occupied;
                        changed = true;
                    }
                    Seat::Occupied if count_occupied_neighbors(&seats, i, j) >= 4 => {
                        support_vec[i][j] = Seat::Empty;
                        changed = true;
                    }
                    _f => {}
                }
            }
        }
        (changed, support_vec)
    }
    pub(crate) fn solve(input: &str) -> usize {
        let mut seats = parse(input);
        let mut changed = true;
        while changed {
            let moved = step(seats);
            changed = moved.0;
            seats = moved.1;
        }
        count(seats)
    }
}

mod part2 {
    use crate::{count, parse, Seat};

    fn count_occupied_neighbors(seats: &[Vec<Seat>], i: usize, j: usize) -> usize {
        let (i, j) = (i as isize, j as isize);
        let mut count = 0;
        for direction in [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ] {
            let (mut i, mut j) = (i + direction.0, j + direction.1);
            while i >= 0
                && j >= 0
                && i < seats.len() as isize
                && j < seats[0].len() as isize
                && matches!(seats[i as usize][j as usize], Seat::Floor)
            {
                i += direction.0;
                j += direction.1;
            }
            if 0 <= i
                && i < seats.len() as isize
                && 0 <= j
                && j < seats[0].len() as isize
                && matches!(seats[i as usize][j as usize], Seat::Occupied)
            {
                count += 1;
            }
        }
        count
    }

    fn step(seats: Vec<Vec<Seat>>) -> (bool, Vec<Vec<Seat>>) {
        let mut support_vec = seats.clone();
        let mut changed = false;
        for i in 0..support_vec.len() {
            for j in 0..support_vec[0].len() {
                match seats[i][j] {
                    Seat::Empty if count_occupied_neighbors(&seats, i, j) == 0 => {
                        support_vec[i][j] = Seat::Occupied;
                        changed = true;
                    }
                    Seat::Occupied if count_occupied_neighbors(&seats, i, j) >= 5 => {
                        support_vec[i][j] = Seat::Empty;
                        changed = true;
                    }
                    _f => {}
                }
            }
        }
        (changed, support_vec)
    }

    pub(crate) fn solve(input: &str) -> usize {
        let mut seats = parse(input);
        let mut changed = true;
        while changed {
            let moved = step(seats);
            changed = moved.0;
            seats = moved.1;
        }
        count(seats)
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_11_input.txt").unwrap()
    };
    println!("{}", part1::solve(&puzzle_input));
    println!("{}", part2::solve(&puzzle_input));
}
