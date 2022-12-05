use std::{convert::identity, fmt::Display};

fn process_board(i: &str) -> Vec<Vec<char>> {
	let mut board = i
		.lines()
		.map(|line| {
			line.as_bytes()
				.chunks(4)
				.map(|chunk| chunk[1] as char)
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	board.pop();
	let width = board[0].len();
	let mut cols = vec![Vec::new(); width];
	for row in board.into_iter().rev() {
		row.into_iter()
			.enumerate()
			.filter(|&(_, c)| c != ' ')
			.for_each(|(i, c)| cols[i].push(c))
	}
	cols
}

pub fn get_from_to<T>(i: &mut [T], from: usize, to: usize) -> (&mut T, &mut T) {
	if from < to {
		let (left, right) = i.split_at_mut(to);
		(&mut left[from], &mut right[0])
	}
	else {
		let (left, right) = i.split_at_mut(from);
		(&mut right[0], &mut left[to])
	}
}

pub fn part1(i: &str) -> impl Display {
	let (starter, instructions) = i.split_once("\n\n").unwrap();

	let mut cols = process_board(starter);

	instructions
		.lines()
		.map(|l| {
			l.split(' ')
				.filter_map(|x| x.parse::<usize>().ok())
				.collect::<Vec<_>>()
				.try_into()
				.unwrap()
		})
		.map(identity::<[_; 3]>)
		.for_each(|[count, from, to]| {
			let (col_from, col_to) = get_from_to(&mut cols, from - 1, to - 1);

			let from_end = &col_from[col_from.len() - count..];
			col_to.extend(from_end.iter().rev());
			col_from.truncate(col_from.len() - count);
		});

	cols.iter().map(|c| c.last().unwrap()).collect::<String>()
}

pub fn part2(i: &str) -> impl Display {
	let (starter, instructions) = i.split_once("\n\n").unwrap();

	let mut cols = process_board(starter);

	instructions
		.lines()
		.map(|l| {
			l.split(' ')
				.filter_map(|x| x.parse::<usize>().ok())
				.collect::<Vec<_>>()
				.try_into()
				.unwrap()
		})
		.for_each(|[count, from, to]: [_; 3]| {
			let (col_from, col_to) = get_from_to(&mut cols, from - 1, to - 1);

			let from_end = &col_from[col_from.len() - count..];
			col_to.extend(from_end.iter());
			col_from.truncate(col_from.len() - count);
		});

	cols.iter().map(|c| c.last().unwrap()).collect::<String>()
}
