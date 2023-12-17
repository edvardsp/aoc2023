use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};

use ndarray::Array2;

#[derive(Debug)]
pub struct Input {
    map: Array2<u32>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let map = s
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .collect();
        let map = Array2::from_shape_vec((height, width), map).unwrap();
        Self { map }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coord((usize, usize));

impl Coord {
    fn next(&self, dir: Direction, shape: &[usize]) -> Option<Self> {
        let (y, x) = self.0;
        match dir {
            Direction::Up if y == 0 => None,
            Direction::Down if y == shape[0] - 1 => None,
            Direction::Left if x == 0 => None,
            Direction::Right if x == shape[1] - 1 => None,
            Direction::Up => Some(Self((y - 1, x))),
            Direction::Down => Some(Self((y + 1, x))),
            Direction::Left => Some(Self((y, x - 1))),
            Direction::Right => Some(Self((y, x + 1))),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn clockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        }
    }

    fn anticlockwise(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

trait State: Clone + Ord + PartialOrd + Hash {
    fn initial(coord: Coord) -> impl Iterator<Item = Self>;
    fn cost(&self) -> u32;
    fn is_goal(&self, goal: Coord) -> bool;
    fn neighbors(&self, map: &Array2<u32>) -> impl Iterator<Item = Self>;
}

#[derive(Copy, Clone, Debug)]
struct Crucible {
    cost: u32,
    coord: Coord,
    dir: Direction,
    steps: usize,
}

impl State for Crucible {
    fn initial(coord: Coord) -> impl Iterator<Item = Self> {
        vec![
            Self {
                cost: 0,
                coord,
                dir: Direction::Right,
                steps: 0,
            },
            Self {
                cost: 0,
                coord,
                dir: Direction::Down,
                steps: 0,
            },
        ]
        .into_iter()
    }

    fn cost(&self) -> u32 {
        self.cost
    }

    fn is_goal(&self, goal: Coord) -> bool {
        self.coord == goal
    }

    fn neighbors(&self, map: &Array2<u32>) -> impl Iterator<Item = Self> {
        let shape = map.shape();
        let mut neighbors = Vec::new();

        let anticlockwise = self.dir.anticlockwise();
        if let Some(pos) = self.coord.next(anticlockwise, shape) {
            neighbors.push(Self {
                cost: self.cost + map[pos.0],
                coord: pos,
                dir: anticlockwise,
                steps: 1,
            });
        }

        if self.steps < 3 {
            if let Some(pos) = self.coord.next(self.dir, shape) {
                neighbors.push(Self {
                    cost: self.cost + map[pos.0],
                    coord: pos,
                    dir: self.dir,
                    steps: self.steps + 1,
                });
            }
        }

        let clockwise = self.dir.clockwise();
        if let Some(pos) = self.coord.next(clockwise, shape) {
            neighbors.push(Self {
                cost: self.cost + map[pos.0],
                coord: pos,
                dir: clockwise,
                steps: 1,
            });
        }

        neighbors.into_iter()
    }
}

impl Eq for Crucible {}

impl PartialEq for Crucible {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.dir == other.dir && self.steps == other.steps
    }
}

impl Hash for Crucible {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
        self.dir.hash(state);
        self.steps.hash(state);
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Min-heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug)]
struct UltraCrucible {
    cost: u32,
    coord: Coord,
    dir: Direction,
    steps: usize,
}

impl State for UltraCrucible {
    fn initial(coord: Coord) -> impl Iterator<Item = Self> {
        vec![
            Self {
                cost: 0,
                coord,
                dir: Direction::Right,
                steps: 0,
            },
            Self {
                cost: 0,
                coord,
                dir: Direction::Down,
                steps: 0,
            },
        ]
        .into_iter()
    }

    fn cost(&self) -> u32 {
        self.cost
    }

    fn is_goal(&self, goal: Coord) -> bool {
        self.coord == goal && self.steps >= 4
    }

    fn neighbors(&self, map: &Array2<u32>) -> impl Iterator<Item = Self> {
        let shape = map.shape();
        let mut neighbors = Vec::new();

        if self.steps >= 4 {
            let anticlockwise = self.dir.anticlockwise();
            if let Some(pos) = self.coord.next(anticlockwise, shape) {
                neighbors.push(Self {
                    cost: self.cost + map[pos.0],
                    coord: pos,
                    dir: anticlockwise,
                    steps: 1,
                });
            }

            let clockwise = self.dir.clockwise();
            if let Some(pos) = self.coord.next(clockwise, shape) {
                neighbors.push(Self {
                    cost: self.cost + map[pos.0],
                    coord: pos,
                    dir: clockwise,
                    steps: 1,
                });
            }
        }

        if self.steps < 10 {
            if let Some(pos) = self.coord.next(self.dir, shape) {
                neighbors.push(Self {
                    cost: self.cost + map[pos.0],
                    coord: pos,
                    dir: self.dir,
                    steps: self.steps + 1,
                });
            }
        }

        neighbors.into_iter()
    }
}

impl Eq for UltraCrucible {}

impl PartialEq for UltraCrucible {
    fn eq(&self, other: &Self) -> bool {
        self.coord == other.coord && self.dir == other.dir && self.steps == other.steps
    }
}

impl Hash for UltraCrucible {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coord.hash(state);
        self.dir.hash(state);
        self.steps.hash(state);
    }
}

impl Ord for UltraCrucible {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Min-heap
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.coord.cmp(&other.coord))
    }
}

impl PartialOrd for UltraCrucible {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<S: State>(map: &Array2<u32>, initial: Coord, goal: Coord) -> Option<u32> {
    let mut heap = BinaryHeap::new();
    let mut dist = HashMap::new();

    for state in S::initial(initial) {
        dist.insert(state.clone(), 0);
        heap.push(state);
    }

    while let Some(state) = heap.pop() {
        if state.is_goal(goal) {
            return Some(state.cost());
        }

        if state.cost() > *dist.get(&state).unwrap_or(&u32::MAX) {
            continue;
        }

        for next in state.neighbors(map) {
            if next.cost() < *dist.get(&next).unwrap_or(&u32::MAX) {
                dist.insert(next.clone(), next.cost());
                heap.push(next);
            }
        }
    }
    None
}

pub fn part1(input: &Input) -> u32 {
    let shape = input.map.shape();
    dijkstra::<Crucible>(
        &input.map,
        Coord((0, 0)),
        Coord((shape[0] - 1, shape[1] - 1)),
    )
    .unwrap()
}

pub fn part2(input: &Input) -> u32 {
    let shape = input.map.shape();
    dijkstra::<UltraCrucible>(
        &input.map,
        Coord((0, 0)),
        Coord((shape[0] - 1, shape[1] - 1)),
    )
    .unwrap()
}
