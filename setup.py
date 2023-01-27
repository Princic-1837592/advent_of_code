import os
from configparser import ConfigParser
from requests import get

python_src_content = """
# https://adventofcode.com/{year}/day/{day}
# https://adventofcode.com/{year}/day/{day}/input


def part1(data: str):
    pass


def part2(data: str):
    pass


if __name__ == "__main__":
    test = True
    # test = False
    test_input = ""\"""\"
    if test:
        puzzle_input = test_input
    else:
        with open("inputs/day_{day:0>2}_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
""".lstrip()

cargo_toml_content = """
[package]
name = "advent_of_code_{year}"
version = "0.1.0"
authors = ["Andrea Princic <princic.1837592@studenti.uniroma1.it>"]
edition = "2021"

[lib]
name = "advent_of_code_{year}"
path = "lib.rs"

[[bin]]
name = "main"
path = "main.rs"

[dependencies]
""".lstrip()
rust_src_content = """
//! https://adventofcode.com/{year}/day/{day}
//! https://adventofcode.com/{year}/day/{day}/input

use std::{{fs::read_to_string, time::Instant}};

#[allow(unused)]
fn parse(input: &str) -> usize {{
    0
}}

#[allow(unused)]
pub mod part1 {{
    use crate::day_{day:0>2}::parse;

    pub fn solve(input: &str) -> usize {{
        0
    }}
}}

#[allow(unused)]
pub mod part2 {{
    use crate::day_{day:0>2}::parse;

    pub fn solve(input: &str) -> usize {{
        0
    }}
}}

pub fn main(test: bool) {{
    let test_input = "".to_owned();
    let puzzle_input = if test {{
        test_input
    }} else {{
        read_to_string("inputs/day_{day:0>2}_input.txt").unwrap()
    }};
    let start = Instant::now();
    println!("{{}}", part1::solve(&puzzle_input));
    println!("Run in {{:?}}", start.elapsed());
    let start = Instant::now();
    println!("{{}}", part2::solve(&puzzle_input));
    println!("Run in {{:?}}", start.elapsed());
}}
""".lstrip()
rust_main_content = """
use std::{{env, time::Instant}};

use advent_of_code_{year} as aoc;

fn main() {{
    let test = env::args().any(|arg| arg == "--test");
    let start = Instant::now();
{single_days}
    println!("Total: {{:?}}", start.elapsed());
}}

""".lstrip()
rust_single_day_content = """
    println!("Running day {day}");
    aoc::day_{day:0>2}::main(test);
    println!();
"""
rust_lib_content = """
{mods}
#[cfg(windows)]
pub const LINE_ENDING: &str = "\\r\\n";
#[cfg(not(windows))]
pub const LINE_ENDING: &str = "\\n";
""".lstrip()
rust_lib_mod_content = """
pub mod day_{day:0>2};
""".lstrip()


def setup_calendar(year: str, language: str = "python", auto_download=True):
    def rust():
        cargo = os.path.join(year, "Cargo.toml")
        if not os.path.exists(cargo):
            with open(cargo, "w") as cargo_toml:
                cargo_toml.write(cargo_toml_content.format(year=year))
        lib = os.path.join(year, "lib.rs")
        if not os.path.exists(lib):
            with open(lib, "w") as lib_rs:
                lib_rs.write(
                    rust_lib_content.format(
                        mods="".join(rust_lib_mod_content.format(day=day) for day in range(1, 25 + 1))
                    )
                )
        for day in range(1, 25 + 1):
            path = os.path.join(year, f"day_{day:0>2}.rs")
            if not os.path.exists(path):
                with open(path, "w") as src:
                    src.write(rust_src_content.format(year=year, day=day))
            path = os.path.join(year, "inputs", f"day_{day:0>2}_input.txt")
            if not os.path.exists(path):
                with open(path, "w") as day_input:
                    if auto_download:
                        config = ConfigParser()
                        config.read("local_config.ini")
                        session = config["auto_download"]["session"]
                        response = get(
                            f"https://adventofcode.com/{year}/day/{day}/input",
                            cookies={"session": session},
                            headers={"User-Agent": "https://github.com/Princic-1837592/advent_of_code"},
                        )
                        day_input.write(response.text)
        path = os.path.join(year, "main.rs")
        if not os.path.exists(path):
            with open(path, "w") as main:
                main.write(
                    rust_main_content.format(
                        year=year,
                        single_days="".join(rust_single_day_content.format(day=day) for day in range(1, 25 + 1))
                    )
                )

    def python():
        for day in range(1, 25 + 1):
            path = os.path.join(year, f"day_{day:0>2}.py")
            if not os.path.exists(path):
                with open(path, "w") as src:
                    src.write(python_src_content.format(year=year, day=day))
            path = os.path.join(year, "inputs", f"day_{day:0>2}_input.txt")
            if not os.path.exists(path):
                with open(path, "w") as _day_input:
                    pass

    if not os.path.exists(year):
        os.mkdir(year)
    inputs = os.path.join(year, "inputs")
    if not os.path.exists(inputs):
        os.mkdir(inputs)
    if language == "python":
        python()
    elif language == "rust":
        rust()


if __name__ == "__main__":
    setup_calendar("2018", "rust", True)
