use color_eyre::eyre::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_integers(file_name: &str) -> Result<Vec<i32>> {
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	file.lines().map(|line| Ok(line?.parse()?)).collect()
}

// the clever solution to the extended task: just compare the value
// being added to the window with the value being removed
fn count_increasing_adjacent_windows(values: &[i32], window_size: usize) -> usize {
	values
		.windows(window_size + 1)
		.filter(|window| window.first() < window.last())
		.count()
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let values = read_integers("input.01")?;
	println!("{}", count_increasing_adjacent_windows(&values, 1));
	println!("{}", count_increasing_adjacent_windows(&values, 3));
	Ok(())
}
