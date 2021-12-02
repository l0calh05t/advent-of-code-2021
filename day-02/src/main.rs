use color_eyre::eyre::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
	#[error("incorrect command length")]
	SplitError,
	#[error("invalid direction \"{0}\"")]
	DirectionError(String),
}

use Error::{DirectionError, SplitError};

#[derive(Clone, Copy)]
enum Command {
	Forward(i32),
	Down(i32),
	Up(i32),
}

use Command::{Down, Forward, Up};

fn read_commands(file_name: &str) -> Result<Vec<Command>> {
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	file.lines()
		.map(|line| {
			let line = line?;
			let (direction, amount) = line.splitn(2, ' ').collect_tuple().ok_or(SplitError)?;
			let amount = amount.parse()?;
			Ok(match direction {
				"forward" => Forward(amount),
				"down" => Down(amount),
				"up" => Up(amount),
				_ => return Err(DirectionError(direction.to_owned()).into()),
			})
		})
		.collect()
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let commands = read_commands("input.02")?;
	let position = commands.iter().fold([0, 0], |mut position, command| {
		let [horizontal, depth] = &mut position;
		match *command {
			Forward(amount) => *horizontal += amount,
			Down(amount) => *depth += amount,
			Up(amount) => *depth -= amount,
		}
		position
	});
	println!("{}", position.iter().product::<i32>());
	let position_aim = commands.iter().fold([0, 0, 0], |mut position, command| {
		let [horizontal, depth, aim] = &mut position;
		match *command {
			Forward(amount) => {
				*horizontal += amount;
				*depth += *aim * amount;
			}
			Down(amount) => *aim += amount,
			Up(amount) => *aim -= amount,
		}
		position
	});
	println!("{}", position_aim[..2].iter().product::<i32>());
	Ok(())
}
