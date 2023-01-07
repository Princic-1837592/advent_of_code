fn parse(input: &str) -> (usize, Vec<Option<usize>>) {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse().unwrap();
    (
        timestamp,
        lines
            .next()
            .unwrap()
            .split(',')
            .map(|b| match b.chars().next().unwrap() {
                'x' => None,
                _ => Some(b.parse().unwrap()),
            })
            .collect(),
    )
}
mod part1 {
    use crate::parse;

    pub(crate) fn solve(input: &str) -> usize {
        let (timestamp, buses) = parse(input);
        let (wait, bus) = buses
            .iter()
            .flatten()
            .map(|b| (b - timestamp % b, b))
            .min_by_key(|(next, _bus)| *next)
            .unwrap();
        wait * bus
    }
}

mod part2 {
    use crate::parse;

    fn find_earliest(
        (mut partial, mcm): (usize, usize),
        (prime, offset): (usize, usize),
    ) -> (usize, usize) {
        let target = (prime - offset % prime) % prime;
        while partial % prime != target {
            partial += mcm;
        }
        (partial, mcm * prime)
    }

    pub(crate) fn solve(input: &str) -> usize {
        let (_, buses) = parse(input);
        buses
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.map(|b| (b as usize, i as usize)))
            .fold((0, 1), find_earliest)
            .0
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "939
7,13,x,x,59,x,31,19"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_13_input.txt").unwrap()
    };
    println!("{}", part1::solve(&puzzle_input));
    println!("{}", part2::solve(&puzzle_input));
}
