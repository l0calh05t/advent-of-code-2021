use ndarray::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

const INPUT: &[u8] = include_bytes!("../input");

fn get_input() -> Array2<u8> {
	let mut lines = 0usize;
	let mut columns = None;
	let values: Vec<_> = INPUT
		.split(|&b| b == b'\n')
		.filter(|line| !line.is_empty())
		.flat_map(|line| {
			lines += 1;
			if let Some(columns) = columns {
				assert_eq!(columns, line.len());
			}
			columns = Some(line.len());
			line
		})
		.map(|byte| match byte {
			b'.' => 0u8,
			b'>' => 1u8,
			b'v' => 2u8,
			_ => unreachable!(),
		})
		.collect();
	Array2::from_shape_vec((lines, columns.unwrap()), values).unwrap()
}

fn step(map: &mut Array2<u8>, temp: &mut Array2<u8>) -> bool {
	let changed = AtomicBool::new(false);
	ndarray::Zip::indexed(&*map).par_map_assign_into(&mut *temp, |i, v| {
		let current = *v;
		let col_dim = map.raw_dim()[1];
		let left = map[(i.0, (i.1 + col_dim - 1) % col_dim)];
		let right = map[(i.0, (i.1 + 1) % col_dim)];
		if current == 0 && left == 1 {
			changed.store(true, Relaxed);
			1
		} else if current == 1 && right == 0 {
			changed.store(true, Relaxed);
			0
		} else {
			current
		}
	});
	ndarray::Zip::indexed(&*temp).par_map_assign_into(map, |i, v| {
		let current = *v;
		let row_dim = temp.raw_dim()[0];
		let above = temp[((i.0 + row_dim - 1) % row_dim, i.1)];
		let below = temp[((i.0 + 1) % row_dim, i.1)];
		if current == 0 && above == 2 {
			changed.store(true, Relaxed);
			2
		} else if current == 2 && below == 0 {
			changed.store(true, Relaxed);
			0
		} else {
			current
		}
	});
	changed.into_inner()
}

fn main() {
	let mut map = get_input();
	let mut temp = Array2::zeros(map.raw_dim());
	let mut steps = 0usize;
	while step(&mut map, &mut temp) {
		steps += 1;
	}
	println!("{}", steps + 1);
}
