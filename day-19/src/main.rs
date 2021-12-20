use color_eyre::Result;
use itertools::Itertools;
use nalgebra::{Matrix3, Vector3};
use regex::Regex;
use std::{
	collections::{HashMap, HashSet},
	mem::{swap, take},
	time::Instant,
};
use thiserror::Error;

type Vec3 = Vector3<i32>;
type Mat3 = Matrix3<i32>;

#[derive(Error, Debug)]
enum Error {
	#[error("incorrect scanner position length")]
	ScannerLength,
	#[error("no input data")]
	NoInput,
	#[error("disconnected parts in input data")]
	SeparateParts,
}

fn parse_scanners() -> Result<Vec<Vec<Vec3>>> {
	let input = include_str!("../input");
	let separator = Regex::new(r"--- scanner \d+ ---")?;

	let mut result = Vec::new();
	let mut current = Vec::new();
	for line in input.lines().filter(|line| !line.is_empty()) {
		if separator.is_match(line) {
			if !current.is_empty() {
				result.push(take(&mut current));
			}
			continue;
		}

		let (x, y, z) = line
			.split(',')
			.map(str::parse::<i32>)
			.collect_tuple()
			.ok_or(Error::ScannerLength)?;
		let (x, y, z) = (x?, y?, z?);
		current.push(Vec3::new(x, y, z));
	}

	if !current.is_empty() {
		result.push(current);
	}

	Ok(result)
}

fn orientations() -> impl Iterator<Item = Mat3> {
	(0..3)
		.cartesian_product(0..3)
		.filter(|(a, b)| a != b)
		.cartesian_product([-1, 1].into_iter().cartesian_product([-1, 1].into_iter()))
		.map(|((ax1, ax2), (sgn1, sgn2))| {
			let a = sgn1 * Vec3::ith_axis(ax1).into_inner();
			let b = sgn2 * Vec3::ith_axis(ax2).into_inner();
			let c = a.cross(&b);
			Mat3::from_columns(&[a, b, c])
		})
}

fn main() -> Result<()> {
	color_eyre::install()?;

	let start = Instant::now();

	let mut scanners = parse_scanners()?;

	let mut scanner_positions = Vec::with_capacity(scanners.len());
	let mut beacons = HashSet::new();
	let mut front = Vec::new();
	let initial = scanners.pop().ok_or(Error::NoInput)?;
	beacons.extend(initial.iter().copied());
	front.push(initial);
	scanner_positions.push(Vec3::new(0, 0, 0));

	let mut unmatched_scanners = Vec::with_capacity(scanners.len());
	let mut counts: HashMap<Vec3, usize> = HashMap::with_capacity(beacons.len());
	while !scanners.is_empty() {
		let reference = front.pop().ok_or(Error::SeparateParts)?;

		'scanner: for mut scanner in scanners.drain(..) {
			for orientation in orientations() {
				let transformed = scanner.iter().map(|v| orientation * v);
				transformed
					.cartesian_product(reference.iter())
					.map(|(p_new, p_old)| p_new - p_old)
					.for_each(|d| {
						*counts.entry(d).or_default() += 1;
					});
				let best = counts
					.drain()
					.max_by_key(|(_, v)| *v)
					.ok_or(Error::NoInput)?;
				if best.1 < 12 {
					continue;
				}
				for beacon in &mut scanner {
					*beacon = orientation * *beacon - best.0;
				}
				beacons.extend(scanner.iter().copied());
				front.push(scanner);
				scanner_positions.push(-best.0);
				continue 'scanner;
			}
			unmatched_scanners.push(scanner);
		}

		swap(&mut scanners, &mut unmatched_scanners);
	}

	let maximum_distance = scanner_positions
		.iter()
		.tuple_combinations()
		.map(|(a, b)| (a - b).abs().sum())
		.max()
		.unwrap();

	let stop = Instant::now();

	println!("{}", beacons.len());
	println!("{}", maximum_distance);
	println!("{} ms", (stop - start).as_millis());

	Ok(())
}
