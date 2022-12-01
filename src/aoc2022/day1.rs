use std::{cmp::Reverse, fmt::Display};

pub fn part1(i: &str) -> impl Display {
	i.split("\n\n")
		.map(|elf| {
			elf.split('\n')
				.map(str::parse::<i64>)
				.map(Result::unwrap)
				.sum::<i64>()
		})
		.max()
		.unwrap()
}

pub fn part2(i: &str) -> impl Display {
	let mut v = i
		.split("\n\n")
		.map(|elf| {
			elf.split('\n')
				.map(str::parse::<i64>)
				.map(Result::unwrap)
				.sum::<i64>()
		})
		.collect::<Vec<_>>();
	v.sort_by_key(|&x| Reverse(x));
	v.iter().take(3).sum::<i64>()
}
