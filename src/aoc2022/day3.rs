use std::{collections::HashSet, fmt::Display};

pub fn part1(i: &str) -> impl Display {
	i.split('\n')
		.map(|line| {
			let (a, b) = line.split_at(line.len() / 2);
			let (a, b) = (
				a.bytes()
					.map(|b| match b {
						b'a'..=b'z' => b - b'a' + 1,
						b'A'..=b'Z' => b - b'A' + 27,
						_ => panic!(),
					})
					.collect::<HashSet<_>>(),
				b.bytes()
					.map(|b| match b {
						b'a'..=b'z' => b - b'a' + 1,
						b'A'..=b'Z' => b - b'A' + 27,
						_ => panic!(),
					})
					.collect::<HashSet<_>>(),
			);
			let common = &a & &b;

			common.into_iter().next().unwrap() as i64
		})
		.sum::<i64>()
}

pub fn part2(i: &str) -> impl Display {
	i.split('\n')
		.array_chunks::<3>()
		.map(|line| {
			let (a, b, c) = (
				line[0]
					.bytes()
					.map(|b| match b {
						b'a'..=b'z' => b - b'a' + 1,
						b'A'..=b'Z' => b - b'A' + 27,
						_ => panic!(),
					})
					.collect::<HashSet<_>>(),
				line[1]
					.bytes()
					.map(|b| match b {
						b'a'..=b'z' => b - b'a' + 1,
						b'A'..=b'Z' => b - b'A' + 27,
						_ => panic!(),
					})
					.collect::<HashSet<_>>(),
				line[2]
					.bytes()
					.map(|b| match b {
						b'a'..=b'z' => b - b'a' + 1,
						b'A'..=b'Z' => b - b'A' + 27,
						_ => panic!(),
					})
					.collect::<HashSet<_>>(),
			);
			let common = &(&a & &b) & &c;

			common.into_iter().next().unwrap() as i64
		})
		.sum::<i64>()
}
