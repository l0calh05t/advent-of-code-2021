use color_eyre::eyre::Result;
use itertools::{process_results, Itertools};
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("incorrect command length")]
	SplitError,
	#[error("invalid direction \"{0}\"")]
	DirectionError(String),
}

fn read_commands(file_name: &str) -> Result<()> {
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	let mut position = (0, 0);
	process_results(file.lines(), |lines| -> Result<_> {
		let (horizontal, depth) = &mut position;
		for line in lines {
			let (direction, amount) = line
				.splitn(2, ' ')
				.collect_tuple()
				.ok_or(Error::SplitError)?;
			let amount = amount.parse::<i32>()?;
			match direction {
				"forward" => *horizontal += amount,
				"down" => *depth += amount,
				"up" => *depth -= amount,
				_ => return Err(Error::DirectionError(direction.to_owned()).into()),
			}
		}
		println!("{:?}", position);
		println!("{}", position.0 * position.1);
		Ok(())
	})??;
	Ok(())
}

fn read_commands_2(file_name: &str) -> Result<()> {
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	let mut position = (0, 0, 0);
	process_results(file.lines(), |lines| -> Result<_> {
		let (aim, horizontal, depth) = &mut position;
		for line in lines {
			let (direction, amount) = line
				.splitn(2, ' ')
				.collect_tuple()
				.ok_or(Error::SplitError)?;
			let amount = amount.parse::<i32>()?;
			match direction {
				"forward" => {
					*horizontal += amount;
					*depth += *aim * amount;
				}
				"down" => *aim += amount,
				"up" => *aim -= amount,
				_ => return Err(Error::DirectionError(direction.to_owned()).into()),
			}
		}
		println!("{:?}", position);
		println!("{}", position.1 * position.2);
		Ok(())
	})??;
	Ok(())
}

fn main() -> Result<()> {
	color_eyre::install()?;
	read_commands("input.02")?;
	read_commands_2("input.02")?;
	Ok(())
}
