use color_eyre::Result;
use itertools::Itertools;
use ndarray::{prelude::*, ErrorKind::IncompatibleShape, ShapeError};
use std::{
	fs::File,
	io::{BufRead, BufReader, Read},
	str::{from_utf8, FromStr},
};

pub fn read_digit_field(file_name: &str) -> Result<Array2<u8>> {
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

pub fn read_comma_separated<T>(file_name: &str) -> Result<Vec<T>>
where
	T: FromStr,
	<T as std::str::FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	file.split(b',')
		.map(|line| Ok(from_utf8(&line?)?.trim().parse()?))
		.collect()
}
