use std::fmt::Display;

fn parse_input<const N: usize>(input: &str) -> Vec<[bool; N]> {
	input
		.lines()
		.map(str::trim)
		.filter(|l| l.len() == N)
		.map(|l| {
			let mut arr = [false; N];
			for (i, b) in arr.iter_mut().enumerate() {
				*b = match l.as_bytes()[i] {
					b'0' => false,
					b'1' => true,
					_ => panic!("oops"),
				}
			}
			arr
		})
		.collect::<Vec<_>>()
}

#[inline(never)]
pub fn part1(input: &str) -> impl Display {
	const SIZE: usize = 12;
	let numbers = parse_input::<SIZE>(input);
	let mut counters = [(0i64, 0i64); SIZE];
	for arr in &numbers {
		arr.iter()
			.zip(counters.iter_mut())
			.for_each(|(x, c)| match x {
				false => c.0 += 1,
				true => c.1 += 1,
			})
	}
	println!("{:?}", counters);
	let mut gamma = 0;
	let mut epsilon = 0;
	counters
		.iter()
		.for_each(|(falses, trues)| match trues > falses {
			true => {
				gamma = (gamma << 1) | 1;
				epsilon <<= 1;
			}
			false => {
				gamma <<= 1;
				epsilon = (epsilon << 1) | 1;
			}
		});

	println!("{:12b}", gamma);
	println!("{:12b}", epsilon);

	gamma * epsilon
}

#[inline(never)]
pub fn part2(input: &str) -> impl Display {
	const SIZE: usize = 12;
	let numbers = parse_input::<SIZE>(input);

	assert!(numbers.len() > 1);

	let mut ox = numbers.clone();
	let mut bit = 0;
	while ox.len() > 1 {
		let (mut falses, mut trues) = (0, 0);
		for arr in &ox {
			match arr[bit] {
				false => falses += 1,
				true => trues += 1,
			}
		}
		match trues >= falses {
			true => {
				ox.retain(|num| num[bit]);
			}
			false => {
				ox.retain(|num| !num[bit]);
			}
		}
		bit += 1;
	}

	let mut ox_out = 0;
	for b in ox[0] {
		ox_out = (ox_out << 1) | if b { 1 } else { 0 };
	}

	let mut co = numbers;
	let mut bit = 0;
	while co.len() > 1 {
		let (mut falses, mut trues) = (0, 0);
		for arr in &co {
			match arr[bit] {
				false => falses += 1,
				true => trues += 1,
			}
		}
		match falses > trues {
			true => {
				co.retain(|num| num[bit]);
			}
			false => {
				co.retain(|num| !num[bit]);
			}
		}
		bit += 1;
	}

	let mut co_out = 0;
	for b in co[0] {
		co_out = (co_out << 1) | if b { 1 } else { 0 };
	}

	ox_out * co_out
}
