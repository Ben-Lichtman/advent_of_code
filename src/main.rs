#![feature(array_windows)]
#![feature(array_chunks)]
#![feature(portable_simd)]

use aoc_driver::*;

pub mod aoc2021;
pub mod aoc2022;
pub mod helpers;

fn main() {
	let session = std::fs::read_to_string(".session.txt").unwrap();

	aoc_magic!(&session, 2022:1:2, aoc2022::day1::part2).unwrap();
}
