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

fn split_string(string: String) -> (String, String) {
	let split = string.split_once(' ').unwrap();
	(split.0.to_string(), split.1.to_string())
}

fn count_chars(hand: &String) -> HashMap<char, i32> {
	hand.chars().fold(HashMap::new(), |mut map, c| {
		*map.entry(c).or_insert(0) += 1;
		map
	})
}

fn cmp_hand(
	a: &String,
	b: &String,
	get_hand_type: &dyn Fn(&String) -> i32,
	get_card_rank: &dyn Fn(&String, usize) -> i32,
) -> Ordering {
	let a_type = get_hand_type(a);
	let b_type = get_hand_type(b);

	if a_type == b_type {
		for i in 0..5 {
			let ord = get_card_rank(a, i).cmp(&get_card_rank(b, i));

			if ord != Ordering::Equal {
				return ord;
			}
		}
	}

	a_type.cmp(&b_type)
}

fn match_structure(structure: &Vec<i32>) -> i32 {
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

fn part_one() {
	let file = File::open("src/bin/day_7.txt").unwrap();
	let reader = BufReader::new(file);

	let sum = reader
		.lines()
		.flatten()
		.map(|x| split_string(x))
		.sorted_by(|a, b| cmp_hand(&a.0, &b.0, &get_hand_type_1, &get_card_rank_1))
		.map(|x| (2, x.1.parse::<i32>().unwrap()))
		.reduce(|acc, e| (acc.0 + 1, acc.1 + e.1 * acc.0))
		.unwrap()
		.1;

	println!("Part One: {}", sum);
}

fn get_hand_type_1(hand: &String) -> i32 {
	match_structure(
		&count_chars(hand)
			.iter()
			.map(|x| *x.1)
			.sorted_by(|a, b| a.cmp(&b))
			.collect::<Vec<i32>>(),
	)
}

fn get_card_rank_1(hand: &String, index: usize) -> i32 {
	[
		'2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
	]
	.iter()
	.position(|&c| c == hand.chars().nth(index).unwrap())
	.unwrap() as i32
}

fn part_two() {
	let file = File::open("src/bin/day_7.txt").unwrap();
	let reader = BufReader::new(file);

	let sum = reader
		.lines()
		.flatten()
		.map(|x| split_string(x))
		.sorted_by(|a, b| cmp_hand(&a.0, &b.0, &get_hand_type_2, &get_card_rank_2))
		.map(|x| (2, x.1.parse::<i32>().unwrap()))
		.reduce(|acc, e| (acc.0 + 1, acc.1 + e.1 * acc.0))
		.unwrap()
		.1;

	println!("Part Two: {}", sum);
}

fn get_hand_type_2(hand: &String) -> i32 {
	let mut chars = count_chars(hand);

	let mut highest_rank = match_structure(
		&chars
			.iter()
			.map(|x| *x.1)
			.sorted_by(|a, b| a.cmp(&b))
			.collect::<Vec<i32>>(),
	);

	if let Some(joker) = chars.remove(&'J') {
		let structure = chars.iter().map(|x| *x.1).collect::<Vec<i32>>();

		for i in 0..structure.len() {
			let mut joker_struct = structure.clone();
			joker_struct[i] += joker;
			joker_struct.sort_by(|a, b| a.cmp(&b));

			let rank = match_structure(&joker_struct);

			if rank > highest_rank {
				highest_rank = rank;
			}
		}
	}

	highest_rank
}

fn get_card_rank_2(hand: &String, index: usize) -> i32 {
	[
		'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
	]
	.iter()
	.position(|&c| c == hand.chars().nth(index).unwrap())
	.unwrap() as i32
}
