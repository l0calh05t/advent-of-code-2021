use color_eyre::Result;
use ndarray::prelude::*;
use ndarray_stats::{interpolate::Midpoint, Quantile1dExt};
use std::{
	fs::File,
	io::{BufRead, BufReader},
	str::from_utf8,
};

fn read_integers(file_name: &str) -> Result<Vec<u16>> {
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	file.split(b',')
		.map(|line| Ok(from_utf8(&line?)?.trim().parse()?))
		.collect()
}

fn calculate_fuel(initial: &[u16], target: u16) -> i32 {
	initial
		.iter()
		.map(|&x| (x as i32 - target as i32).abs())
		.sum()
}

fn triangular_number(n: i32) -> i32 {
	(n * (n + 1)) / 2
}

fn calculate_fuel_triangular(initial: &[u16], target: u16) -> i32 {
	initial
		.iter()
		.map(|&x| triangular_number((x as i32 - target as i32).abs()))
		.sum()
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let mut input = read_integers("day-07/input")?;

	// for the L1 distance, the solution is simply the median
	let median =
		ArrayViewMut::from(&mut input).quantile_mut(0.5f64.try_into().unwrap(), &Midpoint)?;
	println!("{}", calculate_fuel(&input, median));

	// for the "triangular" distance, we just use the straightforward iterative solution
	let lo = *input.iter().min().unwrap();
	let hi = *input.iter().max().unwrap();
	let triangular_best = (lo..hi)
		.into_iter()
		.min_by_key(|&target| calculate_fuel_triangular(&input, target))
		.unwrap();
	println!("{}", calculate_fuel_triangular(&input, triangular_best));
	Ok(())
}
