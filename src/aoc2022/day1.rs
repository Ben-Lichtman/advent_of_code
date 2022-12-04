use crate::util::sorted_arr::SortedArr;
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

pub fn part2_alt(i: &str) -> impl Display {
	let mut groups = i.split("\n\n").map(|elf| {
		elf.split('\n')
			.map(str::parse::<i64>)
			.map(Result::unwrap)
			.sum::<i64>()
	});
	let arr_3 = std::array::from_fn::<_, 3, _>(|_| groups.next().unwrap());
	let mut max_arr = SortedArr::from(arr_3, |&x| Reverse(x));
	groups.for_each(|g| max_arr.insert(g, |&x| Reverse(x)));
	max_arr.into_inner().iter().sum::<i64>()
}
