use color_eyre::Result;
use common::read_digit_field;
use ndarray::{concatenate, prelude::*, IntoDimension};
use petgraph::algo::astar;

#[derive(Clone, Debug)]
struct GridGraph(Array2<u8>);

#[derive(Clone, Debug, Default)]
struct GridMap(Array2<bool>);

impl petgraph::visit::GraphBase for GridGraph {
	type EdgeId = ([usize; 2], [usize; 2]);
	type NodeId = [usize; 2];
}

impl petgraph::visit::VisitMap<[usize; 2]> for GridMap {
	fn visit(&mut self, a: [usize; 2]) -> bool {
		let visited = self.is_visited(&a);
		self.0[a] = true;
		visited
	}

	fn is_visited(&self, a: &[usize; 2]) -> bool {
		self.0[*a]
	}
}

impl petgraph::visit::Visitable for GridGraph {
	type Map = GridMap;

	fn visit_map(&self) -> Self::Map {
		GridMap::default()
	}

	fn reset_map(&self, map: &mut Self::Map) {
		map.0.fill(false);
	}
}

#[derive(Clone, Debug)]
struct GridGraphNeighbors {
	node: [usize; 2],
	shape: [usize; 2],
	axis: u8,
	hilo: i8,
}

impl GridGraphNeighbors {
	fn new(node: [usize; 2], shape: Dim<[usize; 2]>) -> Self {
		Self {
			node,
			shape: [shape[0], shape[1]],
			axis: 0,
			hilo: -1,
		}
	}
}

impl Iterator for GridGraphNeighbors {
	type Item = [usize; 2];

	fn next(&mut self) -> Option<Self::Item> {
		let Self {
			node,
			shape,
			axis,
			hilo,
		} = self;

		let mut result = None;

		while result.is_none() {
			if *axis >= 2 {
				return None;
			}
			if node[*axis as usize] > 0 && *hilo == -1 {
				let mut temp = *node;
				temp[*axis as usize] -= 1;
				result = Some(temp);
			} else if node[*axis as usize] + 1 < shape[*axis as usize] && *hilo == 1 {
				let mut temp = *node;
				temp[*axis as usize] += 1;
				result = Some(temp);
			}
			if *hilo == -1 {
				*hilo = 1;
			} else {
				*hilo = -1;
				*axis += 1;
			}
		}

		result
	}
}

impl petgraph::visit::IntoNeighbors for &GridGraph {
	type Neighbors = GridGraphNeighbors;

	fn neighbors(self, a: Self::NodeId) -> Self::Neighbors {
		GridGraphNeighbors::new(a, self.0.raw_dim())
	}
}

impl petgraph::visit::Data for GridGraph {
	type NodeWeight = ();
	type EdgeWeight = ();
}

#[derive(Clone, Debug)]
struct GridGraphNodeEdges(GridGraphNeighbors);

impl GridGraphNodeEdges {
	fn new(node: [usize; 2], shape: Dim<[usize; 2]>) -> Self {
		Self(GridGraphNeighbors::new(node, shape))
	}
}

impl Iterator for GridGraphNodeEdges {
	type Item = ([usize; 2], [usize; 2], &'static ());

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|other| (self.0.node, other, &()))
	}
}

#[derive(Clone, Debug)]
struct GridGraphEdges(GridGraphNodeEdges);

impl GridGraphEdges {
	fn new(shape: Dim<[usize; 2]>) -> Self {
		Self(GridGraphNodeEdges::new([0, 0], shape))
	}
}

impl Iterator for GridGraphEdges {
	type Item = ([usize; 2], [usize; 2], &'static ());

	fn next(&mut self) -> Option<Self::Item> {
		if let Some(edge) = self.0.next() {
			Some(edge)
		} else {
			let mut temp = self.0 .0.node;
			let shape = self.0 .0.shape;
			if temp[0] + 1 < shape[0] {
				temp[0] += 1;
				self.0 = GridGraphNodeEdges::new(temp, shape.into_dimension());
				self.0.next()
			} else if temp[1] + 1 < shape[1] {
				temp[0] = 0;
				temp[1] += 1;
				self.0 = GridGraphNodeEdges::new(temp, shape.into_dimension());
				self.0.next()
			} else {
				None
			}
		}
	}
}

impl petgraph::visit::IntoEdgeReferences for &GridGraph {
	type EdgeRef = ([usize; 2], [usize; 2], &'static ());
	type EdgeReferences = GridGraphEdges;

	fn edge_references(self) -> Self::EdgeReferences {
		GridGraphEdges::new(self.0.raw_dim())
	}
}

impl petgraph::visit::IntoEdges for &GridGraph {
	type Edges = GridGraphNodeEdges;

	fn edges(self, a: Self::NodeId) -> Self::Edges {
		GridGraphNodeEdges::new(a, self.0.raw_dim())
	}
}

fn main() -> Result<()> {
	color_eyre::install()?;
	let grid = GridGraph(read_digit_field("day-15/input")?);

	let dim = grid.0.raw_dim();
	let goal = [dim[0] - 1, dim[1] - 1];
	if let Some(path) = astar(
		&grid,
		[0, 0],
		|node| node == goal,
		|edge| grid.0[edge.1] as usize,
		|node| (goal[0] - node[0]) + (goal[1] - node[1]),
	) {
		println!("{:?}", path.0);
	}

	let tile = &grid.0;
	let mut tiled_grid = grid.0.clone();
	for k in 0..4 {
		let wrapped = tile.map(|v| (v + k) % 9 + 1);
		tiled_grid = concatenate![Axis(0), tiled_grid, wrapped];
	}
	let tile = tiled_grid.clone();
	for k in 0..4 {
		let wrapped = tile.map(|v| (v + k) % 9 + 1);
		tiled_grid = concatenate![Axis(1), tiled_grid, wrapped];
	}

	let tiled_grid = GridGraph(tiled_grid);

	let dim = tiled_grid.0.raw_dim();
	let goal = [dim[0] - 1, dim[1] - 1];
	if let Some(path) = astar(
		&tiled_grid,
		[0, 0],
		|node| node == goal,
		|edge| tiled_grid.0[edge.1] as usize,
		|node| (goal[0] - node[0]) + (goal[1] - node[1]),
	) {
		println!("{:?}", path.0);
	}

	Ok(())
}
