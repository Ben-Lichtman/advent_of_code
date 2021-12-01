#![feature(bench_black_box)]
#![feature(array_windows)]
#![feature(array_chunks)]
#![feature(portable_simd)]

use std::{
	fs::read_to_string,
	hint::black_box,
	simd::{isizex2, isizex4, masksizex4, usizex2, usizex8},
	time::{Duration, Instant},
};

pub mod year2021;

fn main() {
	aoc_driver::aoc_complete! {
		session_file: ".session.txt"
		input_dir: "input"
		challenges: [
			{
				"2021-1-1": year2021::day1::part1,
				tests: [
					{
						name: "1",
						input: "199
						200
						208
						210
						200
						207
						240
						269
						260
						263
						",
						output: "7",
					}
				]
			}
			{
				"2021-1-2": year2021::day1::part2_simd_w,
				tests: [
					{
						name: "1",
						input: "199
						200
						208
						210
						200
						207
						240
						269
						260
						263
						",
						output: "5",
					}
				]
			}
			{
				"2021-1-2": year2021::day1::part2_simd,
				tests: [
				]
			}
		]
	}
}
