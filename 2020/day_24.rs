use std::{collections::HashSet, fs::read_to_string, time::Instant};

fn parse(input: &str) -> Vec<Vec<(i8, i8, i8)>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .fold((vec![], ' '), |(mut vec, prec), char| match char {
                    c @ ('s' | 'n') => (vec, c),
                    _ => {
                        vec.push(match (prec, char) {
                            ('n', 'e') => (0, 1, -1),
                            (' ', 'e') => (-1, 1, 0),
                            ('s', 'e') => (-1, 0, 1),
                            ('s', 'w') => (0, -1, 1),
                            (' ', 'w') => (1, -1, 0),
                            ('n', 'w') => (1, 0, -1),
                            _ => panic!("Invalid coordinate: {}", char),
                        });
                        (vec, ' ')
                    }
                })
                .0
        })
        .collect()
}

fn get_black_tiles(tiles: Vec<Vec<(i8, i8, i8)>>) -> HashSet<(i8, i8, i8)> {
    let mut black = HashSet::new();
    tiles.iter().for_each(|tile| {
        let coord = tile.iter().fold((0, 0, 0), |acc, tile| {
            (acc.0 + tile.0, acc.1 + tile.1, acc.2 + tile.2)
        });
        if black.contains(&coord) {
            black.remove(&coord);
        } else {
            black.insert(coord);
        }
    });
    black
}

pub mod part1 {
    use super::{get_black_tiles, parse};

    pub fn solve(input: &str) -> usize {
        let tiles = parse(input);
        let black = get_black_tiles(tiles);
        black.len()
    }
}

pub mod part2 {
    use std::collections::{HashMap, HashSet};

    use super::{get_black_tiles, parse};

    fn day(black: HashSet<(i8, i8, i8)>) -> HashSet<(i8, i8, i8)> {
        let mut neighbors_count = HashMap::new();
        let neighbors = [
            (-1, 1, 0),
            (-1, 0, 1),
            (0, 1, -1),
            (1, -1, 0),
            (0, -1, 1),
            (1, 0, -1),
        ];
        black.iter().for_each(|tile| {
            for neighbor in neighbors
                .iter()
                .map(|n| (n.0 + tile.0, n.1 + tile.1, n.2 + tile.2))
            {
                *neighbors_count.entry(neighbor).or_insert(0) += 1;
            }
        });
        let mut result: HashSet<_> = black
            .iter()
            .filter_map(|tile| {
                let neighbors = *neighbors_count.get(tile).unwrap_or(&0);
                (neighbors == 1 || neighbors == 2).then_some(*tile)
            })
            .collect();
        result.extend(
            neighbors_count
                .iter()
                .filter(|(tile, _)| !black.contains(tile))
                .filter_map(|(tile, &neighbors)| (neighbors == 2).then_some(tile)),
        );
        result
    }
    pub fn solve(input: &str) -> usize {
        let tiles = parse(input);
        let mut black = get_black_tiles(tiles);
        for _ in 0..100 {
            black = day(black);
        }
        black.len()
    }
}

pub fn main(test: bool) {
    let test_input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"
        .to_owned();
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
