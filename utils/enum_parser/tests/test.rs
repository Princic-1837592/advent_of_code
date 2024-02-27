use enum_parser::from_char;

#[test]
fn test() {
    #[from_char]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum Cell {
        Path = '.',
        Forest = '#',
        Up = '^',
        Right = '>',
        Down = 'v',
        Left = '<',
    }

    assert_eq!(Cell::Forest, '#'.into());
}
