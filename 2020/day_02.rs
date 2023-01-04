mod part1 {
    pub(crate) fn solve(input: &str) -> usize {
        input.lines().filter(|line| {
            let mut parts = line.split(' ');
            let mut range = parts.next().unwrap().split('-');
            let min = range.next().unwrap().parse::<usize>().unwrap();
            let max = range.next().unwrap().parse::<usize>().unwrap();
            let letter = parts.next().unwrap().chars().next().unwrap();
            let password = parts.next().unwrap();
            let count = password.chars().filter(|c| *c == letter).count();
            count >= min && count <= max
        }).count()
    }
}

mod part2 {
    pub(crate) fn solve(input: &str) -> usize {
        input.lines().filter(|line| {
            let mut parts = line.split(' ');
            let mut range = parts.next().unwrap().split('-');
            let min = range.next().unwrap().parse::<usize>().unwrap();
            let max = range.next().unwrap().parse::<usize>().unwrap();
            let letter = parts.next().unwrap().chars().next().unwrap();
            let password = parts.next().unwrap();
            (password.chars().nth(min - 1).unwrap() == letter) != (password.chars().nth(max - 1).unwrap() == letter)
        }).count()
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc".to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_02_input.txt").unwrap()
    };
    println!("{}", part1::solve(&puzzle_input));
    println!("{}", part2::solve(&puzzle_input));
}
