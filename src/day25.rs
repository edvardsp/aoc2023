use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug)]
pub struct Input {
    graph: Graph,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let mut adjecents: HashMap<String, Vec<String>> = HashMap::new();
        for line in s.lines() {
            let (from, tos) = line.split_once(": ").unwrap();
            let from = from.to_string();
            for to in tos.split_whitespace().map(str::to_string) {
                adjecents.entry(from.clone()).or_default().push(to.clone());
                adjecents.entry(to.clone()).or_default().push(from.clone());
            }
        }
        let graph = Graph::new(adjecents);
        Self { graph }
    }
}

#[derive(Clone, Debug)]
struct Graph {
    vertices: Vec<String>,
    adjecents: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new(adjecents: HashMap<String, Vec<String>>) -> Self {
        let vertices = adjecents.keys().cloned().collect();
        Self {
            vertices,
            adjecents,
        }
    }

    fn len(&self) -> usize {
        self.vertices.len()
    }

    fn randomize(&mut self) {
        self.vertices.shuffle(&mut thread_rng());
        self.adjecents
            .values_mut()
            .for_each(|tos| tos.shuffle(&mut thread_rng()));
    }

    fn seed_vertex(&self) -> &str {
        self.vertices.first().unwrap().as_str()
    }

    fn edges<'g>(&'g self, from: &'g str) -> impl Iterator<Item = (&'g str, &'g str)> {
        match self.adjecents.get(from) {
            Some(tos) => tos.iter().map(String::as_str).map(move |to| (from, to)),
            None => panic!("unknown vertex: {from}"),
        }
    }

    // Create a spanning tree using BFS
    fn spanning_tree(&self, blacklist: &[Edge]) -> Vec<Edge> {
        let mut visited = HashSet::with_capacity(self.len());
        let mut tree = Vec::with_capacity(self.len() - 1);

        let seed = self.seed_vertex();
        visited.insert(seed);

        let mut queue = VecDeque::from_iter(self.edges(seed));
        while let Some((from, to)) = queue.pop_front() {
            let edge = Edge::new(from, to);
            if blacklist.contains(&edge) {
                continue;
            }

            if visited.insert(to) {
                queue.extend(self.edges(to));
                tree.push(edge);
            }
        }

        tree
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Edge(String, String);

impl Edge {
    fn new(from: &str, to: &str) -> Self {
        // In order to have a stable comparison between edges, we sort the to/from vertices.
        let (to, from) = match from.cmp(&to) {
            Ordering::Greater => (to.to_string(), from.to_string()),
            Ordering::Less => (from.to_string(), to.to_string()),
            Ordering::Equal => panic!("an edge cannot point to itself: {from}"),
        };
        Self(to, from)
    }
}

struct History {
    hist: HashMap<Edge, usize>,
}

impl History {
    fn new() -> Self {
        Self {
            hist: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.hist.clear();
    }

    fn record(&mut self, edges: Vec<Edge>) {
        for edge in edges {
            *self.hist.entry(edge).or_default() += 1;
        }
    }

    fn top(&self, amount: usize) -> Vec<(&Edge, &usize)> {
        let mut edges: Vec<_> = self.hist.iter().collect();
        edges.sort_by(|lhs, rhs| rhs.1.cmp(&lhs.1));
        edges.truncate(amount);
        edges
    }
}

pub fn part1(input: &Input) -> usize {
    const MIN_CUT: usize = 3;
    const THRESHOLD: usize = 4;

    let mut graph = input.graph.clone();

    let mut hist = History::new();
    let mut blacklist = Vec::new();
    for wires_clipped in 0..MIN_CUT {
        hist.clear();
        for _i in 0.. {
            graph.randomize();
            hist.record(graph.spanning_tree(&blacklist));

            // Heuristic: the spanning tree will always include one wire of interest, and will with high probability include all non-clipped wires of interest.
            // Based on this, we assume the top N = (3 - wires_clipped) are wires of interest, meaning the next N+1 edge is a wire that should not be clipped.
            // We stop searching for the most recurring edge when the N+1 edge has deviated sufficiently from the most recurring edge.
            // The threshold is based on empirical testing.
            let top = hist.top(MIN_CUT + 1 - wires_clipped);
            let &(edge0, n0) = top.first().unwrap();
            let &(_edge1, n1) = top.last().unwrap();
            if (n0 - n1) >= THRESHOLD {
                println!("Blacklist {edge0:?} after {_i} attemps");
                blacklist.push(edge0.clone());
                break;
            }
        }
    }

    // We now have with high probability blacklisted all wires of interest.
    // Finding the spanning tree one more time should now return a disjointed graph.
    let subtree = graph.spanning_tree(&blacklist);
    let total_len = graph.len();
    // Calculate the number of vertices in the spanning tree with +1
    let group1 = subtree.len() + 1;
    let group2 = total_len - group1;

    group1 * group2
}

pub fn part2(_input: &Input) -> usize {
    2023
}
