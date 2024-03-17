use std::fmt::Debug;

use proc_macros::{from_char, FromStr};

#[test]
fn enums() {
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

#[test]
fn structs() {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, FromStr)]
    #[separator(',')]
    pub struct Test1 {
        x: usize,
        y: usize,
    }

    assert_eq!(Test1 { x: 3, y: 4 }, "3, 4".parse().unwrap());

    #[derive(Copy, Clone, Debug, Eq, PartialEq, FromStr)]
    #[separator("--")]
    pub struct Test2 {
        x: usize,
        y: usize,
    }

    assert_eq!(Test2 { x: 3, y: 4 }, "3 -- 4".parse().unwrap());

    #[derive(Copy, Clone, Debug, Eq, PartialEq, FromStr)]
    pub struct Test3 {
        x: usize,
        y: usize,
    }

    assert_eq!(Test3 { x: 3, y: 4 }, "3   4".parse().unwrap());
    assert_eq!(Test3 { x: 3, y: 4 }, "3 4".parse().unwrap());
}

#[test]
fn advent() {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Default, FromStr)]
    #[separator(',')]
    struct Triple {
        x: isize,
        y: isize,
        z: isize,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, FromStr)]
    #[separator('@')]
    pub struct Hail {
        position: Triple,
        velocity: Triple,
    }

    assert_eq!(
        Hail {
            position: Triple {
                x: 19,
                y: 13,
                z: 30
            },
            velocity: Triple { x: -2, y: 1, z: -2 },
        },
        "19, 13, 30 @ -2,  1, -2".parse().unwrap()
    );
}

#[test]
fn error() {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Default, FromStr)]
    #[separator(',')]
    struct Triple {
        x: isize,
        y: isize,
        z: isize,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, FromStr)]
    #[separator('@')]
    pub struct Hail {
        position: Triple,
        velocity: Triple,
    }

    assert_eq!(
        "19, 13, 30".parse::<Hail>(),
        Err("Unexpected end of input while parsing velocity".to_string())
    );

    assert_eq!(
        "19, 13, 30 @".parse::<Hail>(),
        Err("Error while parsing `velocity`: Error while parsing `x`: cannot parse integer from empty string".to_string())
    );
}
