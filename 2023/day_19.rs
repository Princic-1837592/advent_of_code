//! https://adventofcode.com/2023/day/19
//! https://adventofcode.com/2023/day/19/input

use std::{
    collections::HashMap,
    fs::read_to_string,
    time::{Duration, Instant},
};

use crate::LINE_ENDING;

#[derive(Copy, Clone, Debug)]
pub struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mut parts = value[1..value.len() - 1].split(',');
        Self {
            x: parts.next().unwrap()[2..].parse().unwrap(),
            m: parts.next().unwrap()[2..].parse().unwrap(),
            a: parts.next().unwrap()[2..].parse().unwrap(),
            s: parts.next().unwrap()[2..].parse().unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let name = &value[0..value.find('{').unwrap()];
        let rules = &value[name.len() + 1..value.len() - 1];
        Self {
            name: name.to_owned(),
            rules: rules.split(',').map(Rule::from).collect(),
        }
    }
}

#[derive(Clone, Debug)]
struct Rule {
    category: Category,
    greater: bool,
    value: usize,
    target: String,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split(':').collect();
        if parts.len() == 1 {
            Self {
                category: Category::Last,
                greater: false,
                value: 0,
                target: parts[0].to_owned(),
            }
        } else {
            Self {
                category: parts[0].chars().next().unwrap().into(),
                greater: parts[0].chars().nth(1).unwrap() == '>',
                value: parts[0][2..].parse().unwrap(),
                target: parts[1].to_owned(),
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Category {
    X,
    M,
    A,
    S,
    Last,
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => unreachable!(),
        }
    }
}

type Parsed = (Vec<Part>, HashMap<String, Workflow>);

fn parse(input: &str, separator: String) -> Parsed {
    let mut split = input.split(&separator);
    let workflows = split
        .next()
        .unwrap()
        .lines()
        .map(Workflow::from)
        .map(|w| (w.name.clone(), w))
        .collect();
    let parts = split.next().unwrap().lines().map(Part::from).collect();
    (parts, workflows)
}

pub mod part1 {
    use super::{Category, Parsed, Part, Rule, Workflow};

    impl Workflow {
        fn evaluate(&self, part: Part) -> String {
            for Rule {
                category,
                greater,
                value,
                target,
            } in &self.rules
            {
                if let Category::Last = category {
                    return target.clone();
                }
                let cmp = |p: usize| if *greater { p > *value } else { p < *value };
                if cmp(match category {
                    Category::X => part.x,
                    Category::M => part.m,
                    Category::A => part.a,
                    Category::S => part.s,
                    Category::Last => {
                        unreachable!()
                    }
                }) {
                    return target.clone();
                }
            }
            unreachable!()
        }
    }

    pub fn solve((parts, workflows): Parsed) -> usize {
        let mut result = 0;
        for part in parts {
            let mut workflow = "in".to_owned();
            while workflow != "A" && workflow != "R" {
                workflow = workflows.get(&workflow).unwrap().evaluate(part)
            }
            if workflow == "A" {
                result += part.x + part.m + part.a + part.s;
            }
        }
        result
    }
}

pub mod part2 {
    use std::collections::HashMap;

    use super::{Category, Parsed, Rule, Workflow};

    type Xmas = (
        (usize, usize),
        (usize, usize),
        (usize, usize),
        (usize, usize),
    );

    impl Rule {
        fn shrink_ranges(&self, mut xmas: Xmas) -> (Xmas, Option<Xmas>) {
            if let Category::Last = self.category {
                return (xmas, None);
            }
            let shrink = |(p, o): (&mut (usize, usize), &mut (usize, usize))| {
                if self.greater {
                    o.0 = p.0;
                    p.0 = p.0.max(self.value + 1);
                    o.1 = p.0 - 1
                } else {
                    o.1 = p.1;
                    p.1 = p.1.min(self.value - 1);
                    o.0 = p.1 + 1;
                }
            };
            let mut otherwise = xmas;
            shrink(match self.category {
                Category::X => (&mut xmas.0, &mut otherwise.0),
                Category::M => (&mut xmas.1, &mut otherwise.1),
                Category::A => (&mut xmas.2, &mut otherwise.2),
                Category::S => (&mut xmas.3, &mut otherwise.3),
                Category::Last => {
                    unreachable!()
                }
            });
            (xmas, Some(otherwise))
        }
    }

    fn find_acceptable(
        mut xmas @ (x, m, a, s): Xmas,
        workflow: String,
        workflows: &HashMap<String, Workflow>,
    ) -> usize {
        if x.1 < x.0 || m.1 < m.0 || a.1 < a.0 || s.1 < s.0 {
            0
        } else if workflow == "A" {
            (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1)
        } else if workflow == "R" {
            0
        } else {
            let mut result = 0;
            for rule in &workflows.get(&workflow).unwrap().rules {
                let (new_xmas, otherwise) = rule.shrink_ranges(xmas);
                result += find_acceptable(new_xmas, rule.target.clone(), workflows);
                if let Some(otherwise) = otherwise {
                    xmas = otherwise;
                }
            }
            result
        }
    }

    pub fn solve((_parts, workflows): Parsed) -> usize {
        find_acceptable(
            ((1, 4000), (1, 4000), (1, 4000), (1, 4000)),
            "in".to_owned(),
            &workflows,
        )
    }
}

pub fn main(test: bool, verbose: bool) -> Duration {
    let test_input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
        .to_owned();
    let (puzzle_input, separator) = if test {
        (test_input, "\n".repeat(2))
    } else {
        (
            read_to_string("inputs/day_19_input.txt").unwrap(),
            LINE_ENDING.repeat(2),
        )
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input, separator);
    let elapsed = start.elapsed();
    if verbose {
        println!("Parsed in {:?}", elapsed);
        total += elapsed;
    }

    let start = Instant::now();
    let result = part1::solve(parsed.clone());
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("First part in {:?}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(parsed);
    let elapsed = start.elapsed();
    println!("{}", result);
    println!("Second part in {:?}", elapsed);
    total += elapsed;

    if verbose {
        println!("Total {:?}", total);
    }
    total
}
