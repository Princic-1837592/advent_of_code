//! https://adventofcode.com/2019/day/14
//! https://adventofcode.com/2019/day/14/input

use std::{
    collections::{BinaryHeap, HashMap, HashSet},
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
    target: &String,
    quantity: usize,
) -> usize {
    let mut ore_required = 0;
    let mut needs = HashMap::from([(target, quantity)]);
    let mut queue = BinaryHeap::from([(0, target)]);
    while let Some((_, chemical)) = queue.pop() {
        if let Some(qty_required) = needs.remove(chemical) {
            let (qty_produced, ingredients) = reactions.get(chemical).unwrap();
            let n = qty_required / *qty_produced
                + if (qty_required % *qty_produced) != 0 {
                    1
                } else {
                    0
                };
            for (qty_ingredient, ingredient) in ingredients {
                if ingredient == "ORE" {
                    ore_required += qty_ingredient * n;
                } else {
                    *needs.entry(ingredient).or_insert(0) += qty_ingredient * n;
                    queue.push((-(*order.get(ingredient).unwrap() as isize), ingredient));
                }
            }
        }
    }
    ore_required
}

pub mod part1 {
    use super::{find_ore, parse, topological_order};

    pub fn solve(input: &str) -> usize {
        let reactions = parse(input);
        let order = topological_order(&reactions);
        find_ore(&reactions, &order, &"FUEL".into(), 1)
    }
}

pub mod part2 {
    use super::{find_ore, parse, topological_order};

    pub fn solve(input: &str) -> usize {
        let reactions = parse(input);
        let order = topological_order(&reactions);
        let fuel = &String::from("FUEL");
        let ore_required = find_ore(&reactions, &order, fuel, 1);
        let ore = 1000000000000;
        let mut l = ore / ore_required;
        let mut r = l;
        while find_ore(&reactions, &order, fuel, r) <= ore {
            r *= 2;
        }
        let mut result = usize::MAX;
        while l < r {
            let mid = (l + r) / 2;
            let ore_required = find_ore(&reactions, &order, fuel, mid);
            if ore_required <= ore {
                result = mid;
                l = mid + 1;
            } else {
                r = mid;
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
        read_to_string("../inputs/2019/day_14_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
