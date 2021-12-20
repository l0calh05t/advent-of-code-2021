use itertools::Itertools;
use ndarray::{prelude::*, Zip};

fn to_bool(c: char) -> bool {
	match c {
		'#' => true,
		'.' => false,
		_ => unreachable!(),
	}
}

fn enhance(algorithm: [bool; 512], image: (Array2<bool>, bool)) -> (Array2<bool>, bool) {
	let (image, padding) = image;

	let mut padded_image =
		Array2::from_elem(Ix2(image.shape()[0] + 4, image.shape()[1] + 4), padding);
	padded_image
		.slice_mut(s![
			2..padded_image.shape()[0] - 2,
			2..padded_image.shape()[1] - 2
		])
		.assign(&image);

	let output_image = Zip::from(padded_image.windows([3; 2])).par_map_collect(|window| {
		algorithm[window
			.iter()
			.fold(0usize, |value, pixel| (value << 1) + *pixel as usize)]
	});

	(output_image, algorithm[511 * padding as usize])
}

fn pixels(input: &str) -> Array2<bool> {
	let mut rows = 0usize;
	let mut columns = None;
	let pixels = input
		.lines()
		.flat_map(|line| {
			rows += 1;
			if let Some(columns) = columns {
				assert_eq!(line.len(), columns);
			}
			columns = Some(line.len());
			line.chars()
		})
		.map(to_bool)
		.collect_vec();
	Array2::from_shape_vec((rows, columns.unwrap()), pixels).unwrap()
}

fn main() {
	let input = include_str!("../input");
	let (algorithm, image) = input.split("\n\n").collect_tuple().unwrap();
	let algorithm: [bool; 512] = array_init::from_iter(algorithm.chars().map(to_bool)).unwrap();

	let image = pixels(image);

	let mut image = (image, false);
	for _ in 0..2 {
		image = enhance(algorithm, image);
	}
	println!("{}", image.0.iter().filter(|v| **v).count());

	for _ in 2..50 {
		image = enhance(algorithm, image);
	}
	println!("{}", image.0.iter().filter(|v| **v).count());
}
