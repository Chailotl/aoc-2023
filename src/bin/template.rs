use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	part_one();
	part_two();
}

fn part_one() {
	let file = File::open("src/bin/day_0.txt").unwrap();
	let reader = BufReader::new(file);

	let mut sum = 0;

	for line in reader.lines().flatten() {}

	println!("Part One: {}", sum);
}

fn part_two() {
	let file = File::open("src/bin/day_0.txt").unwrap();
	let reader = BufReader::new(file);

	let mut sum = 0;

	for line in reader.lines().flatten() {}

	println!("Part Two: {}", sum);
}
