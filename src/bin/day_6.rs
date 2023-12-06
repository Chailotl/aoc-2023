use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	part_one();
	part_two();
}

fn part_one() {
	let file = File::open("src/bin/day_6.txt").unwrap();
	let reader = BufReader::new(file);

	let mut iter = reader.lines().flatten();
	let times = iter
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.map(|x| x.parse::<i32>().unwrap())
		.collect::<Vec<i32>>();
	let distances = iter
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.map(|x| x.parse::<i32>().unwrap())
		.collect::<Vec<i32>>();

	let mut sum = 1;

	for i in 0..times.len() {
		let dist = distances[i];
		let mut wins = 0;

		for t in 1..times[i] {
			let time = t * (times[i] - t);

			if time > dist {
				wins += 1;
			}
		}

		sum *= wins;
	}

	println!("Part One: {}", sum);
}

fn part_two() {
	let file = File::open("src/bin/day_6.txt").unwrap();
	let reader = BufReader::new(file);

	let mut iter = reader.lines().flatten();
	let time = iter
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.collect::<Vec<&str>>()
		.join("")
		.parse::<i64>()
		.unwrap();
	let dist = iter
		.next()
		.unwrap()
		.split_whitespace()
		.skip(1)
		.collect::<Vec<&str>>()
		.join("")
		.parse::<i64>()
		.unwrap();

	let mut sum = 0;

	for i in 1..time {
		let t = i * (time - i);

		if t > dist {
			sum += 1;
		}
	}

	println!("Part Two: {}", sum);
}
