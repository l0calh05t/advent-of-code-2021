use chumsky::prelude::*;
use itertools::Itertools;
use std::fmt::Write;
use std::time::Instant;

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

use arrayvec::ArrayVec;

#[derive(Debug, Clone, Default)]
struct SnailfishNumber2(ArrayVec<(u8, u8), 16>);

impl SnailfishNumber2 {
	fn magnitude(&self) -> usize {
		let mut result = 0;
		let mut stack: ArrayVec<u8, 4> = ArrayVec::new();
		for (v, l) in self.0.iter().copied() {
			assert!(stack.len() <= l as _);

			while stack.len() < l as _ {
				if let Some(top) = stack.last_mut() {
					*top += 1;
				}
				stack.push(0);
			}
			let top = stack.last_mut().unwrap();
			*top += 1;

			let multiplier: usize = stack.iter().map(|c| 4 - *c as usize).product();
			result += multiplier * v as usize;

			while matches!(stack.last(), Some(&2)) {
				let _ = stack.pop();
			}
		}
		result
	}
}

impl std::fmt::Display for SnailfishNumber2 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut stack: ArrayVec<u8, 4> = ArrayVec::new();
		for (v, l) in self.0.iter().copied() {
			assert!(stack.len() <= l as _);

			if let Some(top) = stack.last() {
				if *top == 1 {
					f.write_char(',')?;
				}
			}
			while stack.len() < l as _ {
				if let Some(top) = stack.last_mut() {
					*top += 1;
				}
				stack.push(0);
				f.write_char('[')?;
			}
			let top = stack.last_mut().unwrap();
			f.write_fmt(format_args!("{}", v))?;
			*top += 1;

			while matches!(stack.last(), Some(&2)) {
				let _ = stack.pop();
				f.write_str("]")?;
			}
		}
		while let Some(level_count) = stack.pop() {
			assert_eq!(level_count, 2);
			f.write_char(']')?;
		}
		Ok(())
	}
}

impl std::str::FromStr for SnailfishNumber2 {
	type Err = std::convert::Infallible;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut result = Self::default();
		let mut level = 0;
		for c in s.chars() {
			match c {
				'[' => {
					level += 1;
				}
				']' => {
					level -= 1;
				}
				c @ '0'..='9' => result.0.push((c as u8 - b'0', level)),
				',' => {}
				_ => unreachable!(),
			}
		}
		Ok(result)
	}
}

impl std::ops::Add for SnailfishNumber2 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		let mut temp: ArrayVec<(u8, u8), 32> = self
			.0
			.iter()
			.chain(rhs.0.iter())
			.map(|(v, l)| (*v, *l + 1))
			.collect();

		// explode
		let mut idx = 0;
		while idx < temp.len() {
			let (v, l) = temp[idx];
			if l < 5 {
				idx += 1;
				continue;
			}
			if idx > 0 {
				temp[idx - 1].0 += v;
			}
			temp[idx].0 = 0;
			temp[idx].1 = 4;
			let (vn, ln) = temp.remove(idx + 1);
			assert_eq!(ln, 5);
			if idx + 1 < temp.len() {
				temp[idx + 1].0 += vn;
			}
			idx += 1;
		}

		// split
		idx = 0;
		while idx < temp.len() {
			let (v, _) = temp[idx];
			if v < 10 {
				idx += 1;
				continue;
			}
			let half = v / 2;
			let other = v - half;
			if temp[idx].1 < 4 {
				// split without explode
				temp[idx].0 = half;
				temp[idx].1 += 1;
				temp.insert(idx + 1, (other, temp[idx].1));
			} else {
				// split with explode
				temp[idx].0 = 0;
				if idx + 1 < temp.len() {
					temp[idx + 1].0 += other;
				}
				if idx > 0 {
					temp[idx - 1].0 += half;
					idx -= 1;
				}
			}
		}

		Self(temp.iter().copied().collect())
	}
}

fn main() {
	let start = Instant::now();
	let numbers = include_str!("../input")
		.lines()
		.map(|line| SnailfishNumber::parse(line).unwrap())
		.collect_vec();
	let numbers = numbers.iter().cloned();
	let end = Instant::now();
	println!("{} μs", (end - start).as_micros());

	let start = Instant::now();
	let sum = numbers
		.clone()
		.fold(None, |a, b| {
			if let Some(a) = a {
				Some(a + b)
			} else {
				Some(b)
			}
		})
		.unwrap();
	let sum_magnitude = sum.magnitude();
	let end = Instant::now();
	println!("{} ({} μs)", sum_magnitude, (end - start).as_micros());

	let start = Instant::now();
	let max_pairwise_magnitude = numbers
		.clone()
		.cartesian_product(numbers)
		.map(|(a, b)| (a + b).magnitude())
		.max()
		.unwrap();
	let end = Instant::now();
	println!(
		"{} ({} μs)",
		max_pairwise_magnitude,
		(end - start).as_micros()
	);

	let start = Instant::now();
	let numbers = include_str!("../input")
		.lines()
		.map(|line| line.parse::<SnailfishNumber2>().unwrap())
		.collect_vec();
	let numbers = numbers.iter().cloned();
	let end = Instant::now();
	println!("{} μs", (end - start).as_micros());

	let start = Instant::now();
	let sum = numbers
		.clone()
		.fold(None, |a, b| {
			if let Some(a) = a {
				Some(a + b)
			} else {
				Some(b)
			}
		})
		.unwrap();
	let sum_magnitude = sum.magnitude();
	let end = Instant::now();
	println!("{} ({} μs)", sum_magnitude, (end - start).as_micros());

	let start = Instant::now();
	let max_pairwise_magnitude = numbers
		.clone()
		.cartesian_product(numbers)
		.map(|(a, b)| (a + b).magnitude())
		.max()
		.unwrap();
	let end = Instant::now();
	println!(
		"{} ({} μs)",
		max_pairwise_magnitude,
		(end - start).as_micros()
	);
}
