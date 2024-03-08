import os
from configparser import ConfigParser
from time import sleep

from requests import get

python_src_content = """
# https://adventofcode.com/{year}/day/{day}
# https://adventofcode.com/{year}/day/{day}/input


def parse(data: str):
    return data


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
        with open("../inputs/{year}/day_{day:0>2}_input.txt", "r") as input_file:
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

use std::{{
    fs::read_to_string,
    time::{{Duration, Instant}},
}};

type Parsed = Vec<usize>;

fn parse(_input: &str) -> Parsed {{
    vec![]
}}

pub mod part1 {{
    use super::Parsed;

    pub fn solve(_parsed: Parsed) -> usize {{
        0
    }}
}}

pub mod part2 {{
    use super::Parsed;

    pub fn solve(_parsed: Parsed) -> usize {{
        0
    }}
}}

pub fn main(test: bool, verbose: bool) -> Duration {{
    let test_input = "".to_owned();
    let puzzle_input = if test {{
        test_input
    }} else {{
        read_to_string("../inputs/{year}/day_{day:0>2}_input.txt").unwrap()
    }};

    let mut total = Duration::default();

    let start = Instant::now();
    let parsed = parse(&puzzle_input);
    let elapsed = start.elapsed();
    if verbose {{
        println!("Parsed in {{:?}}", elapsed);
        total += elapsed;
    }}

    let start = Instant::now();
    let result = part1::solve(parsed.clone());
    let elapsed = start.elapsed();
    println!("{{}}", result);
    println!("First part in {{:?}}", elapsed);
    total += elapsed;

    let start = Instant::now();
    let result = part2::solve(parsed);
    let elapsed = start.elapsed();
    println!("{{}}", result);
    println!("Second part in {{:?}}", elapsed);
    total += elapsed;

    if verbose {{
        println!("Total {{:?}}", total);
    }}
    total
}}
""".lstrip()
rust_main_content = """
use std::{{env, time::Duration}};

use utils::run_days;

fn main() {{
    run_days!(
        advent_of_code_{year},
        {single_days}
    );
}}
""".lstrip()
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


def setup_calendar(year: str, language: str = "python", auto_download=True, verbose: bool = False):
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
        path = os.path.join(year, "main.rs")
        if not os.path.exists(path):
            with open(path, "w") as main:
                main.write(
                    rust_main_content.format(
                        year=year,
                        single_days="\n        ".join(f"day_{day:0>2}, /**/" for day in range(1, 25 + 1))
                    )
                )

    def python():
        for day in range(1, 25 + 1):
            path = os.path.join(year, f"day_{day:0>2}.py")
            if not os.path.exists(path):
                with open(path, "w") as src:
                    src.write(python_src_content.format(year=year, day=day))

    def inputs():
        directory = os.path.join("inputs", year)
        if not os.path.exists(directory):
            os.mkdir(directory)
        session = "None"
        if auto_download:
            config = ConfigParser()
            config.read("local_config.ini")
            session = config["auto_download"]["session"]
        keep_downloading = True
        for day in range(1, 25 + 1):
            path = os.path.join(directory, f"day_{day:0>2}_input.txt")
            if not os.path.exists(path):
                with open(path, "w") as day_input:
                    if auto_download and keep_downloading:
                        response = get(
                            f"https://adventofcode.com/{year}/day/{day}/input",
                            cookies={"session": session},
                            headers={
                                "User-Agent": "https://github.com/Princic-1837592/advent_of_code/blob/main/setup.py"
                                              " by princic.1837592@studenti.uniroma1.it"
                            },
                        )
                        if response.status_code == 200:
                            day_input.write(response.text.rstrip("\n"))
                            if verbose:
                                print(f"Downloaded day {day}")
                        else:
                            keep_downloading = False
                            print(
                                f"Failed to download day {day} with error code "
                                f"{response.status_code} and response: {response.text}. Quitting"
                            )
            sleep(0.2)

    if not os.path.exists(year):
        if verbose:
            print("Creating directory")
        os.mkdir(year)
    if verbose:
        print("Creating source files")
    if language == "python":
        python()
    elif language == "rust":
        rust()
    if verbose:
        print("Creating inputs")
    inputs()


if __name__ == "__main__":
    setup_calendar("2023", "rust", False, True)
