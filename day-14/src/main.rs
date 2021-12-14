use color_eyre::Result;
use itertools::Itertools;
use std::{
	borrow::BorrowMut,
	collections::{hash_map::Entry, HashMap},
	fs::File,
	io::{BufRead, BufReader},
	mem::swap,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
	#[error("missing polymer template")]
	MissingTemplate,
	#[error("incorrect rule format")]
	IncorrectRuleFormat,
}

use Error::{IncorrectRuleFormat, MissingTemplate};

type Instructions = (String, Vec<((char, char), char)>);

fn read_input(file_name: &str) -> Result<Instructions> {
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	let mut lines = file.lines();
	let template = lines.borrow_mut().next().ok_or(MissingTemplate)??;
	let rules: Vec<_> = lines
		.filter_ok(|line| !line.is_empty())
		.map(|line| -> Result<_> {
			let line = line?;
			let (pair, insertion) = line
				.split(" -> ")
				.collect_tuple()
				.ok_or(IncorrectRuleFormat)?;
			let (a, b) = pair.chars().collect_tuple().ok_or(IncorrectRuleFormat)?;
			let (c,) = insertion
				.chars()
				.collect_tuple()
				.ok_or(IncorrectRuleFormat)?;
			Ok(((a, b), c))
		})
		.try_collect()?;
	Ok((template, rules))
}

fn print_info(polymer: &HashMap<(char, char), usize>, first: char, last: char) {
	let mut element_counts = HashMap::new();
	let mut set_or_add = |k: char, v: usize| {
		match element_counts.entry(k) {
			Entry::Occupied(mut entry) => *entry.get_mut() += v,
			Entry::Vacant(entry) => {
				entry.insert(v);
			}
		};
	};
	for (&pair, &count) in polymer {
		set_or_add(pair.0, count);
		set_or_add(pair.1, count);
	}
	let element_counts = element_counts
		.into_iter()
		.sorted_by_key(|(element, count)| (*count, *element))
		.collect_vec();
	let rare = element_counts.first().unwrap();
	let common = element_counts.last().unwrap();
	let map_count =
		|c: &(char, usize)| (c.1 + (c.0 == first) as usize + (c.0 == last) as usize) / 2;
	let rare = map_count(rare);
	let common = map_count(common);
	println!("{}", common - rare);
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let (template, rules) = read_input("day-14/input")?;

	let rules = HashMap::<(char, char), char>::from_iter(rules);

	let first = template.chars().next().unwrap();
	let last = template.chars().next_back().unwrap();

	let mut polymer = template.chars().tuple_windows().counts();
	let mut new_polymer = HashMap::new();
	for k in 0..40 {
		if k == 10 {
			print_info(&polymer, first, last);
		}
		new_polymer.clear();
		let mut set_or_add = |k: (char, char), v: usize| {
			match new_polymer.entry(k) {
				Entry::Occupied(mut entry) => *entry.get_mut() += v,
				Entry::Vacant(entry) => {
					entry.insert(v);
				}
			};
		};
		for (&pair, &count) in &polymer {
			if let Some(&insertion) = rules.get(&pair) {
				set_or_add((pair.0, insertion), count);
				set_or_add((insertion, pair.1), count);
			} else {
				set_or_add(pair, count);
			}
		}
		swap(&mut polymer, &mut new_polymer);
	}

	print_info(&polymer, first, last);

	Ok(())
}
