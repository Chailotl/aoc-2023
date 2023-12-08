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

fn split_string(string: String) -> (String, String) {
	let split = string.split_once(' ').unwrap();
	(split.0.to_string(), split.1.to_string())
}

fn cmp_hand(a: &String, b: &String) -> Ordering {
	let a_type = hand_type(a);
	let b_type = hand_type(b);

	if a_type == b_type {
		for i in 0..5 {
			let ord = get_rank(a, i).cmp(&get_rank(b, i));

			if ord != Ordering::Equal {
				return ord;
			}
		}
	}

	a_type.cmp(&b_type)
}

fn hand_type(hand: &String) -> i32 {
	let structure = string_to_hashmap(hand)
		.iter()
		.sorted_by(|a, b| a.1.cmp(&b.1))
		.map(|x| *x.1)
		.collect::<Vec<i32>>();

	match structure[..] {
		[5] => 6,
		[1, 4] => 5,
		[2, 3] => 4,
		[1, 1, 3] => 3,
		[1, 2, 2] => 2,
		[1, 1, 1, 2] => 1,
		_ => 0,
	}
}

fn string_to_hashmap(hand: &String) -> HashMap<char, i32> {
	hand.chars().fold(HashMap::new(), |mut map, c| {
		*map.entry(c).or_insert(0) += 1;
		map
	})
}

fn get_rank(hand: &String, index: usize) -> i32 {
	RANKS
		.iter()
		.position(|&c| c == hand.chars().nth(index).unwrap())
		.unwrap() as i32
}

fn part_one() {
	let file = File::open("src/bin/day_7.txt").unwrap();
	let reader = BufReader::new(file);

	let sum = reader
		.lines()
		.flatten()
		.map(|x| split_string(x))
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
