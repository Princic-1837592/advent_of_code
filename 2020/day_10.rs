use std::time::Instant;

fn parse(input: &str) -> Vec<usize> {
    let mut jolts: Vec<_> = input.lines().map(|n| n.parse().unwrap()).collect();
    jolts.push(0);
    jolts.sort();
    jolts.push(jolts[jolts.len() - 1] + 3);
    jolts
}

mod part1 {
    use crate::parse;

    pub(crate) fn solve(input: &str) -> usize {
        let jolts = parse(input);
        let result = jolts
            .iter()
            .fold((0, (0, 0)), |(prev, (ones, threes)), &e| {
                (
                    e,
                    match e - prev {
                        1 => (ones + 1, threes),
                        3 => (ones, threes + 1),
                        _ => (ones, threes),
                    },
                )
            })
            .1;
        result.0 * result.1
    }
}

mod part2 {
    use crate::parse;

    pub(crate) fn solve(input: &str) -> usize {
        let jolts = parse(input);
        let mut dynamic = vec![0; jolts.len()];
        dynamic[0] = 1;
        dynamic[1] = 1;
        for i in 2..jolts.len() {
            let mut combs = dynamic[i - 1];
            if i >= 2 && jolts[i] - jolts[i - 2] <= 3 {
                combs += dynamic[i - 2];
            }
            if i >= 3 && jolts[i] - jolts[i - 3] <= 3 {
                combs += dynamic[i - 3];
            }
            dynamic[i] = combs;
        }
        dynamic[dynamic.len() - 1]
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
    .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_10_input.txt").unwrap()
    };
    println!("{}", part1::solve(&puzzle_input));
    println!("{}", part2::solve(&puzzle_input));
}
