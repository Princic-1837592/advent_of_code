//! https://adventofcode.com/2016/day/4
//! https://adventofcode.com/2016/day/4/input

use std::{fs::read_to_string, time::Instant};

struct Room {
    letters: Vec<char>,
    id: usize,
    checksum: String,
}

impl From<&str> for Room {
    fn from(string: &str) -> Self {
        let parts: Vec<_> = string.split('-').collect();
        let letters: Vec<_> = parts[0..parts.len() - 1]
            .iter()
            .flat_map(|sequence| sequence.chars())
            .collect();
        let mut final_part = parts[parts.len() - 1].split('[');
        let id = final_part.next().unwrap().parse().unwrap();
        let checksum = final_part.next().unwrap().trim_end_matches(']').to_owned();
        Room {
            letters,
            id,
            checksum,
        }
    }
}

fn parse(input: &str) -> Vec<Room> {
    input.lines().map(Room::from).collect()
}

fn is_valid(room: &Room) -> bool {
    let mut frequencies = [0; 26];
    for &letter in &room.letters {
        frequencies[letter as usize - 'a' as usize] += 1;
    }
    let mut sorted: Vec<_> = frequencies.iter().enumerate().collect();
    sorted.sort_by_key(|&(i, f)| (-f, i));
    let checksum: String = sorted[0..5]
        .iter()
        .map(|&(i, _)| (i as u8 + b'a') as char)
        .collect();
    checksum == room.checksum
}

pub mod part1 {
    use crate::day_04::{is_valid, parse};

    pub fn solve(input: &str) -> usize {
        parse(input)
            .iter()
            .filter_map(|room| is_valid(room).then_some(room.id))
            .sum()
    }
}

pub mod part2 {
    use crate::day_04::{is_valid, parse};

    pub fn solve(input: &str) -> usize {
        let rooms = parse(input);
        let rooms: Vec<_> = rooms.iter().filter(|room| is_valid(room)).collect();
        for room in rooms {
            let name: String = room
                .letters
                .iter()
                .map(|&char| (((char as usize - 'a' as usize + room.id) % 26) as u8 + b'a') as char)
                .collect();
            if name.starts_with("northpole") {
                return room.id;
            }
        }
        unreachable!()
    }
}

pub fn main(test: bool) {
    let test_input = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]"
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
