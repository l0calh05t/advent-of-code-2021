use color_eyre::eyre::Result;
use itertools::Itertools;
use ndarray::{s, Array2};
use std::fs::File;
use std::io::{BufRead, BufReader};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
	#[error("incorrect input length")]
	IncorrectLength,
}

use Error::IncorrectLength;

#[derive(Debug)]
struct Line {
	start: (usize, usize),
	stop: (usize, usize),
}

fn read_lines(file_name: &str) -> Result<Vec<Line>> {
	let file = File::open(file_name)?;
	let mut file = BufReader::new(file);
	let mut line = String::new();
	let mut lines = Vec::new();
	while file.read_line(&mut line)? != 0 {
		let (start_x, start_y, stop_x, stop_y) = line
			.trim_end()
			.split(" -> ")
			.map(|point| point.split(','))
			.flatten()
			.map(|value| <Result<usize>>::Ok(str::parse::<usize>(value)?))
			.collect_tuple()
			.ok_or(IncorrectLength)?;
		lines.push(Line {
			start: (start_x?, start_y?),
			stop: (stop_x?, stop_y?),
		});
		// read_line appends. clear manually
		line.clear();
	}
	Ok(lines)
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let lines = read_lines("day-05/input")?;
	let mut shape = (0, 0);
	for line in &lines {
		shape.0 = shape.0.max(line.start.0).max(line.stop.0);
		shape.1 = shape.1.max(line.start.1).max(line.stop.1);
	}
	shape.0 += 1;
	shape.1 += 1;
	let mut map = Array2::<u64>::zeros(shape);
	for line in &lines {
		let start = (line.start.0.min(line.stop.0), line.start.1.min(line.stop.1));
		let stop = (line.start.0.max(line.stop.0), line.start.1.max(line.stop.1));
		if start.0 == stop.0 {
			let mut slice = map.slice_mut(s![start.0, start.1..=stop.1]);
			slice += 1;
		} else if start.1 == stop.1 {
			let mut slice = map.slice_mut(s![start.0..=stop.0, start.1]);
			slice += 1;
		}
	}
	// println!("{}", map.t());
	println!("{}", map.iter().filter(|&&count| count >= 2).count());
	for line in &lines {
		let Line { start, stop } = line;
		if start.0 != stop.0 && start.1 != stop.1 {
			let mut cur = (start.0 as isize, start.1 as isize);
			let step = (
				if start.0 < stop.0 { 1 } else { -1 },
				if start.1 < stop.1 { 1 } else { -1 },
			);
			loop {
				map[[cur.0 as _, cur.1 as _]] += 1;
				if cur.0 == stop.0 as _ {
					break;
				}
				cur.0 += step.0;
				cur.1 += step.1;
			}
		}
	}
	// println!("{}", map.t());
	println!("{}", map.iter().filter(|&&count| count >= 2).count());
	Ok(())
}
