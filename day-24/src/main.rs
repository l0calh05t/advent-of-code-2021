use itertools::Itertools;
use rayon::prelude::*;

const INSTRUCTIONS: &str = include_str!("../input");

fn main() {
	let rtoi = |r: &str| match r {
		"w" => Ok(0usize),
		"x" => Ok(1),
		"y" => Ok(2),
		"z" => Ok(3),
		other => Err(other.parse::<i32>().unwrap()),
	};
	let mut states: Vec<([i32; 4], (usize, usize))> = Vec::new();
	let mut new_states: Vec<([i32; 4], (usize, usize))> = Vec::new();
	states.push(([0; 4], (0, 0)));
	for instruction in INSTRUCTIONS.lines() {
		println!("{}", instruction);
		if let Some(r) = instruction.strip_prefix("inp ") {
			let i = rtoi(r).unwrap();
			new_states = states
				.par_drain(..)
				.flat_map_iter(|(state, (model_lo, model_hi))| {
					(1..=9).map(move |v| {
						let mut new_state = state;
						new_state[i] = v;
						(
							new_state,
							(10 * model_lo + v as usize, 10 * model_hi + v as usize),
						)
					})
				})
				.collect();
		} else if let Some(ab) = instruction.strip_prefix("mul ") {
			let (a, b) = ab.split_whitespace().collect_tuple().unwrap();
			let a = rtoi(a).unwrap();
			let b = rtoi(b);
			states
				.par_drain(..)
				.map(|(mut state, model)| {
					match b {
						Ok(b) => {
							state[a] *= state[b];
						}
						Err(b) => {
							state[a] *= b;
						}
					}
					(state, model)
				})
				.collect_into_vec(&mut new_states);
		} else if let Some(ab) = instruction.strip_prefix("add ") {
			let (a, b) = ab.split_whitespace().collect_tuple().unwrap();
			let a = rtoi(a).unwrap();
			let b = rtoi(b);
			states
				.par_drain(..)
				.map(|(mut state, model)| {
					match b {
						Ok(b) => {
							state[a] += state[b];
						}
						Err(b) => {
							state[a] += b;
						}
					}
					(state, model)
				})
				.collect_into_vec(&mut new_states);
		} else if let Some(ab) = instruction.strip_prefix("div ") {
			let (a, b) = ab.split_whitespace().collect_tuple().unwrap();
			let a = rtoi(a).unwrap();
			let b = rtoi(b);
			states
				.par_drain(..)
				.map(|(mut state, model)| {
					match b {
						Ok(b) => {
							state[a] /= state[b];
						}
						Err(b) => {
							state[a] /= b;
						}
					}
					(state, model)
				})
				.collect_into_vec(&mut new_states);
		} else if let Some(ab) = instruction.strip_prefix("mod ") {
			let (a, b) = ab.split_whitespace().collect_tuple().unwrap();
			let a = rtoi(a).unwrap();
			let b = rtoi(b);
			states
				.par_drain(..)
				.map(|(mut state, model)| {
					match b {
						Ok(b) => {
							state[a] %= state[b];
						}
						Err(b) => {
							state[a] %= b;
						}
					}
					(state, model)
				})
				.collect_into_vec(&mut new_states);
		} else if let Some(ab) = instruction.strip_prefix("eql ") {
			let (a, b) = ab.split_whitespace().collect_tuple().unwrap();
			let a = rtoi(a).unwrap();
			let b = rtoi(b);
			states
				.par_drain(..)
				.map(|(mut state, model)| {
					match b {
						Ok(b) => {
							state[a] = (state[a] == state[b]) as _;
						}
						Err(b) => {
							state[a] = (state[a] == b) as _;
						}
					}
					(state, model)
				})
				.collect_into_vec(&mut new_states);
		} else {
			unreachable!()
		}

		new_states.par_sort_unstable();
		let mut current: Option<([i32; 4], (usize, usize))> = None;
		for (state, (model_lo, model_hi)) in new_states.drain(..) {
			if let Some((cur_state, (cur_lo, cur_hi))) = current {
				if state == cur_state {
					current = Some((cur_state, (cur_lo.min(model_lo), cur_hi.max(model_hi))));
				} else {
					states.push((cur_state, (cur_lo, cur_hi)));
					current = Some((state, (model_lo, model_hi)));
				}
			} else {
				current = Some((state, (model_lo, model_hi)));
			}
		}
		if let Some(current) = current {
			states.push(current);
		}

		println!("{:?}", states.len());
	}

	let largest_model_number = states
		.iter()
		.filter_map(|(state, (_, model))| if state[3] == 0 { Some(*model) } else { None })
		.max()
		.unwrap();
	println!("{}", largest_model_number);

	let smallest_model_number = states
		.iter()
		.filter_map(|(state, (model, _))| if state[3] == 0 { Some(*model) } else { None })
		.min()
		.unwrap();
	println!("{}", smallest_model_number);
}
