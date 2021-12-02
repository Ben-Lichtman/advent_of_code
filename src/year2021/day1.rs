use std::{fmt::Display, simd::i64x8, str::FromStr};

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

#[inline(never)]
pub fn part2_simd_c(input: &str) -> impl Display {
	let mut numbers = parse_input::<i64>(input);

	numbers.resize(numbers.len() + 7, 0);

	-numbers
		.array_chunks::<8>()
		.map(|a| i64x8::from_array(*a))
		.zip(
			numbers[3..]
				.array_chunks::<8>()
				.map(|a| i64x8::from_array(*a)),
		)
		.map(|(a, b)| b.lanes_gt(a).to_int())
		.sum::<i64x8>()
		.horizontal_sum()
}

#[inline(never)]
pub fn part2_simd_w(input: &str) -> impl Display {
	let mut numbers = parse_input::<i64>(input);

	numbers.resize(numbers.len() + 7, 0);

	-numbers
		.windows(11)
		.step_by(8)
		.map(|arr| {
			let a = i64x8::from_slice(&arr[..8]);
			let b = i64x8::from_slice(&arr[3..]);
			b.lanes_gt(a).to_int()
		})
		.sum::<i64x8>()
		.horizontal_sum()
}
