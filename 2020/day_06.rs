use std::time::Instant;

mod part1 {
    use std::collections::HashSet;

    use advent_of_code_2020::LINE_ENDING;

    pub(crate) fn solve(input: &str) -> usize {
        input
            .split(&LINE_ENDING.repeat(2))
            .map(|g| {
                g.lines()
                    .flat_map(|l| l.chars())
                    .collect::<HashSet<_>>()
                    .len()
            })
            .sum()
    }
}

mod part2 {
    use advent_of_code_2020::LINE_ENDING;

    pub(crate) fn solve(input: &str) -> usize {
        input
            .split(&LINE_ENDING.repeat(2))
            .map(|g| {
                let mut people = g.lines().map(|l| l.chars().collect::<Vec<_>>());
                let mut first = people.next().unwrap();
                people.for_each(|p| first.retain(|e| p.contains(e)));
                first.len()
            })
            .sum()
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "abc

a
b
c

ab
ac

a
a
a
a

b"
    .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_06_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
}
