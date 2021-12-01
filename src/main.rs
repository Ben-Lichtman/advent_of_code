#![feature(array_windows)]

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
				"2021-1-2": year2021::day1::part2_alt2,
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
		]
	}
}
