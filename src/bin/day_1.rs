use fancy_regex::Regex;
use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	part_one();
	part_two();
}

fn part_one() {
	let file = File::open("src/bin/day_1.txt").unwrap();
	let reader = BufReader::new(file);

	let mut sum = 0;

	for line in reader.lines().flatten() {
		let mut do_first = true;
		let (mut first, mut last) = ('0', '0');

		for char in line.chars() {
			if char.is_numeric() {
				if do_first {
					do_first = false;
					first = char;
				}

				last = char;
			}
		}

		sum += format!("{}{}", first, last).parse::<i32>().unwrap();
	}

	println!("Part One: {}", sum);
}

fn part_two() {
	let file = File::open("src/bin/day_1.txt").unwrap();
	let reader = BufReader::new(file);

	let mut sum = 0;

	for line in reader.lines().flatten() {
		let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|[1-9]))").unwrap();

		let results: Vec<&str> = re
			.captures_iter(line.as_str())
			.flatten()
			.map(|m| m.get(1).unwrap().as_str())
			.collect();

		sum += format!(
			"{}{}",
			parse_text(results[0]),
			parse_text(results[results.len() - 1])
		)
		.parse::<i32>()
		.unwrap();
	}

	println!("Part Two: {}", sum);
}

fn parse_text(text: &str) -> &str {
	match text {
		"one" => "1",
		"two" => "2",
		"three" => "3",
		"four" => "4",
		"five" => "5",
		"six" => "6",
		"seven" => "7",
		"eight" => "8",
		"nine" => "9",
		_ => text,
	}
}
