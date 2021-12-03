use bitvec::prelude::*;
use color_eyre::eyre::Result;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

const VALID_BITS: usize = 12;

fn read_bit_patterns(file_name: &str) -> Result<Vec<u16>> {
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	file.lines()
		.map(|line| Ok(u16::from_str_radix(&line?, 2)?))
		.collect()
}

fn determine_rating(mut bit_patterns: Vec<u16>, predicate: impl Fn(Ordering) -> bool) -> u16 {
	for position in (0..VALID_BITS).rev() {
		let count: usize = bit_patterns
			.iter()
			.map(|&bit_pattern| ((bit_pattern >> position) & 1) as usize)
			.sum();
		let complement = bit_patterns.len() - count;
		let mask = (predicate(count.cmp(&complement)) as u16) << position;
		bit_patterns = bit_patterns
			.iter()
			.copied()
			.filter(|&bit_pattern| (((bit_pattern ^ mask) >> position) & 1) == 0)
			.collect();
		if bit_patterns.len() == 1 {
			return bit_patterns[0];
		}
	}
	unreachable!();
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let bit_patterns = read_bit_patterns("input.03")?;
	let counts = bit_patterns
		.iter()
		.map(|bit_pattern| -> [usize; 16] {
			array_init::from_iter(
				bit_pattern
					.view_bits::<Lsb0>()
					.iter()
					.map(|bit| *bit as usize),
			)
			.unwrap()
		})
		.fold([0usize; 16], |a, b| {
			array_init::from_iter(a.iter().zip(b.iter()).map(|(a, b)| a + b)).unwrap()
		});
	let mut gamma: BitArray<Lsb0, u16> = BitArray::zeroed();
	counts
		.iter()
		.map(|count| *count > bit_patterns.len() / 2)
		.zip(gamma.iter_mut())
		.for_each(|(input, output)| output.set(input));
	let valid_bit_mask = (1 << VALID_BITS) - 1;
	let gamma = gamma.into_inner() & valid_bit_mask;
	let epsilon = !gamma & valid_bit_mask;
	println!("{}", gamma as u32 * epsilon as u32);
	let oxygen_generator_rating = determine_rating(bit_patterns.clone(), Ordering::is_ge);
	let co2_scrubber_rating = determine_rating(bit_patterns, Ordering::is_lt);
	println!(
		"{}",
		oxygen_generator_rating as u32 * co2_scrubber_rating as u32
	);
	Ok(())
}
