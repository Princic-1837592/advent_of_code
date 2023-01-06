import os

python_src_content = """
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
        with open("day_{day:0>2}_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
""".lstrip()

cargo_toml_content = """
[package]
name = "advent_of_code_{year}"
version = "0.1.0"
authors = ["Andrea Princic <princic.1837592@studenti.uniroma1.it>"]

[lib]
name = "advent_of_code_{year}"
path = "lib.rs"

{bins}

[dependencies]
""".lstrip()
rust_src_content = """
mod part1 {{
    pub(crate) fn solve(input: &str) -> usize {{
        0
    }}
}}

mod part2 {{
    pub(crate) fn solve(input: &str) -> usize {{
        0
    }}
}}

fn main() {{
    let test = true;
    // let test = false;
    let test_input = "".to_owned();
    let puzzle_input = if test {{
        test_input
    }} else {{
        std::fs::read_to_string("inputs/day_{day:0>2}_input.txt").unwrap()
    }};
    println!("{{}}", part1::solve(&puzzle_input));
    println!("{{}}", part2::solve(&puzzle_input));
}}
""".lstrip()
cargo_bin_content = """
[[bin]]
name = "day_{day:0>2}"
path = "day_{day:0>2}.rs"
""".lstrip()
rust_lib_content = """
#[cfg(windows)]
pub const LINE_ENDING: &str = "\r\n";
#[cfg(not(windows))]
pub const LINE_ENDING: &str = "\n";
""".lstrip()


def setup_calendar(year: str, language: str = "python"):
    def rust():
        cargo = os.path.join(year, "Cargo.toml")
        if not os.path.exists(cargo):
            with open(cargo, "w") as cargo_toml:
                cargo_toml.write(
                    cargo_toml_content.format(
                        year=year,
                        bins="\n".join(cargo_bin_content.format(day=day) for day in range(1, 25 + 1)),
                    )
                )
        lib = os.path.join(year, "lib.rs")
        if not os.path.exists(lib):
            with open(lib, "w") as lib_rs:
                lib_rs.write(rust_lib_content)
        for day in range(1, 25 + 1):
            path = os.path.join(year, f"day_{day:0>2}.rs")
            if not os.path.exists(path):
                with open(path, "w") as src:
                    src.write(rust_src_content.format(day=day))
            path = os.path.join(year, "inputs", f"day_{day:0>2}_input.txt")
            if not os.path.exists(path):
                with open(path, "w") as _day_input:
                    pass

    def python():
        for day in range(1, 25 + 1):
            path = os.path.join(year, f"day_{day:0>2}.py")
            if not os.path.exists(path):
                with open(path, "w") as src:
                    src.write(python_src_content.format(day=day))
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
    setup_calendar("2020", "rust")
