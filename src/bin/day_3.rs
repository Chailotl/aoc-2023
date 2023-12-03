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
	let file = File::open("src/bin/day_3.txt").unwrap();
	let reader = BufReader::new(file);

	let mut grid = vec![vec!['.'; 0]; 0];

	for line in reader.lines().flatten() {
		let mut line_char: Vec<char> = line.chars().collect();
		line_char.push('.');
		grid.push(line_char);
	}

	let mut sum = 0;

	for y in 0..grid.len() {
		let mut number = "".to_string();
		let mut valid = false;

		for x in 0..grid[0].len() {
			let ch = grid[y][x];

			if ch.is_numeric() {
				number.push(ch);

				for x_delta in -1..=1 {
					for y_delta in -1..=1 {
						let x2 = x as i32 + x_delta;
						let y2 = y as i32 + y_delta;

						if x2 < 0 || y2 < 0 || x2 >= grid[0].len() as i32 || y2 >= grid.len() as i32
						{
							continue;
						}

						let symbol = grid[y2 as usize][x2 as usize];

						if !symbol.is_numeric() && symbol != '.' {
							valid = true;
						}
					}
				}
			} else if !number.is_empty() {
				if valid {
					sum += number.parse::<i32>().unwrap();
				}

				number.clear();
				valid = false;
			}
		}
	}

	println!("Part One: {}", sum);
}

fn part_two() {
	let file = File::open("src/bin/day_3.txt").unwrap();
	let reader = BufReader::new(file);

	let mut grid = vec![vec!['.'; 0]; 0];

	for line in reader.lines().flatten() {
		let mut line_char: Vec<char> = line.chars().collect();
		line_char.push('.');
		grid.push(line_char);
	}

	let mut parts = HashMap::new();

	for y in 0..grid.len() {
		let mut number = "".to_string();
		let mut valid = false;
		let mut location = "".to_string();

		for x in 0..grid[0].len() {
			let ch = grid[y][x];

			if ch.is_numeric() {
				number.push(ch);

				for x_delta in -1..=1 {
					for y_delta in -1..=1 {
						let x2 = x as i32 + x_delta;
						let y2 = y as i32 + y_delta;

						if x2 < 0 || y2 < 0 || x2 >= grid[0].len() as i32 || y2 >= grid.len() as i32
						{
							continue;
						}

						let symbol = grid[y2 as usize][x2 as usize];

						if symbol == '*' {
							valid = true;
							location = format!("{} {}", x2, y2);
						}
					}
				}
			} else if !number.is_empty() {
				if valid {
					parts
						.entry(location.clone())
						.or_insert(vec![])
						.push(number.parse::<i32>().unwrap());
				}

				number.clear();
				valid = false;
			}
		}
	}

	let mut sum = 0;

	for (_, numbers) in parts {
		if numbers.len() == 2 {
			sum += numbers[0] * numbers[1];
		}
	}

	println!("Part Two: {}", sum);
}
