use color_eyre::eyre::Result;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
	#[error("incomplete input (missing call line)")]
	ExpectedCallLine,
	#[error("incorrect input (expected non-empty padding line)")]
	NonEmptyPadding,
	#[error("incorrect input (board has more or fewer than 25 entries)")]
	IncorrectLength,
}

use Error::{ExpectedCallLine, IncorrectLength, NonEmptyPadding};

fn read_bingo_calls(lines: &mut Lines<BufReader<File>>) -> Result<Vec<u8>> {
	lines
		.next()
		.ok_or(ExpectedCallLine)??
		.split(',')
		.map(|call| Ok(call.parse()?))
		.collect()
}

#[derive(Clone, Copy, Debug)]
struct BingoBoard {
	numbers: [[u8; 5]; 5],
	markers: [[bool; 5]; 5],
}

impl BingoBoard {
	fn mark(&mut self, call: u8) -> bool {
		let BingoBoard { numbers, markers } = self;
		let flat_numbers = numbers.iter().map(|line| line.iter()).flatten();
		let flat_markers = markers.iter_mut().map(|line| line.iter_mut()).flatten();
		for (number, marker) in flat_numbers.zip(flat_markers) {
			if *number == call {
				*marker = true;
				return true;
			}
		}
		false
	}

	fn wins(&self) -> bool {
		// check rows
		self.markers.iter().any(|row| row.iter().all(|x| *x)) ||
		// check columns
		(0..5).any(|col| self.markers.iter().all(|row| row[col]))
	}

	fn score(&self, call: u8) -> u32 {
		let flat_numbers = self.numbers.iter().map(|line| line.iter()).flatten();
		let flat_markers = self.markers.iter().map(|line| line.iter()).flatten();
		let unmarked_sum: u32 = flat_numbers
			.zip(flat_markers)
			.map(|(&number, &marker)| number as u32 * !marker as u32)
			.sum();
		unmarked_sum * call as u32
	}
}

fn read_bingo_board(lines: &mut Lines<BufReader<File>>) -> Result<Option<BingoBoard>> {
	let padding_line = match lines.next() {
		None => return Ok(None),
		Some(padding_line) => padding_line?,
	};
	if !padding_line.is_empty() {
		return Err(NonEmptyPadding.into());
	}
	let board = array_init::from_iter(lines.take(5).filter_map(|line| -> Option<[u8; 5]> {
		let line = line.ok()?;
		array_init::from_iter(
			line.split_whitespace()
				.filter_map(|entry| entry.parse().ok()),
		)
	}))
	.ok_or(IncorrectLength)?;
	Ok(Some(BingoBoard {
		numbers: board,
		markers: <_>::default(),
	}))
}

struct BingoInput(Vec<u8>, Vec<BingoBoard>);

fn read_bingo_input(file_name: &str) -> Result<BingoInput> {
	let file = File::open(file_name)?;
	let file = BufReader::new(file);
	let mut lines = file.lines();
	let calls = read_bingo_calls(&mut lines)?;
	let mut bingo_boards = Vec::new();
	while let Some(bingo_board) = read_bingo_board(&mut lines)? {
		bingo_boards.push(bingo_board);
	}
	Ok(BingoInput(calls, bingo_boards))
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let BingoInput(calls, mut bingo_boards) = read_bingo_input("input.04")?;
	for call in calls {
		println!("{}", call);
		for board in &mut bingo_boards {
			if board.mark(call) && board.wins() {
				println!("bingo! {}", board.score(call));
			}
		}
		bingo_boards = bingo_boards
			.iter()
			.filter(|board| !board.wins())
			.copied()
			.collect();
		if bingo_boards.is_empty() {
			break;
		}
	}
	Ok(())
}
