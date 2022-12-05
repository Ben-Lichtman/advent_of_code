#![feature(array_windows)]
#![feature(array_chunks)]
#![feature(portable_simd)]
#![feature(iter_array_chunks)]
#![allow(clippy::identity_op)]
#![allow(clippy::map_identity)]

use aoc_driver::*;

pub mod aoc2021;
pub mod aoc2022;
pub mod util;

fn main() {
	let session = std::fs::read_to_string(".session.txt").unwrap();

	aoc_magic!(&session, 2022:1:1, aoc2022::day1::part1).expect("==> 2022:1:1");
	aoc_magic!(&session, 2022:1:2, aoc2022::day1::part2).expect("==> 2022:1:2");

	aoc_magic!(&session, 2022:2:1, aoc2022::day2::part1).expect("==> 2022:2:1");
	aoc_magic!(&session, 2022:2:2, aoc2022::day2::part2).expect("==> 2022:2:2");

	aoc_magic!(&session, 2022:3:1, aoc2022::day3::part1).expect("==> 2022:3:1");
	aoc_magic!(&session, 2022:3:2, aoc2022::day3::part2).expect("==> 2022:3:2");

	aoc_magic!(&session, 2022:4:1, aoc2022::day4::part1).expect("==> 2022:4:1");
	aoc_magic!(&session, 2022:4:2, aoc2022::day4::part2).expect("==> 2022:4:2");

	aoc_magic!(&session, 2022:5:1, aoc2022::day5::part1).expect("==> 2022:5:1");
	aoc_magic!(&session, 2022:5:2, aoc2022::day5::part2).expect("==> 2022:5:2");

	// let test_input = std::fs::read_to_string("inputs/2022/4.txt").unwrap();
	// let ans = aoc2022::day4::part2(&test_input);
	// println!("{}", ans);
}
