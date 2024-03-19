pub use proc_macros::{from_char, FromStr};

pub mod coords;
pub mod math;
pub mod matrix;
#[macro_use]
pub mod parsing;

#[macro_export]
macro_rules! run_days {
    ($year:ident, $($day:ident),* $(,)*) => {
        let args: Vec<_> = env::args().collect();
        let test = args.iter().any(|arg| arg == "--test");
        let verbose = args.iter().any(|arg| arg == "--verbose");
        let mut total = Duration::default();

        $(
            println!("Running {}", stringify!($day));
            total += $year::$day::main(test, verbose);
            println!();
        )*

        println!("Total: {:?}", total);
    };
}
