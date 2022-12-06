use std::{collections::HashSet, fmt::Display};

pub fn part1(i: &str) -> impl Display {
	i.as_bytes()
		.windows(4)
		.position(|w| HashSet::<u8>::from_iter(w.iter().copied()).len() == 4)
		.map(|n| n + 4)
		.unwrap()
}

pub fn part2(i: &str) -> impl Display {
	i.as_bytes()
		.windows(14)
		.position(|w| HashSet::<u8>::from_iter(w.iter().copied()).len() == 14)
		.map(|n| n + 14)
		.unwrap()
}
