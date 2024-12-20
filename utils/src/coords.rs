use strum_macros::EnumIter;

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

fn iter(near: &[(isize, isize)], i: isize, j: isize) -> impl Iterator<Item = (isize, isize)> + '_ {
	near.iter().filter_map(move |(ni, nj)| {
		i.checked_add(*ni)
			.and_then(|i| j.checked_add(*nj).map(|j| (i, j)))
	})
}

pub fn iter_cross_near(i: isize, j: isize) -> impl Iterator<Item = (isize, isize)> {
	iter(&CROSS_NEAR, i, j)
}

pub fn iter_near(i: isize, j: isize) -> impl Iterator<Item = (isize, isize)> {
	iter(&NEAR, i, j)
}

pub fn u_iter_cross_near(
	i: usize,
	j: usize,
	width: usize,
	height: usize,
) -> impl Iterator<Item = (usize, usize)> {
	[
		(i + 1 < width).then(|| (i + 1, j)),
		(j + 1 < height).then(|| (i, j + 1)),
		(i >= 1).then(|| (i - 1, j)),
		(j >= 1).then(|| (i, j - 1)),
	]
	.into_iter()
	.flatten()
}

pub fn u_iter_near(
	i: usize,
	j: usize,
	width: usize,
	height: usize,
) -> impl Iterator<Item = (usize, usize)> {
	u_iter_cross_near(i, j, width, height).chain(
		[
			(i >= 1 && j >= 1).then(|| (i - 1, j - 1)),
			(i >= 1 && j + 1 < height).then(|| (i - 1, j + 1)),
			(i + 1 < width && j >= 1).then(|| (i + 1, j - 1)),
			(i + 1 < width && j + 1 < height).then(|| (i + 1, j + 1)),
		]
		.into_iter()
		.flatten(),
	)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, EnumIter)]
pub enum Direction {
	N,
	E,
	S,
	W,
}

impl Direction {
	pub const UP: Self = Self::E;
	pub const RIGHT: Self = Self::E;
	pub const DOWN: Self = Self::S;
	pub const LEFT: Self = Self::W;

	pub const fn opposite(&self) -> Self {
		match self {
			Self::N => Self::S,
			Self::E => Self::W,
			Self::S => Self::N,
			Self::W => Self::E,
		}
	}

	pub const fn next_i(&self, i: isize, j: isize) -> (isize, isize) {
		match self {
			Self::N => (i - 1, j),
			Self::E => (i, j + 1),
			Self::S => (i + 1, j),
			Self::W => (i, j - 1),
		}
	}

	pub const fn next_u(&self, i: usize, j: usize) -> (usize, usize) {
		match self {
			Self::N => (i.wrapping_sub(1), j),
			Self::E => (i, j + 1),
			Self::S => (i + 1, j),
			Self::W => (i, j.wrapping_sub(1)),
		}
	}
}
