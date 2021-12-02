use std::fmt::Display;

fn parse_input(input: &str) -> Vec<(&str, i64)> {
	input
		.lines()
		.map(str::split_whitespace)
		.filter_map(|mut l| Some((l.next()?, l.next()?)))
		.filter_map(|(w, n)| Some((w, n.parse::<i64>().ok()?)))
		.collect::<Vec<_>>()
}

#[inline(never)]
pub fn part1(input: &str) -> impl Display {
	let inputs = parse_input(input);

	let sum = inputs
		.into_iter()
		.fold((0, 0), |(x, y), (word, num)| match word {
			"forward" => (x + num, y),
			"up" => (x, y - num),
			"down" => (x, y + num),
			_ => panic!(),
		});

	sum.0 * sum.1
}

#[inline(never)]
pub fn part2(input: &str) -> impl Display {
	let inputs = parse_input(input);

	let sum = inputs
		.into_iter()
		.fold((0, 0, 0), |(h, d, aim), (word, num)| match word {
			"forward" => (h + num, d + aim * num, aim),
			"up" => (h, d, aim - num),
			"down" => (h, d, aim + num),
			_ => panic!(),
		});

	sum.0 * sum.1
}
