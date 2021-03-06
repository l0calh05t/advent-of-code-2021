use color_eyre::Result;
use common::read_comma_separated;
use ndarray::prelude::*;
use ndarray_linalg::{types::c64, AsDiagonal, Eig, Inverse, LinearOperator};

fn main() -> Result<()> {
	color_eyre::install()?;
	let timers = read_comma_separated::<u8>("day-06/input")?;
	let mut timer_counts = [0usize; 9];
	for timer in timers {
		assert!((0..=6).contains(&timer));
		timer_counts[timer as usize] += 1;
	}

	let iteration_matrix = arr2(&[
		[0., 1., 0., 0., 0., 0., 0., 0., 0.],
		[0., 0., 1., 0., 0., 0., 0., 0., 0.],
		[0., 0., 0., 1., 0., 0., 0., 0., 0.],
		[0., 0., 0., 0., 1., 0., 0., 0., 0.],
		[0., 0., 0., 0., 0., 1., 0., 0., 0.],
		[0., 0., 0., 0., 0., 0., 1., 0., 0.],
		[1., 0., 0., 0., 0., 0., 0., 1., 0.],
		[0., 0., 0., 0., 0., 0., 0., 0., 1.],
		[1., 0., 0., 0., 0., 0., 0., 0., 0.],
	]);

	let (eigenvalues, eigenvectors) = iteration_matrix.eig()?;
	let eigenvectors_inv = eigenvectors.inv()?;

	// compute Re{P D⁸⁰ P⁻¹}x = M⁸⁰x
	let result_80 = eigenvectors
		.apply2(
			&eigenvalues
				.mapv(|e: c64| e.powc(80.0.into()))
				.as_diagonal()
				.apply2(&eigenvectors_inv),
		)
		.view()
		.split_complex()
		.re
		.mapv(|e| e.round() as usize)
		.dot(&ArrayView::from(&timer_counts))
		.sum();
	println!("{}", result_80);

	// compute Re{P D²⁵⁶ P⁻¹}x = M²⁵⁶x
	let result_256 = eigenvectors
		.apply2(
			&eigenvalues
				.mapv(|e: c64| e.powc(256.0.into()))
				.as_diagonal()
				.apply2(&eigenvectors_inv),
		)
		.view()
		.split_complex()
		.re
		.mapv(|e| e.round() as usize)
		.dot(&ArrayView::from(&timer_counts))
		.sum();
	println!("{}", result_256);

	Ok(())
}
