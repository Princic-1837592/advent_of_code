//! https://adventofcode.com/2019/day/14
//! https://adventofcode.com/2019/day/14/input

use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    time::Instant,
};

type Input = (usize, String);
type Reactions = HashMap<String, (usize, Vec<Input>)>;

fn parse(input: &str) -> Reactions {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(" => ");
            let (input, output) = (parts.next().unwrap(), parts.next().unwrap());
            let inputs: Vec<_> = input
                .split(", ")
                .map(|input| {
                    let mut parts = input.split_whitespace();
                    (
                        parts.next().unwrap().parse().unwrap(),
                        parts.next().unwrap().to_owned(),
                    )
                })
                .collect();
            let mut output = output.split_whitespace();
            let (produced, output) = (
                output.next().unwrap().parse().unwrap(),
                output.next().unwrap().to_owned(),
            );
            (output, (produced, inputs))
        })
        .collect()
}

fn topological_order(reactions: &Reactions) -> HashMap<String, usize> {
    fn dfs(
        reactions: &Reactions,
        node: String,
        visited: &mut HashSet<String>,
        order: &mut Vec<String>,
    ) {
        visited.insert(node.clone());
        if node == "ORE" {
            return;
        }
        let (_, ingredients) = reactions.get(&node).unwrap();
        for (_, ingredient) in ingredients {
            if !visited.contains(ingredient) {
                dfs(reactions, ingredient.clone(), visited, order);
            }
        }
        order.push(node);
    }
    let mut visited = HashSet::new();
    let mut order = Vec::with_capacity(reactions.len() + 1);
    dfs(reactions, "FUEL".into(), &mut visited, &mut order);
    order.reverse();
    order.push("ORE".into());
    order
        .iter()
        .enumerate()
        .map(|(i, e)| (e.clone(), i))
        .collect()
}

fn find_ore(
    reactions: &Reactions,
    order: &HashMap<String, usize>,
    target: String,
    quantity: usize,
) -> usize {
    let mut ore_required = 0;
    let mut needs = HashMap::from([(target, quantity)]);
    while !needs.is_empty() {
        let chemical = &needs
            .keys()
            .min_by_key(|&v| order.get(v).unwrap())
            .unwrap()
            .clone();
        let qty_required = needs.remove(chemical).unwrap();
        let (qty_produced, ingredients) = reactions.get(chemical).unwrap();
        let n = (qty_required as f32 / *qty_produced as f32).ceil() as usize;
        for (qty_ingredient, ingredient) in ingredients {
            if ingredient == "ORE" {
                ore_required += qty_ingredient * n;
            } else {
                *needs.entry(ingredient.clone()).or_insert(0) += qty_ingredient * n;
            }
        }
    }
    ore_required
}

pub mod part1 {
    use crate::day_14::{find_ore, parse, topological_order};

    pub fn solve(input: &str) -> usize {
        let reactions = parse(input);
        let order = topological_order(&reactions);
        find_ore(&reactions, &order, "FUEL".into(), 1)
    }
}

pub mod part2 {
    use crate::day_14::{find_ore, parse, topological_order};

    pub fn solve(input: &str) -> usize {
        let reactions = parse(input);
        let order = topological_order(&reactions);
        let ore_required = find_ore(&reactions, &order, "FUEL".into(), 1);
        let ore = 1000000000000;
        let (mut l, mut r) = (ore / ore_required, 82892753);
        let mut result = usize::MAX;
        while l < r {
            let mid = (l + r) / 2;
            let ore_required = find_ore(&reactions, &order, "FUEL".into(), mid);
            if ore_required <= ore {
                result = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        result
    }
}

pub fn main(test: bool) {
    let test_input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_14_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
