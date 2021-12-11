use chumsky::prelude::*;
use std::time::Instant;

static INPUT: &str = include_str!("../input");

fn parser() -> impl Parser<char, (), Error = Simple<char>> {
	recursive(|r| {
		let parentheses = r.clone().repeated().ignored().delimited_by('(', ')');
		let brackets = r.clone().repeated().ignored().delimited_by('[', ']');
		let braces = r.clone().repeated().ignored().delimited_by('{', '}');
		let angle_brackets = r.repeated().ignored().delimited_by('<', '>');
		parentheses.or(brackets).or(braces).or(angle_brackets)
	})
	.repeated()
	.ignored()
	.then_ignore(end())
}

// fn parser_with_recovery() -> impl Parser<char, (), Error = Simple<char>> {
// 	recursive(|r| {
// 		let parentheses = r.clone().repeated().ignored().delimited_by('(', ')');
// 		let brackets = r.clone().repeated().ignored().delimited_by('[', ']');
// 		let braces = r.clone().repeated().ignored().delimited_by('{', '}');
// 		let angle_brackets = r.clone().repeated().ignored().delimited_by('<', '>');
// 		parentheses
// 			.or(brackets)
// 			.or(braces)
// 			.or(angle_brackets)
// 	})
// 	.repeated()
// 	.ignored()
// 	.then_ignore(
// 		end()
// 			.recover_with(nested_delimiters(
// 				'(',
// 				')',
// 				[('[', ']'), ('{', '}'), ('<', '>')],
// 				|_| (),
// 			))
// 			.recover_with(nested_delimiters(
// 				'[',
// 				']',
// 				[('(', ')'), ('{', '}'), ('<', '>')],
// 				|_| (),
// 			))
// 			.recover_with(nested_delimiters(
// 				'{',
// 				'}',
// 				[('(', ')'), ('[', ']'), ('<', '>')],
// 				|_| (),
// 			))
// 			.recover_with(nested_delimiters(
// 				'<',
// 				'>',
// 				[('(', ')'), ('[', ']'), ('{', '}')],
// 				|_| (),
// 			)),
// 	)
// }

fn main() {
	let start = Instant::now();

	let syntax_error_score = INPUT
		.lines()
		.map(|line| {
			if let Err(simple) = parser().parse(line) {
				assert!(simple.len() == 1);
				let simple = simple.first().unwrap();
				if simple.reason() == &chumsky::error::SimpleReason::Unexpected {
					match simple.found() {
						None => 0,
						Some(')') => 3,
						Some(']') => 57,
						Some('}') => 1197,
						Some('>') => 25137,
						_ => unreachable!(),
					}
				} else {
					unreachable!();
				}
			} else {
				unreachable!();
			}
		})
		.sum::<usize>();
	println!("{}", syntax_error_score);

	let mid = Instant::now();

	let mut recovery_scores = Vec::new();
	let mut line_recovered = String::new();
	for line in INPUT.lines().filter(|&line| {
		if let Err(simple) = parser().parse(line) {
			assert!(simple.len() == 1);
			let simple = simple.first().unwrap();
			simple.reason() == &chumsky::error::SimpleReason::Unexpected && simple.found() == None
		} else {
			false
		}
	}) {
		line_recovered.clear();
		line_recovered.push_str(line);
		while let Err(simple) = parser().parse(line_recovered.as_str()) {
			assert!(simple.len() == 1);
			let simple = simple.first().unwrap();
			if simple.reason() == &chumsky::error::SimpleReason::Unexpected
				&& simple.found() == None
			{
				for c in simple.expected() {
					if matches!(c, ')' | ']' | '}' | '>') {
						line_recovered.push(*c);
						break;
					}
				}
			} else {
				unreachable!();
			}
		}
		let recovery = line_recovered.strip_prefix(line).unwrap();
		let recovery_score = recovery
			.chars()
			.map(|c| match c {
				')' => 1,
				']' => 2,
				'}' => 3,
				'>' => 4,
				_ => unreachable!(),
			})
			.fold(0usize, |old, new| old * 5 + new);
		recovery_scores.push(recovery_score);

		// println!("{}", recovery);
		// println!("{:?}", parser_with_recovery().parse_recovery_verbose(line));
		// todo!();
	}
	recovery_scores.sort_unstable();
	let median_recovery_score = recovery_scores[recovery_scores.len() / 2];
	println!("{}", median_recovery_score);

	let end = Instant::now();

	// excessively optimized version
	let opening = [b'(', b'[', b'{', b'<'];
	let closing = [b')', b']', b'}', b'>'];
	let mut error_score = 0usize;
	let mut recovery_scores = Vec::new();
	let mut stack = Vec::with_capacity(256);
	'lines: for line in INPUT.as_bytes().split(|&b| b == b'\n') {
		if line.is_empty() {
			continue;
		}
		stack.clear();
		for c in line.iter().copied() {
			if opening.contains(&c) {
				stack.push(c);
			} else if let Some(o) = stack.pop() {
				let ci = closing.iter().position(|&cc| cc == c).unwrap();
				if opening[ci] != o {
					error_score += match c {
						b')' => 3,
						b']' => 57,
						b'}' => 1197,
						b'>' => 25137,
						_ => unreachable!(),
					};
					continue 'lines;
				}
			} else {
				continue 'lines;
			}
		}
		let recovery_score = stack
			.iter()
			.rev()
			.map(|&c| match c {
				b'(' => 1,
				b'[' => 2,
				b'{' => 3,
				b'<' => 4,
				_ => unreachable!(),
			})
			.fold(0usize, |old, new| old * 5 + new);
		recovery_scores.push(recovery_score);
	}
	recovery_scores.sort_unstable();
	let median_recovery_score = recovery_scores[recovery_scores.len() / 2];

	// fast enough that time is dominated by printing
	let end_optimized_no_print = Instant::now();
	println!("{}", error_score);
	println!("{}", median_recovery_score);
	let end_optimized = Instant::now();

	println!("{} ms", (mid - start).as_millis());
	println!("{} ms", (end - mid).as_millis());
	println!("{} μs", (end_optimized_no_print - end).as_micros());
	println!("{} μs", (end_optimized - end).as_micros());
}
