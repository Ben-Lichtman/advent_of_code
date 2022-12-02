use std::fmt::Display;

enum Play {
	R,
	P,
	S,
}

enum Res {
	Win,
	Draw,
	Loss,
}

fn translate1(i: &str) -> Play {
	match i {
		"A" | "X" => Play::R,
		"B" | "Y" => Play::P,
		"C" | "Z" => Play::S,
		_ => panic!(),
	}
}

fn translate2(i: &str) -> Res {
	match i {
		"X" => Res::Loss,
		"Y" => Res::Draw,
		"Z" => Res::Win,
		_ => panic!(),
	}
}

pub fn part1(i: &str) -> impl Display {
	i.split('\n')
		.map(|line| (translate1(&line[0..1]), translate1(&line[2..3])))
		.map(|(a, b)| match (a, b) {
			(Play::R, Play::R) => 1 + 3,
			(Play::R, Play::P) => 2 + 6,
			(Play::R, Play::S) => 3 + 0,
			(Play::P, Play::R) => 1 + 0,
			(Play::P, Play::P) => 2 + 3,
			(Play::P, Play::S) => 3 + 6,
			(Play::S, Play::R) => 1 + 6,
			(Play::S, Play::P) => 2 + 0,
			(Play::S, Play::S) => 3 + 3,
		})
		.sum::<i64>()
}

pub fn part2(i: &str) -> impl Display {
	i.split('\n')
		.map(|line| (translate1(&line[0..1]), translate2(&line[2..3])))
		.map(|(a, b)| match (a, b) {
			(Play::R, Res::Win) => 6 + 2,
			(Play::R, Res::Draw) => 3 + 1,
			(Play::R, Res::Loss) => 0 + 3,
			(Play::P, Res::Win) => 6 + 3,
			(Play::P, Res::Draw) => 3 + 2,
			(Play::P, Res::Loss) => 0 + 1,
			(Play::S, Res::Win) => 6 + 1,
			(Play::S, Res::Draw) => 3 + 3,
			(Play::S, Res::Loss) => 0 + 2,
		})
		.sum::<i64>()
}
