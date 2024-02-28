use std::fmt::Debug;

use struct_parser::FromLine;

#[test]
fn test() {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, FromLine)]
    #[separator(',')]
    pub struct Test1 {
        x: usize,
        y: usize,
    }

    assert_eq!(Test1 { x: 3, y: 4 }, "3, 4".into());

    #[derive(Copy, Clone, Debug, Eq, PartialEq, FromLine)]
    #[separator("--")]
    pub struct Test2 {
        x: usize,
        y: usize,
    }

    assert_eq!(Test2 { x: 3, y: 4 }, "3 -- 4".into());

    #[derive(Copy, Clone, Debug, Eq, PartialEq, FromLine)]
    pub struct Test3 {
        x: usize,
        y: usize,
    }

    assert_eq!(Test3 { x: 3, y: 4 }, "3   4".into());
    assert_eq!(Test3 { x: 3, y: 4 }, "3 4".into());
}

#[test]
fn advent() {
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Default, FromLine)]
    #[separator(',')]
    struct Triple {
        x: isize,
        y: isize,
        z: isize,
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq, FromLine)]
    #[separator('@')]
    pub struct Hail {
        #[into]
        position: Triple,
        #[into]
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
        "19, 13, 30 @ -2,  1, -2".into()
    );
}
