use std::{fmt::Display, str::FromStr};

fn parse_input<T: FromStr>(input: &str) -> Vec<T> {
	input
		.lines()
		.map(str::trim)
		.flat_map(str::parse::<T>)
		.collect::<Vec<_>>()
}

#[inline(never)]
pub fn part1(input: &str) -> impl Display {
	let numbers = parse_input::<i64>(input);

	numbers.array_windows::<2>().filter(|[a, b]| b > a).count()
}

#[inline(never)]
pub fn part2(input: &str) -> impl Display {
	let numbers = parse_input::<i64>(input);

	let totals = numbers
		.array_windows::<3>()
		.map(|[a, b, c]| a + b + c)
		.collect::<Vec<_>>();

	totals.array_windows::<2>().filter(|[a, b]| b > a).count()
}

#[inline(never)]
pub fn part2_alt1(input: &str) -> impl Display {
	let numbers = parse_input::<i64>(input);

	numbers
		.array_windows::<4>()
		.filter(|[head, .., tail]| tail > head)
		.count()
}

#[inline(never)]
pub fn part2_alt2(input: &str) -> impl Display {
	let numbers = parse_input::<i64>(input);

	numbers
		.iter()
		.zip(numbers.iter().skip(3))
		.filter(|(a, b)| b > a)
		.count()
}
