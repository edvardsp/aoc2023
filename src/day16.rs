use ndarray::Array2;
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct Input {
    map: Array2<char>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let map = s.lines().flat_map(|line| line.chars()).collect();
        let map = Array2::from_shape_vec((height, width), map).unwrap();
        Self { map }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Coord((usize, usize));

impl Coord {
    fn next(&self, dir: Direction, shape: &[usize]) -> Option<Self> {
        let (y, x) = self.0;
        match dir {
            Direction::Up if y > 0 => Some(Self((y - 1, x))),
            Direction::Up => None,
            Direction::Down if y < shape[0] - 1 => Some(Self((y + 1, x))),
            Direction::Down => None,
            Direction::Left if x > 0 => Some(Self((y, x - 1))),
            Direction::Left => None,
            Direction::Right if x < shape[1] - 1 => Some(Self((y, x + 1))),
            Direction::Right => None,
        }
    }
}

fn energized(map: &Array2<char>, initial: Coord, dir: Direction) -> usize {
    let mut lights: VecDeque<_> = VecDeque::from([(initial, dir)]);
    let mut visited = HashSet::new();
    let shape = map.shape();
    let mut energized = HashSet::new();
    while let Some((coord, dir)) = lights.pop_front() {
        energized.insert(coord.0);
        if !visited.insert((coord, dir)) {
            continue;
        }

        match (map[coord.0], dir) {
            ('.' | '|', Direction::Up | Direction::Down) => {
                if let Some(coord) = coord.next(dir, shape) {
                    lights.push_back((coord, dir));
                }
            }
            ('.' | '-', Direction::Left | Direction::Right) => {
                if let Some(coord) = coord.next(dir, shape) {
                    lights.push_back((coord, dir));
                }
            }
            ('|', Direction::Left | Direction::Right) => {
                if let Some(coord) = coord.next(Direction::Up, shape) {
                    lights.push_back((coord, Direction::Up));
                }
                if let Some(coord) = coord.next(Direction::Down, shape) {
                    lights.push_back((coord, Direction::Down));
                }
            }
            ('-', Direction::Up | Direction::Down) => {
                if let Some(coord) = coord.next(Direction::Left, shape) {
                    lights.push_back((coord, Direction::Left));
                }
                if let Some(coord) = coord.next(Direction::Right, shape) {
                    lights.push_back((coord, Direction::Right));
                }
            }
            ('/', Direction::Up) | ('\\', Direction::Down) => {
                if let Some(coord) = coord.next(Direction::Right, shape) {
                    lights.push_back((coord, Direction::Right));
                }
            }
            ('/', Direction::Down) | ('\\', Direction::Up) => {
                if let Some(coord) = coord.next(Direction::Left, shape) {
                    lights.push_back((coord, Direction::Left));
                }
            }
            ('/', Direction::Left) | ('\\', Direction::Right) => {
                if let Some(coord) = coord.next(Direction::Down, shape) {
                    lights.push_back((coord, Direction::Down));
                }
            }
            ('/', Direction::Right) | ('\\', Direction::Left) => {
                if let Some(coord) = coord.next(Direction::Up, shape) {
                    lights.push_back((coord, Direction::Up));
                }
            }
            _ => {}
        }
    }
    energized.len()
}

pub fn part1(input: &Input) -> usize {
    energized(&input.map, Coord((0, 0)), Direction::Right)
}

pub fn part2(input: &Input) -> usize {
    let shape = input.map.shape();
    (0..shape[0])
        .map(|y| (Coord((y, 0)), Direction::Right))
        .chain((0..shape[0]).map(|y| (Coord((y, shape[1] - 1)), Direction::Left)))
        .chain((0..shape[1]).map(|x| (Coord((0, x)), Direction::Down)))
        .chain((0..shape[1]).map(|x| (Coord((shape[0] - 1, x)), Direction::Up)))
        .map(|(coord, dir)| energized(&input.map, coord, dir))
        .max()
        .unwrap()
}
