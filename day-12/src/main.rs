use itertools::Itertools;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

static INPUT: &str = include_str!("../input");

fn parse_graph() -> UnGraph<&'static str, ()> {
	let mut graph = UnGraph::new_undirected();

	// ensure nodes aren't added twice
	let mut node_map: HashMap<&'static str, NodeIndex> = HashMap::new();
	let mut get_or_insert = |name: &'static str| {
		use std::collections::hash_map::Entry;
		match node_map.entry(name) {
			Entry::Occupied(entry) => *entry.get(),
			Entry::Vacant(entry) => *entry.insert(graph.add_node(name)),
		}
	};

	// ensure start/end always have indices 0/1
	get_or_insert("start");
	get_or_insert("end");

	// parse edges
	let mut edges = Vec::new();
	for line in INPUT.lines() {
		let (a, b) = line.split('-').collect_tuple().unwrap();
		let a = get_or_insert(a);
		let b = get_or_insert(b);
		edges.push((a, b));
	}
	graph.extend_with_edges(edges);

	graph
}

fn is_small_cave(node: NodeIndex, graph: &UnGraph<&'static str, ()>) -> bool {
	graph
		.node_weight(node)
		.unwrap()
		.chars()
		.all(char::is_lowercase)
}

fn find_paths(
	current: NodeIndex,
	partial: &mut Vec<NodeIndex>,
	paths: &mut Vec<Vec<NodeIndex>>,
	graph: &UnGraph<&'static str, ()>,
) {
	// end reached
	if current == 1.into() {
		partial.push(current);
		paths.push(partial.clone());
		let _ = partial.pop();
		return;
	}

	// small caves should only be visited once
	if is_small_cave(current, graph) && partial.contains(&current) {
		return;
	}

	partial.push(current);
	for edge in graph.edges(current) {
		find_paths(edge.target(), partial, paths, graph);
	}
	let _ = partial.pop();
}

fn find_paths_task_two(
	current: NodeIndex,
	partial: &mut Vec<NodeIndex>,
	paths: &mut Vec<Vec<NodeIndex>>,
	graph: &UnGraph<&'static str, ()>,
) {
	// end reached
	if current == 1.into() {
		partial.push(current);
		paths.push(partial.clone());
		let _ = partial.pop();
		return;
	}

	// small caves should only be visited once or twice
	if is_small_cave(current, graph) {
		let counts = partial
			.iter()
			.filter(|&&i| is_small_cave(i, graph))
			.copied()
			.counts();
		if counts.contains_key(&current) && counts.values().any(|&c| c >= 2) {
			return;
		}
	}

	partial.push(current);
	// start may only be visited once
	for edge in graph.edges(current).filter(|i| i.target() != 0.into()) {
		find_paths_task_two(edge.target(), partial, paths, graph);
	}
	let _ = partial.pop();
}

fn main() {
	let graph = parse_graph();

	let mut paths = Vec::new();
	find_paths(0.into(), &mut Vec::new(), &mut paths, &graph);
	println!("{}", paths.len());

	paths.clear();
	find_paths_task_two(0.into(), &mut Vec::new(), &mut paths, &graph);
	println!("{}", paths.len());
}
