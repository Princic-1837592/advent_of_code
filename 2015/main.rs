use std::{env, time::Duration};

macro_rules! run_days {
    ($($day:ident),* $(,)*) => {
        let test = env::args().any(|arg| arg == "--test");
        let mut total = Duration::default();

        $(
            println!("Running {}", stringify!($day));
            total += advent_of_code_2015::$day::main(test);
            println!();
        )*

        println!("Total: {:?}", total);
    };
}

fn main() {
    run_days!(
        day_01, /**/
        day_02, /**/
        day_03, /**/
        day_04, /**/
        day_05, /**/
        day_06, /**/
        day_07, /**/
        day_08, /**/
        day_09, /**/
        day_10, /**/
        day_11, /**/
        day_12, /**/
        day_13, /**/
        day_14, /**/
        day_15, /**/
        day_16, /**/
        day_17, /**/
        day_18, /**/
        day_19, /**/
        day_20, /**/
        day_21, /**/
        day_22, /**/
        day_23, /**/
        day_24, /**/
        day_25, /**/
    );
}
