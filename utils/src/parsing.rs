use std::{fmt::Debug, str::FromStr};

pub fn parse_alpha<const S: char>(chars: &str) -> usize {
    let mut result = 0;
    for char in chars.chars() {
        result = result * 26 + char as usize - S as usize
    }
    result
}

#[macro_export]
macro_rules! parse_matrix {
    ($input:ident, $t:ty) => {
        $input
            .lines()
            .map(|l| l.chars().map(<$t>::from).collect())
            .collect()
    };
}

#[macro_export]
macro_rules! parse_lines {
    ($input:ident, $t:ty) => {
        $input.lines().map(|l| l.parse().unwrap()).collect()
    };
}

pub fn parse_matrix<T: From<char>>(input: &str) -> Vec<Vec<T>> {
    parse_matrix!(input, T)
}

pub fn parse_line<T>(input: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    parse_lines!(input, T)
}
