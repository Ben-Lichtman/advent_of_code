#![feature(bench_black_box)]
#![feature(array_windows)]
#![feature(array_chunks)]
#![feature(portable_simd)]

pub mod year2021;

fn main() {
	aoc_driver::aoc_complete! {
		session_file: ".session.txt"
		input_dir: "input"
		challenges: [
			{
				"2021-2-1": year2021::day2::part1,
				tests: [
					{
						name: "1",
						input: "
forward 5
down 5
forward 8
up 3
down 8
forward 2
						",
						output: "150",
					}
				]
			}
			{
				"2021-2-2": year2021::day2::part2,
			}
		]
	}
}
