pub fn parse_alpha<const S: char>(chars: &str) -> usize {
    let mut result = 0;
    for char in chars.chars() {
        result = result * 26 + char as usize - S as usize
    }
    result
}
