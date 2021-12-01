use color_eyre::eyre::Result;
use itertools::process_results;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_integers(file_name: &str) -> Result<Vec<i32>> {
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	let values: Vec<_> = process_results(file.lines(), |lines| {
		process_results(lines.map(|s| s.parse::<i32>()), |values| values.collect())
	})??;
	Ok(values)
}

fn count_increasing_pairs(values: &[i32]) -> i32 {
	values
		.windows(2)
		.map(|window| (window[0] < window[1]) as i32)
		.sum()
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let values = read_integers("input.01")?;
	let result = count_increasing_pairs(&values);
	println!("{}", result);
	let windowed_values: Vec<_> = values
		.as_slice()
		.windows(3)
		.map(|window| window.iter().sum::<i32>())
		.collect();
	let result_2 = count_increasing_pairs(&windowed_values);
	println!("{}", result_2);
	Ok(())
}
