use color_eyre::Result;
use itertools::Itertools;
use std::{
	borrow::BorrowMut,
	collections::HashMap,
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
	for (&pair, &count) in polymer {
		*element_counts.entry(pair.0).or_default() += count;
		*element_counts.entry(pair.1).or_default() += count;
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
		for (&pair, &count) in &polymer {
			if let Some(&insertion) = rules.get(&pair) {
				*new_polymer.entry((pair.0, insertion)).or_default() += count;
				*new_polymer.entry((insertion, pair.1)).or_default() += count;
			} else {
				*new_polymer.entry(pair).or_default() += count;
			}
		}
		swap(&mut polymer, &mut new_polymer);
	}

	print_info(&polymer, first, last);

	Ok(())
}
