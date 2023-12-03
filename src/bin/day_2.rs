use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	part_one();
	part_two();
}

fn part_one() {
	let file = File::open("src/bin/day_2.txt").unwrap();
	let reader = BufReader::new(file);

	let mut sum = 0;

	for line in reader.lines().flatten() {
		let (mut red, mut green, mut blue) = (0, 0, 0);

		let (game_id, game) = line.split_once(':').unwrap();

		for hand in game.split(';').map(|s| s.trim()) {
			for cubes in hand.split(", ") {
				let (num, color) = cubes.split_once(' ').unwrap();

				let num = num.parse::<i32>().unwrap();

				match color {
					"red" => {
						if num > red {
							red = num;
						}
					}
					"green" => {
						if num > green {
							green = num;
						}
					}
					"blue" => {
						if num > blue {
							blue = num;
						}
					}
					_ => {}
				}
			}
		}

		if red <= 12 && green <= 13 && blue <= 14 {
			sum += game_id
				.split_ascii_whitespace()
				.skip(1)
				.next()
				.unwrap()
				.parse::<i32>()
				.unwrap();
		}
	}

	println!("Part One: {}", sum);
}

fn part_two() {
	let file = File::open("src/bin/day_2.txt").unwrap();
	let reader = BufReader::new(file);

	let mut sum = 0;

	for line in reader.lines().flatten() {
		let (mut red, mut green, mut blue) = (0, 0, 0);

		let game = line.split_once(':').unwrap().1;

		for hand in game.split(';').map(|s| s.trim()) {
			for cubes in hand.split(", ") {
				let (num, color) = cubes.split_once(' ').unwrap();

				let num = num.parse::<i32>().unwrap();

				match color {
					"red" => {
						if num > red {
							red = num;
						}
					}
					"green" => {
						if num > green {
							green = num;
						}
					}
					"blue" => {
						if num > blue {
							blue = num;
						}
					}
					_ => {}
				}
			}
		}

		sum += red * green * blue;
	}

	println!("Part Two: {}", sum);
}
