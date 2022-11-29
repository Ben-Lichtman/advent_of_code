#![feature(array_windows)]
#![feature(array_chunks)]
#![feature(portable_simd)]

use aoc_driver::*;

pub mod aoc2021;
pub mod helpers;

fn main() {
	let session = std::fs::read_to_string(".session.txt").unwrap();
	aoc_magic!(&session, 2021:3:1, aoc2021::day3::part1).unwrap();
}
