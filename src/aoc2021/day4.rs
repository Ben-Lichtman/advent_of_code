use std::{collections::HashMap, fmt::Display, hash::Hash, str::FromStr};

fn parse_input<T: FromStr + Eq + Hash>(input: &str) -> (Vec<T>, Vec<HashMap<T, (u8, u8)>>) {
	let mut segments = input.split("\n\n");
	let draws = segments.next().unwrap();
	let nums = draws
		.split(',')
		.flat_map(str::parse::<T>)
		.collect::<Vec<_>>();

	let boards = segments
		.map(|s| {
			s.split_whitespace()
				.flat_map(str::parse::<T>)
				.enumerate()
				.map(|(n, num)| (num, ((n % 5) as u8, (n / 5) as u8)))
				.collect::<HashMap<_, _>>()
		})
		.collect::<Vec<_>>();

	(nums, boards)
}

#[inline(never)]
pub fn part1(input: &str) -> impl Display {
	let (draws, boards) = parse_input::<i64>(input);

	let num_boards = boards.len();
	let mut marks = vec![[[false; 5]; 5]; num_boards];

	let mut last_draw = 0;
	let mut last_i = 0;

	'outer: for draw in draws {
		last_draw = draw;
		for i in 0..num_boards {
			last_i = i;
			if let Some((x, y)) = boards[i].get(&draw) {
				marks[i][*y as usize][*x as usize] = true;
				// Check row
				let row_finished = marks[i][*y as usize].iter().all(|&b| b);
				let col_finished = marks[i].iter().all(|row| row[*x as usize]);
				if row_finished || col_finished {
					break 'outer;
				}
			}
		}
	}

	let winning_board_value = boards[last_i]
		.iter()
		.map(|(val, (x, y))| {
			if !marks[last_i][*y as usize][*x as usize] {
				*val
			}
			else {
				0
			}
		})
		.sum::<i64>();

	winning_board_value * last_draw
}

#[inline(never)]
pub fn part2(input: &str) -> impl Display {
	let (draws, boards) = parse_input::<i64>(input);

	let num_boards = boards.len();
	let mut finished = vec![false; num_boards];
	let mut marks = vec![[[false; 5]; 5]; num_boards];

	let mut last_draw = 0;
	let mut last_i = 0;

	for draw in draws {
		if finished.iter().all(|b| *b) {
			break;
		}
		last_draw = draw;
		for i in 0..num_boards {
			if finished[i] {
				continue;
			}
			last_i = i;
			if let Some((x, y)) = boards[i].get(&draw) {
				marks[i][*y as usize][*x as usize] = true;
				// Check row
				let row_finished = marks[i][*y as usize].iter().all(|&b| b);
				let col_finished = marks[i].iter().all(|row| row[*x as usize]);
				if row_finished || col_finished {
					finished[i] = true;
				}
			}
		}
	}

	let winning_board_value = boards[last_i]
		.iter()
		.map(|(val, (x, y))| {
			if !marks[last_i][*y as usize][*x as usize] {
				*val
			}
			else {
				0
			}
		})
		.sum::<i64>();

	winning_board_value * last_draw
}
