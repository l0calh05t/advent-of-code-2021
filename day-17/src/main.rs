use color_eyre::Result;
use itertools::Itertools;
use regex::Regex;

fn main() -> Result<()> {
	color_eyre::install()?;

	// parse input
	let input = include_str!("../input");
	let pattern = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)")?;
	let captures = pattern.captures(input).unwrap();
	let (x_min, x_max, y_min, y_max) = captures
		.iter()
		.filter_map(|capture| {
			let capture = capture?;
			capture.as_str().parse::<i32>().ok()
		})
		.collect_tuple()
		.unwrap();

	// check our assumptions
	assert!(x_min <= x_max);
	assert!(y_min <= y_max);
	assert!(x_min >= 0);
	assert!(y_max <= 0);

	let x_target = x_min..=x_max;
	let y_target = y_min..=y_max;

	// for x, the valid range is easy to determine
	// x_vel_0 must be large enough to reach x_min
	// x_vel_0 must be at most x_max
	// x_vel (x_vel + 1) / 2 = x_min
	// x_vel² + x_vel - 2 x_min = 0
	// x_vel = (-1 ± sqrt(1 + 8 x_min)) / 2
	let x_vel_min = ((-1.0 + (1.0 + 8.0 * x_min as f64).sqrt()) / 2.0).ceil() as i32;
	let x_vel_range = x_vel_min..=x_max;

	// for y, this is just a conservative guess for now
	// the actual range should probably depend on the choice of x
	// -> determine t range for which x is in x_target and compute corresponding y velocities?
	let y_vel_range = y_min..=300;

	let simulate = |(x_vel_0, y_vel_0): (i32, i32)| {
		let mut x = 0;
		let mut y = 0;
		let mut x_vel = x_vel_0;
		let mut y_vel = y_vel_0;
		let mut y_max = 0;

		while y >= y_min && x <= x_max {
			x += x_vel;
			y += y_vel;
			x_vel -= x_vel.signum();
			y_vel -= 1;
			y_max = y_max.max(y);

			if x_target.contains(&x) && y_target.contains(&y) {
				return Some(y_max);
			}
		}

		None
	};

	let y_max = x_vel_range
		.clone()
		.cartesian_product(y_vel_range.clone())
		.filter_map(simulate)
		.max()
		.unwrap();
	println!("{}", y_max);

	let valid_count = x_vel_range
		.cartesian_product(y_vel_range)
		.filter_map(simulate)
		.count();
	println!("{}", valid_count);

	Ok(())
}
