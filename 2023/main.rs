use std::{env, time::Duration};

use advent_of_code_2023 as aoc;

fn main() {
    let test = env::args().any(|arg| arg == "--test");
    let mut total = Duration::default();

    println!("Running day 1");
    total += aoc::day_01::main(test);
    println!();

    println!("Running day 2");
    total += aoc::day_02::main(test);
    println!();

    println!("Running day 3");
    total += aoc::day_03::main(test);
    println!();

    println!("Running day 4");
    total += aoc::day_04::main(test);
    println!();

    println!("Running day 5");
    total += aoc::day_05::main(test);
    println!();

    println!("Running day 6");
    total += aoc::day_06::main(test);
    println!();

    println!("Running day 7");
    total += aoc::day_07::main(test);
    println!();

    println!("Running day 8");
    total += aoc::day_08::main(test);
    println!();

    println!("Running day 9");
    total += aoc::day_09::main(test);
    println!();

    println!("Running day 10");
    total += aoc::day_10::main(test);
    println!();

    println!("Running day 11");
    total += aoc::day_11::main(test);
    println!();

    println!("Running day 12");
    total += aoc::day_12::main(test);
    println!();

    println!("Running day 13");
    total += aoc::day_13::main(test);
    println!();

    println!("Running day 14");
    total += aoc::day_14::main(test);
    println!();

    println!("Running day 15");
    total += aoc::day_15::main(test);
    println!();

    println!("Running day 16");
    total += aoc::day_16::main(test);
    println!();

    println!("Running day 17");
    total += aoc::day_17::main(test);
    println!();

    println!("Running day 18");
    total += aoc::day_18::main(test);
    println!();

    println!("Running day 19");
    total += aoc::day_19::main(test);
    println!();

    println!("Running day 20");
    total += aoc::day_20::main(test);
    println!();

    println!("Running day 21");
    total += aoc::day_21::main(test);
    println!();

    println!("Running day 22");
    total += aoc::day_22::main(test);
    println!();

    println!("Running day 23");
    total += aoc::day_23::main(test);
    println!();

    println!("Running day 24");
    total += aoc::day_24::main(test);
    println!();

    println!("Running day 25");
    total += aoc::day_25::main(test);
    println!();

    println!("Total: {:?}", total);
}
