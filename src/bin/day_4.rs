use std::{
	collections::HashMap,
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	part_one();
	part_two();
}

fn part_one() {
	let file = File::open("src/bin/day_4.txt").unwrap();
	let reader = BufReader::new(file);

	let mut sum = 0;

	for line in reader.lines().flatten() {
		let numbers = line[10..].split_once('|').unwrap();
		let winning_nums: Vec<&str> = numbers.0.split_whitespace().collect();
		let our_nums: Vec<&str> = numbers.1.split_whitespace().collect();

		let mut points = 0;

		for win in winning_nums {
			if our_nums.contains(&win) {
				if points == 0 {
					points = 1;
				} else {
					points *= 2;
				}
			}
		}

		sum += points;
	}

	println!("Part One: {}", sum);
}

fn part_two() {
	let file = File::open("src/bin/day_4.txt").unwrap();
	let reader = BufReader::new(file);

	let mut sum = 0;

	let lines: Vec<String> = reader.lines().flatten().collect();
	let mut cards = HashMap::new();

	for i in 0..lines.len() {
		let copies = *cards.entry(i).or_insert(1);
		sum += copies;

		let numbers: (&str, &str) = lines[i][10..].split_once('|').unwrap();
		let winning_nums: Vec<&str> = numbers.0.split_whitespace().collect();
		let our_nums: Vec<&str> = numbers.1.split_whitespace().collect();

		let mut matches = 0;

		for win in winning_nums {
			if our_nums.contains(&win) {
				matches += 1;
			}
		}

		for card in 0..matches {
			*cards.entry(card + i + 1).or_insert(1) += copies;
		}
	}

	println!("Part Two: {}", sum);
}
