fn multiverse_game(
	positions: [u8; 2],
	scores: [u8; 2],
	index: usize,
	multiverses: usize,
) -> [usize; 2] {
	// 111×1 3
	// 112×3 4
	// 113×3 5
	// 122×3 5
	// 123×6 6
	// 133×3 7
	// 222×1 6
	// 223×3 7
	// 233×3 8
	// 333×1 9
	// 3 => 1
	// 4 => 3
	// 5 => 6
	// 6 => 7
	// 7 => 6
	// 8 => 3
	// 9 => 1
	let player = index % 2;
	let mut wins = [0; 2];
	for (roll, multiplier) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
		let mut positions = positions;
		let mut scores = scores;
		positions[player] += roll;
		positions[player] %= 10;
		scores[player] += positions[player] + 1;
		let partial_wins = if scores[player] >= 21 {
			let mut partial = [0, 0];
			partial[player] = multiplier * multiverses;
			partial
		} else {
			multiverse_game(positions, scores, index + 1, multiplier * multiverses)
		};
		for (sum, part) in wins.iter_mut().zip(partial_wins.into_iter()) {
			*sum += part;
		}
	}
	wins
}

fn main() {
	// getting lazy... these are the input values, adjusted by - 1
	let starting_positions = [3, 8];

	// in modular arithmetic, the deterministic 3d100 produce a
	// sequence 6, 5, 4, 3, 2, 1, 0, 9, 8, 7, ...
	let deterministic_die = (0..=9).rev().cycle().skip(3);
	let mut wins = false;
	let result = deterministic_die
		.enumerate()
		.scan(
			(starting_positions, [0; 2], 0),
			|(positions, scores, rolls), (index, roll)| {
				let player = index % 2;
				// we store positions as 0..=9 instead of 1..=10 so
				// modular arithmetic can be used directly
				positions[player] += roll;
				positions[player] %= 10;
				// adjust score for different numbering
				scores[player] += positions[player] + 1;
				*rolls = 3 * (index + 1);
				Some((*positions, *scores, *rolls))
			},
		)
		.take_while(move |(_, scores, _)| {
			let result = !wins;
			wins = scores[0] >= 1000 || scores[1] >= 1000;
			result
		})
		.last()
		.unwrap();
	if result.1[0] >= 1000 {
		println!("{}", result.1[1] * result.2);
	} else {
		println!("{}", result.1[0] * result.2);
	}

	// getting lazy... these are the input values, adjusted by - 1
	let starting_positions = [3, 8];
	let wins = multiverse_game(starting_positions, [0; 2], 0, 1);
	println!("{:?}", wins.iter().max().unwrap());
}
