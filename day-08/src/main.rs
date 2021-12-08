use bitvec::{array::BitArray, bitarr, order::Lsb0};
use color_eyre::Result;
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
struct SegmentState(BitArray<Lsb0, u8>);

impl SegmentState {
	fn from_str(input: &str) -> Option<Self> {
		let mut state = BitArray::new(0);
		for char in input.as_bytes().iter().copied() {
			let index = char.checked_sub(b'a')?;
			if index >= 7 {
				return None;
			}
			state.set(index as _, true);
		}
		Some(SegmentState(state))
	}

	fn is_one(self) -> bool {
		self.0.count_ones() == 2
	}

	fn is_four(self) -> bool {
		self.0.count_ones() == 4
	}

	fn is_seven(self) -> bool {
		self.0.count_ones() == 3
	}

	fn is_eight(self) -> bool {
		self.0.count_ones() == 7
	}
}

fn get_input() -> Vec<([SegmentState; 10], [SegmentState; 4])> {
	let text = include_str!("../input");
	let mut ret = Vec::new();
	for line in text.lines() {
		let (inputs, outputs) = line.split('|').collect_tuple().unwrap();
		let inputs: [_; 10] = array_init::from_iter(
			inputs
				.split_whitespace()
				.map(|string| SegmentState::from_str(string).unwrap()),
		)
		.unwrap();
		let outputs: [_; 4] = array_init::from_iter(
			outputs
				.split_whitespace()
				.map(|string| SegmentState::from_str(string).unwrap()),
		)
		.unwrap();
		ret.push((inputs, outputs))
	}
	ret
}

fn decode(inout: ([SegmentState; 10], [SegmentState; 4])) -> usize {
	let mut potential_encodings: [BitArray<Lsb0, u8>; 8] = [BitArray::new(0b1111111); 8];

	let one_segments = bitarr![0, 0, 1, 0, 0, 1, 0];
	let four_segments = bitarr![0, 1, 1, 1, 0, 1, 0];
	let seven_segments = bitarr![1, 0, 1, 0, 0, 1, 0];

	let two_segments = bitarr![1, 0, 1, 1, 1, 0, 1];
	let three_segments = bitarr![1, 0, 1, 1, 0, 1, 1];
	let five_segments = bitarr![1, 1, 0, 1, 0, 1, 1];

	let zero_segments = bitarr![1, 1, 1, 0, 1, 1, 1];
	let six_segments = bitarr![1, 1, 0, 1, 1, 1, 1];
	let nine_segments = bitarr![1, 1, 1, 1, 0, 1, 1];

	for input in inout.0 {
		if input.is_one() {
			for (index, value) in input.0.iter().enumerate() {
				potential_encodings[index] &= if value == true {
					one_segments
				} else {
					!one_segments
				};
			}
		} else if input.is_seven() {
			for (index, value) in input.0.iter().enumerate() {
				potential_encodings[index] &= if value == true {
					seven_segments
				} else {
					!seven_segments
				};
			}
		} else if input.is_four() {
			for (index, value) in input.0.iter().enumerate() {
				potential_encodings[index] &= if value == true {
					four_segments
				} else {
					!four_segments
				};
			}
		}
	}

	let potential_match = |input: &SegmentState,
	                       segments: &BitArray<Lsb0, [usize; 1]>,
	                       potential_encodings: &[BitArray<Lsb0, u8>; 8]| {
		input
			.0
			.iter_ones()
			.all(|index| (potential_encodings[index] & *segments).any())
	};

	let validate_match = |input: &SegmentState,
	                      segments: &BitArray<Lsb0, [usize; 1]>,
	                      potential_encodings: &[BitArray<Lsb0, u8>; 8]| {
		input
			.0
			.iter_ones()
			.map(|index| potential_encodings[index] & *segments)
			.tuple_combinations()
			.all(|(a, b)| (a | b).count_ones() >= 2)
	};

	for input in inout.0 {
		let mut matches = None;
		// 2 3 5
		if input.0.count_ones() == 5 {
			if potential_match(&input, &two_segments, &potential_encodings)
				&& validate_match(&input, &two_segments, &potential_encodings)
			{
				if matches.is_some() {
					break;
				}
				matches = Some(two_segments)
			}
			if potential_match(&input, &three_segments, &potential_encodings)
				&& validate_match(&input, &three_segments, &potential_encodings)
			{
				if matches.is_some() {
					break;
				}
				matches = Some(three_segments)
			}
			if potential_match(&input, &five_segments, &potential_encodings)
				&& validate_match(&input, &five_segments, &potential_encodings)
			{
				if matches.is_some() {
					break;
				}
				matches = Some(five_segments)
			}
		}
		// 0 6 9
		else if input.0.count_ones() == 6 {
			if potential_match(&input, &zero_segments, &potential_encodings)
				&& validate_match(&input, &zero_segments, &potential_encodings)
			{
				if matches.is_some() {
					break;
				}
				matches = Some(zero_segments)
			}
			if potential_match(&input, &six_segments, &potential_encodings)
				&& validate_match(&input, &six_segments, &potential_encodings)
			{
				if matches.is_some() {
					break;
				}
				matches = Some(six_segments)
			}
			if potential_match(&input, &nine_segments, &potential_encodings)
				&& validate_match(&input, &nine_segments, &potential_encodings)
			{
				if matches.is_some() {
					break;
				}
				matches = Some(nine_segments)
			}
		}
		if let Some(segments) = matches {
			for (index, value) in input.0.iter().enumerate() {
				potential_encodings[index] &= if value == true { segments } else { !segments };
			}
		}
	}

	// check that all patterns are unique now
	assert!(potential_encodings
		.iter()
		.take(7)
		.all(|pattern| pattern.count_ones() == 1));

	let encodings = potential_encodings;
	let mut result = 0;
	for output in inout.1 {
		result *= 10;
		if output.is_one() {
			result += 1;
		} else if output.is_four() {
			result += 4;
		} else if output.is_seven() {
			result += 7;
		} else if output.is_eight() {
			result += 8;
		} else {
			let mut pattern: BitArray<Lsb0, u8> = BitArray::new(0);
			for index in output.0.iter_ones() {
				pattern |= encodings[index];
			}
			if output.0.count_ones() == 5 {
				if pattern[..7] == two_segments[..7] {
					result += 2;
				} else if pattern[..7] == three_segments[..7] {
					result += 3;
				} else {
					assert_eq!(pattern[..7], five_segments[..7]);
					result += 5;
				}
			} else {
				assert_eq!(output.0.count_ones(), 6);
				if pattern[..7] == six_segments[..7] {
					result += 6;
				} else if pattern[..7] == nine_segments[..7] {
					result += 9;
				} else {
					assert_eq!(pattern[..7], zero_segments[..7]);
				}
			}
		}
	}
	result
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let input = get_input();
	let easily_recognizable = input
		.iter()
		.flat_map(|input| input.1)
		.filter(|display| {
			display.is_one() || display.is_four() || display.is_seven() || display.is_eight()
		})
		.count();
	println!("{}", easily_recognizable);
	println!("{}", input.iter().copied().map(decode).sum::<usize>());
	Ok(())
}
