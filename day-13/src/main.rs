use color_eyre::Result;
use itertools::Itertools;
use std::{
	borrow::BorrowMut,
	fs::File,
	io::{BufRead, BufReader},
};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
	#[error("incorrect input (point has != 2 values)")]
	IncorrectPointLength,
	#[error("incorrect input (instruction has incorrect format)")]
	IncorrectInstructionFormat,
}

use Error::{IncorrectInstructionFormat, IncorrectPointLength};

enum Fold {
	Horizontal(usize),
	Vertical(usize),
}

use Fold::{Horizontal, Vertical};

type Instructions = (Vec<(usize, usize)>, Vec<Fold>);

fn read_input(file_name: &str) -> Result<Instructions> {
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	let mut lines = file.lines();
	let points = lines
		.borrow_mut()
		.take_while(|line| line.is_ok() && !line.as_ref().unwrap().is_empty())
		.map(|line| -> Result<_> {
			let line = line?;
			let (x, y) = line
				.split(',')
				.map(|e| e.parse::<usize>())
				.collect_tuple()
				.ok_or(IncorrectPointLength)?;
			Ok((x?, y?))
		})
		.collect::<Result<Vec<_>>>()?;
	let instructions = lines
		.map(|line| -> Result<_> {
			let line = line?;
			let (fold, along, value) = line
				.split_whitespace()
				.collect_tuple()
				.ok_or(IncorrectInstructionFormat)?;
			if fold != "fold" || along != "along" {
				return Err(IncorrectInstructionFormat.into());
			}
			let (direction, amount) = value
				.split('=')
				.collect_tuple()
				.ok_or(IncorrectInstructionFormat)?;
			let amount = amount.parse::<usize>()?;
			match direction {
				"x" => Ok(Horizontal(amount)),
				"y" => Ok(Vertical(amount)),
				_ => Err(IncorrectInstructionFormat.into()),
			}
		})
		.collect::<Result<Vec<_>>>()?;
	Ok((points, instructions))
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let (mut points, instructions) = read_input("day-13/input")?;
	for instruction in instructions {
		match instruction {
			Horizontal(x) => {
				for point in &mut points {
					if point.0 >= x {
						point.0 = x - (point.0 - x)
					}
				}
			}
			Vertical(y) => {
				for point in &mut points {
					if point.1 >= y {
						point.1 = y - (point.1 - y)
					}
				}
			}
		}
		points.sort_unstable();
		points.dedup();
		println!("{}", points.len());
	}
	points.sort_by_key(|point| (point.1, point.0));
	let mut x = 0;
	let mut y = 0;
	for point in points {
		while y < point.1 {
			println!();
			y += 1;
			x = 0;
		}
		while x < point.0 {
			print!(" ");
			x += 1;
		}
		print!("█");
		x += 1;
	}
	Ok(())
}
