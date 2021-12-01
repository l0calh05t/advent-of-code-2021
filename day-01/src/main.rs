use color_eyre::eyre::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::process_results;

fn main() -> Result<()> {
	color_eyre::install()?;
	let file = File::open("input.01")?;
	let file = BufReader::new(file);
	let values: Vec<_> = process_results(file.lines(), |lines| {
		process_results(lines.map(|s| s.parse::<i32>()), |values| values.collect())
	})??;
	let result: i32 = values
		.as_slice()
		.windows(2)
		.map(|window| (window[0] < window[1]) as i32)
		.sum();
	println!("{}", result);
	let windowed_values: Vec<_> = values
		.as_slice()
		.windows(3)
		.map(|window| window.iter().sum::<i32>())
		.collect();
	let result_2: i32 = windowed_values
		.as_slice()
		.windows(2)
		.map(|window| (window[0] < window[1]) as i32)
		.sum();
	println!("{}", result_2);
	Ok(())
}
