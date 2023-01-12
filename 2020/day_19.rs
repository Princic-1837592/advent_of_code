use std::{cmp::Ordering, time::Instant};

use crate::LINE_ENDING;

#[derive(Debug, Clone)]
struct Rule {
    number: usize,
    rule: Match,
}

impl Eq for Rule {}

impl PartialEq<Self> for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl PartialOrd<Self> for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number.partial_cmp(&other.number)
    }
}

impl Ord for Rule {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

#[derive(Debug, Clone)]
enum Match {
    Char(char),
    And(Vec<usize>),
    Or(Vec<usize>, Vec<usize>),
}

fn parse(input: &str) -> (Vec<Rule>, Vec<String>) {
    let separator = LINE_ENDING.repeat(2);
    let mut parts = input.split(&separator);
    let mut rules: Vec<_> = parts.next().unwrap().lines().map(From::from).collect();
    rules.sort();
    let messages = parts.next().unwrap();
    (rules, messages.lines().map(|l| l.to_owned()).collect())
}

impl From<&str> for Rule {
    fn from(string: &str) -> Self {
        let mut parts = string.split(": ");
        Rule {
            number: parts.next().unwrap().parse().unwrap(),
            rule: Match::from(parts.next().unwrap()),
        }
    }
}

impl From<&str> for Match {
    fn from(string: &str) -> Self {
        let rule = string.split(": ").next().unwrap();
        if rule.contains('"') {
            return Match::Char(rule.chars().nth(1).unwrap());
        }
        if rule.contains('|') {
            let mut parts = rule.split(" | ");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
            return Match::Or(
                left.split(' ').map(|s| s.parse().unwrap()).collect(),
                right.split(' ').map(|s| s.parse().unwrap()).collect(),
            );
        }
        Match::And(rule.split(' ').map(|s| s.parse().unwrap()).collect())
    }
}

pub mod part1 {
    use super::{parse, Match, Rule};

    fn matches(message: &str, rule: usize, rules: &Vec<Rule>) -> (bool, usize) {
        if message.is_empty() {
            return (false, 0);
        }
        match rules[rule].rule {
            Match::Char(char) => {
                if message.starts_with(char) {
                    (true, 1)
                } else {
                    (false, 0)
                }
            }
            Match::And(ref and) => {
                let mut index = 0;
                for &rule in and {
                    let (matches, consumed) = matches(&message[index..], rule, rules);
                    if !matches {
                        return (false, 0);
                    }
                    index += consumed;
                }
                (true, index)
            }
            Match::Or(ref left, ref right) => {
                let mut index = 0;
                let mut matches_left = true;
                for &rule in left {
                    let (matches, consumed) = matches(&message[index..], rule, rules);
                    if !matches {
                        matches_left = false;
                        break;
                    }
                    index += consumed;
                }
                if matches_left {
                    return (true, index);
                }
                index = 0;
                for &rule in right {
                    let (matches, consumed) = matches(&message[index..], rule, rules);
                    if !matches {
                        return (false, 0);
                    }
                    index += consumed;
                }
                (true, index)
            }
        }
    }

    pub fn solve(input: &str) -> usize {
        let (rules, messages) = parse(input);
        messages
            .iter()
            .filter(|m| {
                let (matches, lenght) = matches(m, 0, &rules);
                matches && lenght == m.len()
            })
            .count()
    }
}

pub mod part2 {
    use super::{parse, Match, Rule};

    fn matches(message: &str, rule: usize, rules: &Vec<Rule>) -> (bool, Vec<usize>) {
        if message.is_empty() {
            return (false, Vec::new());
        }
        match rules[rule].rule {
            Match::Char(char) => {
                if message.starts_with(char) {
                    (true, vec![1])
                } else {
                    (false, Vec::new())
                }
            }
            Match::And(ref and) => {
                let mut indexes: Vec<_> = vec![0];
                for &rule in and {
                    let mut new_indexes = Vec::new();
                    for index in indexes.iter() {
                        let (matches, consumed) = matches(&message[*index..], rule, rules);
                        if matches {
                            new_indexes.extend(consumed.iter().map(|c| index + c));
                        }
                    }
                    if new_indexes.is_empty() {
                        return (false, Vec::new());
                    }
                    indexes = new_indexes;
                }
                (true, indexes)
            }
            Match::Or(ref left, ref right) => {
                let mut indexes: Vec<_> = vec![0];
                let mut matches_left = true;
                for &rule in left {
                    let mut new_indexes = Vec::new();
                    for index in indexes.iter() {
                        let (matches, consumed) = matches(&message[*index..], rule, rules);
                        if matches {
                            new_indexes.extend(consumed.iter().map(|c| index + c));
                        }
                    }
                    if new_indexes.is_empty() {
                        matches_left = false;
                        break;
                    }
                    indexes = new_indexes;
                }
                let left_indexes = indexes;
                indexes = vec![0];
                let mut matches_right = true;
                for &rule in right {
                    let mut new_indexes = Vec::new();
                    for index in indexes.iter() {
                        let (matches, consumed) = matches(&message[*index..], rule, rules);
                        if matches {
                            new_indexes.extend(consumed.iter().map(|c| index + c));
                        }
                    }
                    if new_indexes.is_empty() {
                        matches_right = false;
                        break;
                    }
                    indexes = new_indexes;
                }
                let mut result = vec![];
                if matches_left {
                    result.extend(left_indexes);
                }
                if matches_right {
                    result.extend(indexes);
                }
                (true, result)
            }
        }
    }

    pub fn solve(input: &str) -> usize {
        let (mut rules, messages) = parse(input);
        rules[8] = Rule {
            number: 8,
            rule: Match::Or(vec![42], vec![42, 8]),
        };
        rules[11] = Rule {
            number: 11,
            rule: Match::Or(vec![42, 31], vec![42, 11, 31]),
        };
        messages
            .iter()
            .filter(|m| {
                let (matches, lenghts) = matches(m, 0, &rules);
                matches && lenghts.contains(&m.len())
            })
            .count()
    }
}

pub fn main(test: bool) {
    let test_input = "0: 8 11
1: \"a\"
2: 1 24 | 14 4
3: 5 14 | 16 1
4: 1 1
5: 1 14 | 15 1
6: 14 14 | 1 14
7: 14 5 | 1 21
8: 42
9: 14 27 | 1 26
10: 23 14 | 28 1
11: 42 31
12: 24 14 | 19 1
13: 14 3 | 1 12
14: \"b\"
15: 1 | 14
16: 15 1 | 14 14
17: 14 2 | 1 7
18: 15 15
19: 14 1 | 14 14
20: 14 14 | 1 15
21: 14 1 | 1 14
22: 14 14
23: 25 1 | 22 14
24: 14 1
25: 1 1 | 1 14
26: 14 22 | 1 20
27: 1 6 | 14 18
28: 16 1
29: 29
30: 29
31: 14 17 | 1 13
32: 29
33: 29
34: 29
35: 29
36: 29
37: 29
38: 29
39: 29
40: 29
41: 29
42: 9 14 | 10 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
        .to_owned()
        .replace('\n', "\r\n");
    let puzzle_input = if test {
        test_input
    } else {
        std::fs::read_to_string("inputs/day_19_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("{:?}", start.elapsed());
}
