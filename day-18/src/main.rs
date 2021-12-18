use chumsky::prelude::*;
use itertools::Itertools;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
	Regular(usize),
	Pair(Box<SnailfishNumber>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SnailfishNumber(Element, Element);

fn parser() -> impl Parser<char, SnailfishNumber, Error = Simple<char>> {
	let regular = one_of("0123456789".chars())
		.map(|digit: char| Element::Regular(digit as usize - '0' as usize));
	let element = recursive(|element| {
		regular.or(just('[')
			.ignore_then(element.clone())
			.then_ignore(just(','))
			.then(element)
			.then_ignore(just(']'))
			.map(|(a, b): (Element, Element)| Element::Pair(Box::new(SnailfishNumber(a, b)))))
	});
	just('[')
		.ignore_then(element.clone())
		.then_ignore(just(','))
		.then(element)
		.then_ignore(just(']'))
		.map(|(a, b): (Element, Element)| SnailfishNumber(a, b))
}

impl SnailfishNumber {
	fn parse(input: &str) -> Result<Self, Vec<Simple<char>>> {
		parser().parse(input)
	}

	fn reduce(&mut self) {
		while self.explode(0).is_some() || self.split() {}
	}

	fn explode(&mut self, depth: usize) -> Option<(Option<usize>, Option<usize>)> {
		if let Some((left, mut right)) = self.0.explode(depth + 1) {
			if let Some(right) = right.take() {
				self.1.add_leftmost(right);
			}
			Some((left, right))
		} else if let Some((mut left, right)) = self.1.explode(depth + 1) {
			if let Some(left) = left.take() {
				self.0.add_rightmost(left);
			}
			Some((left, right))
		} else {
			None
		}
	}

	fn split(&mut self) -> bool {
		self.0.split() || self.1.split()
	}

	fn magnitude(&self) -> usize {
		3 * self.0.magnitude() + 2 * self.1.magnitude()
	}
}

impl Element {
	fn explode(&mut self, depth: usize) -> Option<(Option<usize>, Option<usize>)> {
		if let Element::Pair(ref mut pair) = self {
			if depth == 4 {
				if let SnailfishNumber(Element::Regular(a), Element::Regular(b)) = pair.as_ref() {
					let result = (Some(*a), Some(*b));
					*self = Element::Regular(0);
					Some(result)
				} else {
					unreachable!()
				}
			} else {
				pair.explode(depth)
			}
		} else {
			None
		}
	}

	fn split(&mut self) -> bool {
		match self {
			Element::Regular(digit) if *digit >= 10 => {
				let digit = *digit;
				let half = digit / 2;
				*self = Element::Pair(Box::new(SnailfishNumber(
					Element::Regular(half),
					Element::Regular(digit - half),
				)));
				true
			}
			Element::Pair(ref mut pair) => pair.split(),
			_ => false,
		}
	}

	fn magnitude(&self) -> usize {
		match self {
			Element::Regular(digit) => *digit,
			Element::Pair(pair) => pair.magnitude(),
		}
	}

	fn add_leftmost(&mut self, value: usize) {
		match self {
			Element::Regular(digit) => *digit += value,
			Element::Pair(ref mut pair) => pair.0.add_leftmost(value),
		}
	}

	fn add_rightmost(&mut self, value: usize) {
		match self {
			Element::Regular(digit) => *digit += value,
			Element::Pair(ref mut pair) => pair.1.add_rightmost(value),
		}
	}
}

impl std::ops::Add for SnailfishNumber {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let mut result =
			SnailfishNumber(Element::Pair(Box::new(self)), Element::Pair(Box::new(rhs)));
		result.reduce();
		result
	}
}

impl std::fmt::Display for Element {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Element::Regular(digit) => f.write_fmt(format_args!("{}", digit)),
			Element::Pair(number) => f.write_fmt(format_args!("{}", number.as_ref())),
		}
	}
}

impl std::fmt::Display for SnailfishNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char('[')?;
		f.write_fmt(format_args!("{}", self.0))?;
		f.write_char(',')?;
		f.write_fmt(format_args!("{}", self.1))?;
		f.write_char(']')
	}
}

fn main() {
	let numbers = include_str!("../input")
		.lines()
		.map(|line| SnailfishNumber::parse(line).unwrap());
	let sum = numbers
		.clone()
		.fold(None, |a: Option<SnailfishNumber>, b| {
			if let Some(a) = a {
				Some(a + b)
			} else {
				Some(b)
			}
		})
		.unwrap();
	println!("{}", sum.magnitude());
	let max_pairwise_magnitude = numbers
		.clone()
		.cartesian_product(numbers)
		.map(|(a, b)| (a + b).magnitude())
		.max()
		.unwrap();
	println!("{}", max_pairwise_magnitude);
}
