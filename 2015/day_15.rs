//! https://adventofcode.com/2015/day/15
//! https://adventofcode.com/2015/day/15/input

use std::{
    fs::read_to_string,
    iter::Sum,
    ops::{AddAssign, Mul},
    time::Instant,
};

#[derive(Copy, Clone, Debug, Default)]
struct Ingredient {
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

fn parse(input: &str) -> Vec<Ingredient> {
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

pub mod part1 {
    use itertools::Itertools;

    use crate::day_15::{parse, Ingredient};

    pub fn solve(input: &str) -> isize {
        let ingredients = parse(input);
        (1..=ingredients.len())
            .map(|_| 0..=100)
            .multi_cartesian_product()
            .filter(|spoons| spoons.iter().sum::<isize>() == 100)
            .map(|spoons| {
                ingredients
                    .iter()
                    .zip(spoons)
                    .map(|(&ingredient, spoons)| (ingredient * spoons))
                    .sum::<Ingredient>()
                    .mul()
            })
            .max()
            .unwrap()
    }
}

pub mod part2 {
    use itertools::Itertools;

    use crate::day_15::{parse, Ingredient};

    pub fn solve(input: &str) -> isize {
        let ingredients = parse(input);
        (1..=ingredients.len())
            .map(|_| 0..=100)
            .multi_cartesian_product()
            .filter(|spoons| spoons.iter().sum::<isize>() == 100)
            .map(|spoons| {
                ingredients
                    .iter()
                    .zip(spoons)
                    .map(|(&ingredient, spoons)| (ingredient * spoons))
                    .sum::<Ingredient>()
            })
            .filter(|recipe| recipe.calories == 500)
            .map(|recipe| recipe.mul())
            .max()
            .unwrap()
    }
}

pub fn main(test: bool) {
    let test_input = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"
        .to_owned();
    let puzzle_input = if test {
        test_input
    } else {
        read_to_string("inputs/day_15_input.txt").unwrap()
    };
    let start = Instant::now();
    println!("{}", part1::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
    let start = Instant::now();
    println!("{}", part2::solve(&puzzle_input));
    println!("Run in {:?}", start.elapsed());
}
