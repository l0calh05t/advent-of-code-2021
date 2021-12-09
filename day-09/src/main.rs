use color_eyre::Result;
use itertools::Itertools;
use ndarray::{prelude::*, ErrorKind::IncompatibleShape, ShapeError};
use std::{fs::File, io::Read};

fn read_input(file_name: &str) -> Result<Array2<u8>> {
	let mut bytes = Vec::new();
	File::open(file_name)?.read_to_end(&mut bytes)?;
	let mut lines = 0usize;
	let mut columns = None;
	let values = bytes
		.split(|&b| b == b'\n')
		.map(|line| {
			if line.is_empty() {
				return Ok(line);
			}
			lines += 1;
			if let Some(columns) = columns {
				if columns != line.len() {
					return Err(ShapeError::from_kind(IncompatibleShape).into());
				}
			}
			columns = Some(line.len());
			Ok(line)
		})
		.flatten_ok()
		.map_ok(|&b| b.checked_sub(b'0').unwrap())
		.collect::<Result<Vec<_>>>()?;
	Ok(Array2::from_shape_vec((lines, columns.unwrap()), values)?)
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let input = read_input("day-09/input")?;

	// pad ndarray to ensure the size of the window iterator matches the si
	let mut padded_input = Array2::from_elem(Ix2(input.shape()[0] + 2, input.shape()[1] + 2), 9);
	padded_input
		.slice_mut(s![
			1..padded_input.shape()[0] - 1,
			1..padded_input.shape()[1] - 1
		])
		.assign(&input);
	// use ndarray windows to iterate and find low spots
	let risk: usize = padded_input
		.windows((3, 3))
		.into_iter()
		.map(|window| {
			if window[[1, 1]] < window[[0, 1]]
				&& window[[1, 1]] < window[[2, 1]]
				&& window[[1, 1]] < window[[1, 0]]
				&& window[[1, 1]] < window[[1, 2]]
			{
				window[[1, 1]] as usize + 1
			} else {
				0
			}
		})
		.sum();
	println!("{}", risk);

	// same thing but standard iteration (no padding necessary)
	let mut markers = Array2::<u8>::zeros(input.raw_dim());
	let mut queue = Vec::new();
	let mut basin_sizes = Vec::new();
	let shape = input.shape();
	for idx in ndarray::indices(shape) {
		let r = idx[0];
		let c = idx[1];
		let cur = input[[r, c]];

		// inverted low spot condition for early-out
		if (r > 0 && input[[r - 1, c]] <= cur)
			|| (r < shape[0] - 1 && input[[r + 1, c]] <= cur)
			|| (c > 0 && input[[r, c - 1]] <= cur)
			|| (c < shape[1] - 1 && input[[r, c + 1]] <= cur)
		{
			continue;
		}

		// adapted flood-fill algorithm
		// this version uses a queue, often implemented recursively instead
		queue.push([r, c]);
		markers.fill(0);
		while let Some([r, c]) = queue.pop() {
			let cur = input[[r, c]];
			markers[[r, c]] = 1;
			if r > 0 && (cur + 1..9).contains(&input[[r - 1, c]]) {
				queue.push([r - 1, c]);
			}
			if r < shape[0] - 1 && (cur + 1..9).contains(&input[[r + 1, c]]) {
				queue.push([r + 1, c]);
			}
			if c > 0 && (cur + 1..9).contains(&input[[r, c - 1]]) {
				queue.push([r, c - 1]);
			}
			if c < shape[1] - 1 && (cur + 1..9).contains(&input[[r, c + 1]]) {
				queue.push([r, c + 1]);
			}
		}

		// compute and store size of basin
		basin_sizes.push(markers.mapv(|v| v as usize).sum());
	}

	// sort and print product of last three elements
	basin_sizes.sort_unstable();
	println!(
		"{}",
		basin_sizes[basin_sizes.len() - 3..]
			.iter()
			.product::<usize>()
	);

	Ok(())
}
