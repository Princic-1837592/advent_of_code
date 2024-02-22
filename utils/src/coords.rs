pub const CROSS_NEAR: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
pub const NEAR: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn iter(near: &[(isize, isize)], i: isize, j: isize) -> impl Iterator<Item=(isize, isize)> + '_ {
    near.iter().filter_map(move |(ni, nj)| {
        i.checked_add(*ni)
            .and_then(|i| j.checked_add(*nj).map(|j| (i, j)))
    })
}

pub fn iter_cross_near(i: isize, j: isize) -> impl Iterator<Item=(isize, isize)> {
    iter(&CROSS_NEAR, i, j)
}

pub fn iter_near(i: isize, j: isize) -> impl Iterator<Item=(isize, isize)> {
    iter(&NEAR, i, j)
}
