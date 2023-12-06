use core::fmt;
use std::{
	fs::File,
	io::{BufRead, BufReader},
};

fn main() {
	part_one();
	part_two();
}

#[derive(Debug, Clone)]
struct Mapping {
	start: i64,
	length: i64,
	offset: i64,
}

impl Mapping {
	fn end(&self) -> i64 {
		self.start + self.length - 1
	}
}

fn part_one() {
	let file = File::open("src/bin/day_5.txt").unwrap();
	let reader = BufReader::new(file);

	let mut seeds = vec![];
	let mut current_mapping = 0;
	let mut mappings = vec![vec![]; 7];

	for line in reader.lines().flatten() {
		if line.starts_with("seeds:") {
			seeds = line
				.split_once(':')
				.unwrap()
				.1
				.split_whitespace()
				.map(|s| s.parse::<i64>().unwrap())
				.collect();
		} else {
			match &line[..] {
				"seed-to-soil map:" => {
					current_mapping = 0;
				}
				"soil-to-fertilizer map:" => {
					current_mapping = 1;
				}
				"fertilizer-to-water map:" => {
					current_mapping = 2;
				}
				"water-to-light map:" => {
					current_mapping = 3;
				}
				"light-to-temperature map:" => {
					current_mapping = 4;
				}
				"temperature-to-humidity map:" => {
					current_mapping = 5;
				}
				"humidity-to-location map:" => {
					current_mapping = 6;
				}
				"" => {}
				_ => {
					let split: Vec<i64> = line
						.split_whitespace()
						.map(|s| s.parse::<i64>().unwrap())
						.collect();

					mappings[current_mapping].push(Mapping {
						start: split[1],
						length: split[2],
						offset: split[0] - split[1],
					});
				}
			}
		}
	}

	let mut closest = i64::MAX;

	for mut seed in seeds {
		for i in 0..=6 {
			for mapping in &mappings[i] {
				let start = mapping.start;
				let end = mapping.end();

				if seed >= start && seed <= end {
					seed += mapping.offset;
					break;
				}
			}
		}

		if seed < closest {
			closest = seed;
		}
	}

	println!("Part One: {}", closest);
}

#[derive(Debug, Clone)]
struct Interval {
	start: i64,
	length: i64,
}

impl Interval {
	fn new(start: i64, length: i64) -> Self {
		Self { start, length }
	}

	fn end(&self) -> i64 {
		self.start + self.length - 1
	}

	fn split(&self, at: i64) -> Option<(Self, Self)> {
		if at <= self.start || at >= self.end() {
			return None;
		}

		let length_a = at - self.start;
		let length_b = self.length - length_a;
		let start_a = self.start;
		let start_b = self.start + length_a;

		Some((Self::new(start_a, length_a), Self::new(start_b, length_b)))
	}
}

impl fmt::Display for Interval {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[{}, {}]", self.start, self.length)
	}
}

struct List(Vec<Interval>);

impl fmt::Display for List {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let vec = &self.0;
		write!(f, "[")?;

		for (count, v) in vec.iter().enumerate() {
			if count != 0 {
				write!(f, ", ")?;
			}
			write!(f, "{}", v)?;
		}

		write!(f, "]")
	}
}

fn part_two() {
	let file = File::open("src/bin/day_5.txt").unwrap();
	let reader = BufReader::new(file);

	let mut seeds = vec![];
	let mut current_mapping = 0;
	let mut mappings = vec![vec![]; 7];

	for line in reader.lines().flatten() {
		if line.starts_with("seeds:") {
			let mut iter = line
				.split_once(':')
				.unwrap()
				.1
				.split_whitespace()
				.map(|s| s.parse::<i64>().unwrap());

			let size = iter.clone().collect::<Vec<i64>>().len() / 2;

			for _ in 0..size {
				seeds.push(Interval {
					start: iter.next().unwrap(),
					length: iter.next().unwrap(),
				});
			}
		} else {
			match &line[..] {
				"seed-to-soil map:" => {
					current_mapping = 0;
				}
				"soil-to-fertilizer map:" => {
					current_mapping = 1;
				}
				"fertilizer-to-water map:" => {
					current_mapping = 2;
				}
				"water-to-light map:" => {
					current_mapping = 3;
				}
				"light-to-temperature map:" => {
					current_mapping = 4;
				}
				"temperature-to-humidity map:" => {
					current_mapping = 5;
				}
				"humidity-to-location map:" => {
					current_mapping = 6;
				}
				"" => {}
				_ => {
					let split: Vec<i64> = line
						.split_whitespace()
						.map(|s| s.parse::<i64>().unwrap())
						.collect();

					mappings[current_mapping].push(Mapping {
						start: split[1],
						length: split[2],
						offset: split[0] - split[1],
					});
				}
			}
		}
	}

	for i in 0..7 {
		let mut cut_seeds = vec![];

		for mut seed in seeds {
			let mut cuts: Vec<i64> = mappings[i]
				.iter()
				.flat_map(|x| [x.start, x.end() + 1])
				.collect();
			cuts.sort_by(|a, b| a.cmp(&b));

			for cut in cuts {
				if let Some(fragments) = seed.split(cut) {
					cut_seeds.push(fragments.0);
					seed = fragments.1;
				}
			}

			cut_seeds.push(seed);
		}

		let mut mapped_seeds = vec![];

		for mut seed in cut_seeds {
			for mapping in &mappings[i] {
				if seed.start >= mapping.start && seed.end() <= mapping.end() {
					seed.start += mapping.offset;
					break;
				}
			}

			mapped_seeds.push(seed);
		}

		seeds = mapped_seeds;
		//println!("{} -> {}", before, seeds.len());
		//println!("{}", List(seeds.clone()));
	}

	let mut closest = i64::MAX;

	for seed in seeds {
		if seed.start < closest {
			closest = seed.start;
		}
	}

	println!("Part Two: {}", closest);
}
