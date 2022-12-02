#![feature(array_windows)]
#![feature(array_chunks)]
#![feature(portable_simd)]

use aoc_driver::*;

pub mod aoc2021;
pub mod aoc2022;
pub mod util;

fn main() {
	let session = std::fs::read_to_string(".session.txt").unwrap();

	// let ans = aoc2022::day2::part1(&std::fs::read_to_string("inputs/2022/2.txt").unwrap());
	// println!("{}", ans);

	aoc_magic!(&session, 2022:2:2, aoc2022::day2::part2).unwrap();
}
