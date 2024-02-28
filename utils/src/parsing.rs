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
        $input.lines().map(<$t>::from).collect()
    };
}
