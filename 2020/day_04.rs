use std::time::Instant;

mod part1 {
    pub(crate) fn solve(input: &str) -> usize {
        let passports: Vec<Vec<_>> = input
            .split("\r\n\r\n")
            .map(|l| l.split(|c| c == ' ' || c == '\n').collect())
            .collect();
        passports
            .iter()
            .filter(|p| {
                p.len() == 8
                    || (p.iter().filter(|f| f.starts_with("cid:")).count() == 0 && p.len() == 7)
            })
            .count()
    }
}

mod part2 {
    use regex::Regex;

    fn validate(passport: &Vec<&str>) -> bool {
        let pattern = Regex::new(r"^(byr:(19[2-9]\d|200[0-2])|iyr:(201\d|2020)|eyr:(202\d|2030)|hgt:((1[5-8]\d|19[0-3])cm|(59|6\d|7[0-6])in)|hcl:#[a-f0-9]{6}|ecl:(amb|blu|brn|gry|grn|hzl|oth)|pid:\d{9})$").unwrap();
        for field in passport {
            if !field.starts_with("cid:") && !pattern.is_match(field) {
                return false;
            }
        }
        /*for field in passport {
            match (&field[..3], &field[4..]) {
                ("byr", value) => {
                    let year: usize = value.parse().unwrap_or(0);
                    if !(1920..=2002).contains(&year) {
                        return false;
                    }
                }
                ("iyr", value) => {
                    let year: usize = value.parse().unwrap_or(0);
                    if !(2010..=2020).contains(&year) {
                        return false;
                    }
                }
                ("eyr", value) => {
                    let year: usize = value.parse().unwrap_or(0);
                    if !(2020..=2030).contains(&year) {
                        return false;
                    }
                }
                ("hgt", value) => {
                    let height: usize = value[..value.len() - 2].parse().unwrap_or(0);
                    let unit = &value[value.len() - 2..];
                    if unit == "cm" {
                        if !(150..=193).contains(&height) {
                            return false;
                        }
                    } else if !(59..=76).contains(&height) {
                        return false;
                    }
                }
                ("hcl", value) => {
                    let mut chars = value.chars();
                    if chars.next().unwrap() != '#' {
                        return false;
                    }
                    if chars.filter(|c| ('a'..='f').contains(c) || ('0'..='9').contains(c)).count() != 6 {
                        return false;
                    }
                }
                ("ecl", value) => {
                    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value) {
                        return false;
                    }
                }
                ("pid", value) => {
                    if value.chars().filter(|c| ('0'..='9').contains(c)).count() != 9 {
                        return false;
                    }
                }
                _ => {}
            }
        }*/
        true
    }

    pub(crate) fn solve(input: &str) -> usize {
        input
            .split("\r\n\r\n")
            .map(|l| l.split_whitespace().collect::<Vec<_>>())
            .filter(|p| {
                p.len() == 8
                    || (p.iter().filter(|f| f.starts_with("cid:")).count() == 0 && p.len() == 7)
            })
            .filter(validate)
            .count()
    }
}

fn main() {
    // let test = true;
    let test = false;
    let test_input = "yecl:gry pid:998952368 eyr:2026 hcl:#fffffd
byr:1940 iyr:2014 cid:147 hgt:174cm"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_04_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
}
}
