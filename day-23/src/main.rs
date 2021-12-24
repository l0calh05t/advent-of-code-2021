use itertools::Itertools;
use pathfinding::directed::dijkstra::dijkstra;
use std::{fmt::Write, mem::take};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State<const ROOM_SIZE: usize> {
	hallway: ([Tile; 2], [Tile; 3], [Tile; 2]),
	rooms: [[Tile; ROOM_SIZE]; 4],
}

impl<const ROOM_SIZE: usize> State<ROOM_SIZE> {
	fn locations(&self) -> impl Iterator<Item = Location> + Clone {
		(0u8..4)
			.cartesian_product(0..ROOM_SIZE as u8)
			.map(|(n, i)| Location::Room(n, i))
			.chain(
				[
					Location::Hallway(0, 0),
					Location::Hallway(0, 1),
					Location::Hallway(1, 0),
					Location::Hallway(1, 1),
					Location::Hallway(1, 2),
					Location::Hallway(2, 0),
					Location::Hallway(2, 1),
				]
				.into_iter(),
			)
	}

	fn distance(&self, from: Location, to: Location) -> Option<usize> {
		if self[from].is_empty() || !self[to].is_empty() {
			return None;
		}
		if let Location::Room(number, index) = to {
			if !self[from].is_type(number) {
				return None;
			}
			if !self.rooms[number as usize][index as usize + 1..]
				.iter()
				.all(|tile| tile.is_type(number))
			{
				return None;
			}
		}
		self.distance_helper(from, to)
	}

	fn distance_helper(&self, from: Location, to: Location) -> Option<usize> {
		if from.same_room(&to) {
			return None;
		}
		if let Location::Room(to_number, _) = to {
			if let Location::Room(from_number, _) = from {
				let intersection = Location::Hallway(1, to_number.min(from_number));
				if !self[intersection].is_empty() {
					return None;
				}
				return self
					.distance_helper(from, intersection)
					.and_then(|a| self.distance_helper(intersection, to).map(|b| a + b));
			} else {
				return self.distance_helper(to, from);
			}
		}
		if let Location::Room(room_number, room_index) = from {
			if !self.rooms[room_number as usize][..room_index as usize]
				.iter()
				.all(Tile::is_empty)
			{
				return None;
			}
			match to {
				Location::Hallway(0, hallway_index) => {
					if !self.hallway.0[..hallway_index as usize]
						.iter()
						.all(Tile::is_empty)
					{
						return None;
					}
					return self.hallway.1[..room_number as usize]
						.iter()
						.all(Tile::is_empty)
						.then(|| {
							2 + 2 * room_number as usize
								+ hallway_index as usize + room_index as usize
						});
				}
				Location::Hallway(1, hallway_index) => {
					let hallway_index = hallway_index as usize;
					let room_number = room_number as usize;
					let range = if hallway_index < room_number {
						hallway_index + 1..room_number
					} else {
						room_number..hallway_index
					};
					let passed = range.len();
					return self.hallway.1[range]
						.iter()
						.all(Tile::is_empty)
						.then(|| 2 + 2 * passed + room_index as usize);
				}
				Location::Hallway(2, hallway_index) => {
					if !self.hallway.2[..hallway_index as usize]
						.iter()
						.all(Tile::is_empty)
					{
						return None;
					}
					return self.hallway.1[room_number as usize..]
						.iter()
						.all(Tile::is_empty)
						.then(|| {
							2 + 2 * (3 - room_number) as usize
								+ hallway_index as usize + room_index as usize
						});
				}
				_ => {}
			}
		}
		unreachable!();
	}
}

// why is Default still only implemented for fixed-size arrays?!
impl<const ROOM_SIZE: usize> Default for State<ROOM_SIZE> {
	fn default() -> Self {
		Self {
			hallway: Default::default(),
			rooms: [[Default::default(); ROOM_SIZE]; 4],
		}
	}
}

impl<const ROOM_SIZE: usize> std::fmt::Display for State<ROOM_SIZE> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("#############\n")?;
		f.write_char('#')?;
		for tile in self.hallway.0.iter().rev() {
			f.write_fmt(format_args!("{}", tile))?;
		}
		f.write_char('.')?;
		for tile in Itertools::intersperse(self.hallway.1.iter(), &Tile::Empty) {
			f.write_fmt(format_args!("{}", tile))?;
		}
		f.write_char('.')?;
		for tile in self.hallway.2.iter() {
			f.write_fmt(format_args!("{}", tile))?;
		}
		f.write_str("#\n###")?;
		for k in 0..ROOM_SIZE {
			for room in self.rooms {
				f.write_fmt(format_args!("{}", room[k]))?;
				f.write_char('#')?;
			}
			if k == 0 {
				f.write_str("##\n  #")?;
			} else if k + 1 < ROOM_SIZE {
				f.write_str("\n  #")?;
			} else {
				f.write_str("\n  #########")?;
			}
		}
		Ok(())
	}
}

impl<const ROOM_SIZE: usize> std::ops::Index<Location> for State<ROOM_SIZE> {
	type Output = Tile;

	fn index(&self, index: Location) -> &Self::Output {
		match index {
			Location::Hallway(0, index) => &self.hallway.0[index as usize],
			Location::Hallway(1, index) => &self.hallway.1[index as usize],
			Location::Hallway(2, index) => &self.hallway.2[index as usize],
			Location::Room(number, index) => &(self.rooms[number as usize][index as usize]),
			_ => unreachable!(),
		}
	}
}

impl<const ROOM_SIZE: usize> std::ops::IndexMut<Location> for State<ROOM_SIZE> {
	fn index_mut(&mut self, index: Location) -> &mut Self::Output {
		match index {
			Location::Hallway(0, index) => &mut self.hallway.0[index as usize],
			Location::Hallway(1, index) => &mut self.hallway.1[index as usize],
			Location::Hallway(2, index) => &mut self.hallway.2[index as usize],
			Location::Room(number, index) => &mut (self.rooms[number as usize][index as usize]),
			_ => unreachable!(),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
	Empty,
	Amphipod(u8),
}

impl Tile {
	fn is_empty(&self) -> bool {
		matches!(self, Tile::Empty)
	}

	fn is_type(&self, t: u8) -> bool {
		match self {
			Tile::Amphipod(x) => *x == t,
			_ => false,
		}
	}

	fn move_cost(&self) -> usize {
		match self {
			Tile::Amphipod(t) => 10usize.pow(*t as u32),
			_ => unreachable!(),
		}
	}
}

impl Default for Tile {
	fn default() -> Self {
		Tile::Empty
	}
}

impl std::fmt::Display for Tile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Tile::Empty => f.write_char('.'),
			Tile::Amphipod(t) => f.write_char((*t + b'A') as char),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Location {
	Hallway(u8, u8),
	Room(u8, u8),
}

impl Location {
	fn same_room(&self, other: &Location) -> bool {
		if self == other {
			true
		} else {
			match self {
				Location::Hallway(_, _) => matches!(other, Location::Hallway(_, _)),
				Location::Room(number, _) => {
					if let Location::Room(other_number, _) = other {
						number == other_number
					} else {
						false
					}
				}
			}
		}
	}
}

fn successors<const ROOM_SIZE: usize>(state: &State<ROOM_SIZE>) -> Vec<(State<ROOM_SIZE>, usize)> {
	state
		.locations()
		.cartesian_product(state.locations())
		.filter_map(|(from, to)| {
			state.distance(from, to).map(|distance| {
				let mut target = *state;
				let tile = take(&mut target[from]);
				target[to] = tile;
				(target, tile.move_cost() * distance)
			})
		})
		.collect()
}

fn main() {
	let initial_state = {
		let mut initial_state = State::<2>::default();
		initial_state.rooms[0][0] = Tile::Amphipod(3); // D
		initial_state.rooms[0][1] = Tile::Amphipod(3); // D
		initial_state.rooms[1][0] = Tile::Amphipod(2); // C
		initial_state.rooms[1][1] = Tile::Amphipod(0); // A
		initial_state.rooms[2][0] = Tile::Amphipod(1); // B
		initial_state.rooms[2][1] = Tile::Amphipod(0); // A
		initial_state.rooms[3][0] = Tile::Amphipod(2); // C
		initial_state.rooms[3][1] = Tile::Amphipod(1); // B
		initial_state
	};

	let (_path, cost) = dijkstra(&initial_state, successors, |state| {
		state.locations().all(|location| {
			if let Location::Room(number, _) = location {
				state[location].is_type(number)
			} else {
				state[location].is_empty()
			}
		})
	})
	.unwrap();

	println!("{}", cost);

	let initial_state = {
		let mut initial_state = State::<4>::default();
		initial_state.rooms[0][0] = Tile::Amphipod(3); // D
		initial_state.rooms[0][1] = Tile::Amphipod(3); // D *
		initial_state.rooms[0][2] = Tile::Amphipod(3); // D *
		initial_state.rooms[0][3] = Tile::Amphipod(3); // D
		initial_state.rooms[1][0] = Tile::Amphipod(2); // C
		initial_state.rooms[1][1] = Tile::Amphipod(2); // C *
		initial_state.rooms[1][2] = Tile::Amphipod(1); // B *
		initial_state.rooms[1][3] = Tile::Amphipod(0); // A
		initial_state.rooms[2][0] = Tile::Amphipod(1); // B
		initial_state.rooms[2][1] = Tile::Amphipod(1); // B *
		initial_state.rooms[2][2] = Tile::Amphipod(0); // A *
		initial_state.rooms[2][3] = Tile::Amphipod(0); // A
		initial_state.rooms[3][0] = Tile::Amphipod(2); // C
		initial_state.rooms[3][1] = Tile::Amphipod(0); // A *
		initial_state.rooms[3][2] = Tile::Amphipod(2); // C *
		initial_state.rooms[3][3] = Tile::Amphipod(1); // B
		initial_state
	};

	let (_path, cost) = dijkstra(&initial_state, successors, |state| {
		state.locations().all(|location| {
			if let Location::Room(number, _) = location {
				state[location].is_type(number)
			} else {
				state[location].is_empty()
			}
		})
	})
	.unwrap();

	println!("{}", cost);
}
