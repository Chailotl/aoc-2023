use itertools::Itertools;
use std::{
	cmp::Ordering,
	collections::HashMap,
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	part_one();
	part_two();
}

const RANKS: [char; 13] = [
	'2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn cmp_hand(a: &String, b: &String) -> Ordering {
	let mut a_type = hand_type(a);
	let mut b_type = hand_type(b);

	if a_type == b_type {
		for i in 0..5 {
			let a_char = RANKS
				.iter()
				.position(|&c| c == a.chars().nth(i).unwrap())
				.unwrap();
			let b_char = RANKS
				.iter()
				.position(|&c| c == b.chars().nth(i).unwrap())
				.unwrap();

			if a_char != b_char {
				a_type = a_char as i32;
				b_type = b_char as i32;
				break;
			}
		}
	}

	a_type.cmp(&b_type)
}

fn hand_type(hand: &String) -> i32 {
	let mut three = 0;
	let mut two = 0;

	for (_, count) in string_to_hashmap(hand) {
		if count == 5 {
			return 6;
		} else if count == 4 {
			return 5;
		} else if count == 3 {
			three += 1;
		} else if count == 2 {
			two += 1;
		}
	}

	if three == 1 && two == 1 {
		return 4;
	} else if three == 1 {
		return 3;
	} else if two == 2 {
		return 2;
	} else if two == 1 {
		return 1;
	}

	0
}

fn string_to_hashmap(hand: &String) -> HashMap<char, i32> {
	hand.chars().fold(HashMap::new(), |mut map, c| {
		*map.entry(c).or_insert(0) += 1;
		map
	})
}

fn part_one() {
	let file = File::open("src/bin/day_7.txt").unwrap();
	let reader = BufReader::new(file);

	let sum = reader
		.lines()
		.flatten()
		.map(|x| {
			let y = x.split_once(' ').unwrap();
			(y.0.to_string(), y.1.to_string())
		})
		.sorted_by(|a, b| cmp_hand(&a.0, &b.0))
		.map(|x| (2, x.1.parse::<i32>().unwrap()))
		.reduce(|acc, e| (acc.0 + 1, acc.1 + e.1 * acc.0))
		.unwrap()
		.1;

	println!("Part One: {}", sum);
}

fn part_two() {
	let file = File::open("src/bin/day_7.txt").unwrap();
	let reader = BufReader::new(file);

	let mut sum = 0;

	for line in reader.lines().flatten() {}

	println!("Part Two: {}", sum);
}
