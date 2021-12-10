use chumsky::prelude::*;

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
// 			.recover_with(nested_delimiters('(', ')', [], |_| ()))
// 			.recover_with(nested_delimiters('[', ']', [], |_| ()))
// 			.recover_with(nested_delimiters('{', '}', [], |_| ()))
// 			.recover_with(nested_delimiters('<', '>', [], |_| ()))
// 	})
// 	.repeated()
// 	.ignored()
// 	.then_ignore(end())
// }

fn main() {
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
	}
	recovery_scores.sort_unstable();
	let median_recovery_score = recovery_scores[recovery_scores.len() / 2];
	println!("{}", median_recovery_score);
}
