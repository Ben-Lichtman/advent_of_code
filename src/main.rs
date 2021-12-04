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
				"2021-3-2": year2021::day3::part2,

			}

		]
	}
}

/*
			{
				"2021-x-x": year2021::dayx::partx,
			}
*/
