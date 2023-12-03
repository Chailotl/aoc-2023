use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	part_one();
	part_two();
}

fn part_one() {
	let file = File::open("src/bin/day_3.txt").unwrap();
	let reader = BufReader::new(file);

	println!("Part One: {}", 0);
}

fn part_two() {
	let file = File::open("src/bin/day_3.txt").unwrap();
	let reader = BufReader::new(file);

	println!("Part Two: {}", 0);
}
