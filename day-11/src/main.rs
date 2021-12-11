use color_eyre::Result;
use itertools::Itertools;
use ndarray::{prelude::*, ErrorKind::IncompatibleShape, IntoDimension, ShapeError};
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

fn update(state: &mut Array2<u8>, flash_map: &mut Array2<u8>) -> usize {
	let shape = state.raw_dim().into_dimension();
	let mut flash_count = 0;
	flash_map.fill(0);
	*state += 1;
	while {
		let mut new_flashes = 0;
		for (r, c) in ndarray::indices(shape) {
			if flash_map[[r, c]] != 0 || state[[r, c]] <= 9 {
				continue;
			}
			new_flashes += 1;
			flash_map[[r, c]] = 1;
			for r_off in [-1, 0, 1] {
				for c_off in [-1, 0, 1] {
					let r = r as isize + r_off;
					let c = c as isize + c_off;
					if !(0..shape[0] as isize).contains(&r) || !(0..shape[1] as isize).contains(&c)
					{
						continue;
					}
					state[[r as _, c as _]] += 1;
				}
			}
		}
		flash_count += new_flashes;
		new_flashes != 0
	} {}
	for (r, c) in ndarray::indices(shape) {
		if state[[r, c]] > 9 {
			state[[r, c]] = 0;
		}
	}
	flash_count
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let initial_state = read_input("day-11/input")?;
	let mut state = initial_state.clone();
	let mut flash_map = Array2::<u8>::zeros(state.raw_dim());
	let flashes: usize = (0..100).map(|_| update(&mut state, &mut flash_map)).sum();
	println!("{}", flashes);
	let mut state = initial_state;
	let synchronous_step = (1..).find(|_| {
		update(&mut state, &mut flash_map);
		flash_map.as_slice().unwrap().iter().all(|f| *f == 1)
	});
	println!("{}", synchronous_step.unwrap());
	Ok(())
}
