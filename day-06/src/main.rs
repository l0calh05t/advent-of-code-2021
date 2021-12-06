use color_eyre::Result;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	str::from_utf8,
};

fn read_integers(file_name: &str) -> Result<Vec<u8>> {
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	file.split(b',')
		.map(|line| Ok(from_utf8(&line?)?.trim().parse()?))
		.collect()
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let timers = read_integers("day-06/input")?;
	let mut timer_counts = [0usize; 9];
	for timer in timers {
		assert!((0..=6).contains(&timer));
		timer_counts[timer as usize] += 1;
	}
	let iterate = |timer_counts: &mut [usize; 9]| {
		// shift every timer one to the left, 0 is mapped to 8
		timer_counts.rotate_left(1);
		// every timer that was in 0 should also be added to 6
		timer_counts[6] += timer_counts[8];
	};
	for _ in 0..80 {
		iterate(&mut timer_counts);
	}
	println!("{}", timer_counts.iter().sum::<usize>());
	for _ in 80..256 {
		iterate(&mut timer_counts);
	}
	println!("{}", timer_counts.iter().sum::<usize>());
	Ok(())
}
