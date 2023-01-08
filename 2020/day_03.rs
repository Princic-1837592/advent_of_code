use std::time::Instant;

mod part1 {
    pub(crate) fn solve(input: &str) -> usize {
        let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut trees = 0;
        let mut coord = (0, 0);
        while coord.0 < map.len() {
            if map[coord.0][coord.1] == '#' {
                trees += 1;
            }
            coord.0 += 1;
            coord.1 = (coord.1 + 3) % map[0].len();
        }
        trees
    }
}

mod part2 {
    pub(crate) fn solve(input: &str) -> usize {
        let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
        let mut result = 1;
        for (r, c) in &[(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
            let mut trees = 0;
            let mut coord = (0, 0);
            while coord.0 < map.len() {
                if map[coord.0][coord.1] == '#' {
                    trees += 1;
                }
                coord.0 += r;
                coord.1 = (coord.1 + c) % map[0].len();
            }
            result *= trees;
        }
        result
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_03_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
}
}
