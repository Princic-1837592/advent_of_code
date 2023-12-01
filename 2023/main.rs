use std::{env, time::Instant};

use advent_of_code_2023 as aoc;

fn main() {
    let test = env::args().any(|arg| arg == "--test");
    let start = Instant::now();

    println!("Running day 1");
    aoc::day_01::main(test);
    println!();

    println!("Running day 2");
    aoc::day_02::main(test);
    println!();

    println!("Running day 3");
    aoc::day_03::main(test);
    println!();

    println!("Running day 4");
    aoc::day_04::main(test);
    println!();

    println!("Running day 5");
    aoc::day_05::main(test);
    println!();

    println!("Running day 6");
    aoc::day_06::main(test);
    println!();

    println!("Running day 7");
    aoc::day_07::main(test);
    println!();

    println!("Running day 8");
    aoc::day_08::main(test);
    println!();

    println!("Running day 9");
    aoc::day_09::main(test);
    println!();

    println!("Running day 10");
    aoc::day_10::main(test);
    println!();

    println!("Running day 11");
    aoc::day_11::main(test);
    println!();

    println!("Running day 12");
    aoc::day_12::main(test);
    println!();

    println!("Running day 13");
    aoc::day_13::main(test);
    println!();

    println!("Running day 14");
    aoc::day_14::main(test);
    println!();

    println!("Running day 15");
    aoc::day_15::main(test);
    println!();

    println!("Running day 16");
    aoc::day_16::main(test);
    println!();

    println!("Running day 17");
    aoc::day_17::main(test);
    println!();

    println!("Running day 18");
    aoc::day_18::main(test);
    println!();

    println!("Running day 19");
    aoc::day_19::main(test);
    println!();

    println!("Running day 20");
    aoc::day_20::main(test);
    println!();

    println!("Running day 21");
    aoc::day_21::main(test);
    println!();

    println!("Running day 22");
    aoc::day_22::main(test);
    println!();

    println!("Running day 23");
    aoc::day_23::main(test);
    println!();

    println!("Running day 24");
    aoc::day_24::main(test);
    println!();

    println!("Running day 25");
    aoc::day_25::main(test);
    println!();

    println!("Total: {:?}", start.elapsed());
}
