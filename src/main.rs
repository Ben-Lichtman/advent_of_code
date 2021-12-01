pub mod year2021;

fn main() {
	aoc_driver::aoc_complete! {
		session_file: ".session.txt"
		input_dir: "input"
		challenges: [
			{
				"2021-1-1": year2021::day1::part1,
				tests: [

				]
			}
		]
	}
}
