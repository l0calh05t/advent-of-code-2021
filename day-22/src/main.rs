use arrayvec::ArrayVec;
use color_eyre::Result;
use itertools::Itertools;
use regex::Regex;
use std::{
	fs::File,
	io::{BufRead, BufReader},
	mem::swap,
	ops::RangeInclusive,
};
use thiserror::Error;

#[derive(Debug, Clone)]
struct Cuboid([RangeInclusive<i32>; 3]);

impl Cuboid {
	fn split(&self, other: &Cuboid) -> (ArrayVec<Self, 26>, ArrayVec<Self, 1>, ArrayVec<Self, 26>) {
		let xyzs: [_; 3] =
			array_init::from_iter(self.0.iter().zip(other.0.iter()).map(|(rs, ro)| {
				let mut vs = [*rs.start(), *rs.end() + 1, *ro.start(), *ro.end() + 1];
				vs.sort_unstable();
				vs
			}))
			.unwrap();
		let (xs, ys, zs) = xyzs
			.iter()
			.map(|vs| {
				vs.iter()
					.tuple_windows()
					.filter_map(|(l, h)| if l == h { None } else { Some(*l..=*h - 1) })
			})
			.collect_tuple()
			.unwrap();
		let xyzs = itertools::iproduct!(xs, ys, zs);
		let mut in_self = ArrayVec::new();
		let mut in_both = ArrayVec::new();
		let mut in_other = ArrayVec::new();
		for ranges in xyzs.into_iter().map(|(xr, yr, zr)| [xr, yr, zr]) {
			// the containment checks only need to check a single point after the split
			let is_in_self = self
				.0
				.iter()
				.zip(ranges.iter())
				.all(|(a, b)| a.contains(b.start()));
			let is_in_other = other
				.0
				.iter()
				.zip(ranges.iter())
				.all(|(a, b)| a.contains(b.start()));
			if is_in_self && is_in_other {
				in_both.push(Cuboid(ranges));
			} else if is_in_self {
				in_self.push(Cuboid(ranges));
			} else if is_in_other {
				in_other.push(Cuboid(ranges));
			}
		}
		(in_self, in_both, in_other)
	}

	fn overlaps(&self, other: &Cuboid) -> bool {
		self.0
			.iter()
			.zip(other.0.iter())
			.all(|(s, o)| s.start() <= o.end() && o.start() <= s.end())
	}

	fn count(&self) -> usize {
		self.0.iter().cloned().map(Iterator::count).product()
	}

	fn count_within(&self, bounds: &Cuboid) -> usize {
		if !self.overlaps(bounds) {
			return 0;
		}
		let (_, inside, _) = self.split(bounds);
		inside.iter().map(Cuboid::count).sum()
	}
}

#[derive(Debug, Clone)]
enum Instruction {
	On(Cuboid),
	Off(Cuboid),
}

impl Instruction {
	fn is_on(&self) -> bool {
		matches!(self, Instruction::On(_))
	}

	fn cuboid(&self) -> Cuboid {
		match self {
			Instruction::On(cuboid) | Instruction::Off(cuboid) => cuboid,
		}
		.clone()
	}
}

#[derive(Error, Debug)]
enum Error {
	#[error("incorrect input (instruction has incorrect format)")]
	IncorrectInstructionFormat,
}

fn parse_instructions(file_name: &str) -> Result<Vec<Instruction>> {
	let pattern =
		Regex::new(r"(on|off) x=(-?\d+)\.\.(-?\d+),y=(-?\d+)\.\.(-?\d+),z=(-?\d+)\.\.(-?\d+)")?;
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	file.lines()
		.map(|line| {
			let line = line?;
			let captures = pattern
				.captures(&line)
				.ok_or(Error::IncorrectInstructionFormat)?;
			let mut captures = captures.iter();
			let _ = captures.next(); // skip initial everything-group
			let instruction = captures
				.next()
				.flatten()
				.ok_or(Error::IncorrectInstructionFormat)?;
			let constructor = match instruction.as_str() {
				"on" => Instruction::On,
				"off" => Instruction::Off,
				_ => return Err(Error::IncorrectInstructionFormat.into()),
			};
			let (x_min, x_max, y_min, y_max, z_min, z_max) = captures
				.map(|m| -> Result<i32> {
					let m = m.ok_or(Error::IncorrectInstructionFormat)?;
					Ok(m.as_str().parse()?)
				})
				.collect_tuple()
				.ok_or(Error::IncorrectInstructionFormat)?;
			let (x_min, x_max, y_min, y_max, z_min, z_max) =
				(x_min?, x_max?, y_min?, y_max?, z_min?, z_max?);
			Ok(constructor(Cuboid([
				x_min..=x_max,
				y_min..=y_max,
				z_min..=z_max,
			])))
		})
		.collect()
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let instructions = parse_instructions("day-22/input")?;
	let mut active = Vec::with_capacity(instructions.len());
	let mut next_active = Vec::with_capacity(instructions.len());
	for instruction in instructions {
		let enable = instruction.is_on();
		let cuboid = instruction.cuboid();
		for active_cuboid in active.drain(..) {
			if !cuboid.overlaps(&active_cuboid) {
				next_active.push(active_cuboid);
				continue;
			}
			let (in_active, _in_both, _in_instruction) = active_cuboid.split(&cuboid);
			next_active.extend(in_active);
		}
		if enable {
			next_active.push(cuboid);
		}
		swap(&mut active, &mut next_active);
	}
	let bounds = Cuboid([-50..=50, -50..=50, -50..=50]);
	let bounded_active_count: usize = active
		.iter()
		.map(|cuboid| cuboid.count_within(&bounds))
		.sum();
	println!("{}", bounded_active_count);
	let active_count: usize = active.iter().map(Cuboid::count).sum();
	println!("{}", active_count);
	Ok(())
}
