use std::{convert::identity, fmt::Display};

pub fn part1(i: &str) -> impl Display {
	i.lines()
		.map(|l| {
			l.split(['-', ','])
				.filter_map(|x| x.parse::<i64>().ok())
				.collect::<Vec<_>>()
		})
		.map(|v| v.try_into().unwrap())
		.map(identity::<[_; 4]>)
		.filter(|[a, b, c, d]| (a <= c && d <= b) || (c <= a && b <= d))
		.count()
}

pub fn part2(i: &str) -> impl Display {
	i.lines()
		.map(|l| {
			l.split(['-', ','])
				.filter_map(|x| x.parse::<i64>().ok())
				.collect::<Vec<_>>()
		})
		.map(|v| v.try_into().unwrap())
		.map(identity::<[_; 4]>)
		.filter(|[a, b, c, d]| (a..=b).contains(&c) || (c..=d).contains(&a))
		.count()
}
