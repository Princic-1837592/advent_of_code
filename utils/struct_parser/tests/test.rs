use struct_parser::from_line;

#[test]
fn test() {
    #[from_line(',')]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct Test1 {
        x: usize,
        y: usize,
    }

    assert_eq!(Test1 { x: 3, y: 4 }, "3, 4".into());

    #[from_line("--")]
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct Test2 {
        x: usize,
        y: usize,
    }

    assert_eq!(Test2 { x: 3, y: 4 }, "3 -- 4".into());
}
