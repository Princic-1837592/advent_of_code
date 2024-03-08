//! https://adventofcode.com/2015/day/15
//! https://adventofcode.com/2015/day/15/input

use std::{
    fs::read_to_string,
    iter::Sum,
    ops::{AddAssign, Mul},
    time::{Duration, Instant},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct Ingredient {
    capacity: isize,
    durability: isize,
    flavor: isize,
    texture: isize,
    calories: isize,
}

impl Ingredient {
    fn mul(&self) -> isize {
        self.capacity * self.durability * self.flavor * self.texture
    }
}

impl Mul<isize> for Ingredient {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Ingredient {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl AddAssign<Ingredient> for Ingredient {
    fn add_assign(&mut self, rhs: Ingredient) {
        self.capacity += rhs.capacity;
        self.durability += rhs.durability;
        self.flavor += rhs.flavor;
        self.texture += rhs.texture;
        self.calories += rhs.calories;
    }
}

impl Sum for Ingredient {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = Ingredient::default();
        for ingredient in iter {
            result += ingredient;
        }
        Ingredient {
            capacity: result.capacity.max(0),
            durability: result.durability.max(0),
            flavor: result.flavor.max(0),
            texture: result.texture.max(0),
            calories: result.calories,
        }
    }
}

type Parsed = Vec<Ingredient>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line
                .split(", ")
                .map(|part| part.split(' ').last().unwrap().parse().unwrap())
                .collect();
            Ingredient {
                capacity: parts[0],
                durability: parts[1],
                flavor: parts[2],
                texture: parts[3],
                calories: parts[4],
            }
        })
        .collect()
}

fn solve_generic(ingredients: Vec<Ingredient>, check_calories: bool) -> isize {
    let n = ingredients.len();
    let mut max = 0;
    let mut spoons = vec![0; n];
    let mut total = 0;
    for _ in 1.. {
        if add_one(&mut spoons, &mut total) {
            break;
        }
        if total != 100 {
            continue;
        }
        let recipe = ingredients
            .iter()
            .zip(&spoons)
            .map(|(&ingredient, &spoons)| (ingredient * spoons))
            .sum::<Ingredient>();
        if recipe.capacity <= 0
            || recipe.durability <= 0
            || recipe.flavor <= 0
            || recipe.texture <= 0
            || check_calories && recipe.calories != 500
        {
            continue;
        }
        max = max.max(Ingredient::mul(&recipe));
    }
    max
}

fn add_one(spoons: &mut [isize], total: &mut isize) -> bool {
    let mut carry = true;
    for spoon in spoons {
        if carry {
            *spoon += 1;
            *total += 1;
            carry = *spoon > 100;
            if carry {
                *spoon = 0;
                *total -= 101;
            }
        }
    }
    carry
}

pub mod part1 {
    use super::{solve_generic, Parsed};

    pub fn solve(ingredients: Parsed) -> isize {
        solve_generic(ingredients, false)
    }
}

pub mod part2 {
    use super::{solve_generic, Parsed};

    pub fn solve(ingredients: Parsed) -> isize {
        solve_generic(ingredients, true)
    }
}

pub fn main(test: bool) -> Duration {
    let test_input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("../inputs/2015/day_15_input.txt").unwrap()
    };

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    println!("Parsed in {:?}", elapsed);
    total += elapsed;

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

    println!("Total {:?}", total);
    total
}
