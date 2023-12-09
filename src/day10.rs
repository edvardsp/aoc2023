#![allow(clippy::reversed_empty_ranges)] 

use std::collections::HashSet;
use ndarray::{s, Array2};

#[derive(Debug)]
pub struct Input {
    map: Map,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let width: usize = s.lines().next().unwrap().len();
        let inner_map: Vec<char> = s
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| match c {
                'F' => '┌',
                'J' => '┘',
                'L' => '└',
                '7' => '┐',
                c => c,
            })
            .collect();
        let height = inner_map.len() / width;
        let inner_map = Array2::from_shape_vec((height, width), inner_map).unwrap();
        let mut map = Array2::from_elem((height + 2, width + 2), '.');
        map.slice_mut(s![1..-1, 1..-1]).assign(&inner_map);
        Self { map }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
    }
}

type Map = Array2<char>;
type Coord = (usize, usize);

fn traverse(map: &Map, mut curr: Coord, mut direction: Direction) -> Option<(Vec<Coord>, char)> {
    let first_direction = direction;
    let mut path = Vec::new();
    'outer: loop {
        path.push(curr);

        let tile = map[curr];
        curr = match (tile, direction) {
            ('S' | '┘' | '└' | '|', Direction::Up) => (curr.0 - 1, curr.1),
            ('S' | '┌' | '┐' | '|', Direction::Down) => (curr.0 + 1, curr.1),
            ('S' | '┘' | '┐' | '-', Direction::Left) => (curr.0, curr.1 - 1),
            ('S' | '└' | '┌' | '-', Direction::Right) => (curr.0, curr.1 + 1),
            _ => return None,
        };

        let tile = map[curr];
        direction = match (tile, direction) {
            ('.', _) => return None,
            ('S', _) => break 'outer,
            ('|', _) => direction,
            ('-', _) => direction,
            ('┘', Direction::Right) => Direction::Up,
            ('┘', Direction::Down) => Direction::Left,
            ('└', Direction::Down) => Direction::Right,
            ('└', Direction::Left) => Direction::Up,
            ('┌', Direction::Left) => Direction::Down,
            ('┌', Direction::Up) => Direction::Right,
            ('┐', Direction::Up) => Direction::Left,
            ('┐', Direction::Right) => Direction::Down,
            combo => unreachable!("invalid combination: {:?}", combo),
        }
    }

    let piece = match (direction, first_direction) {
        (Direction::Up, Direction::Up) | (Direction::Down, Direction::Down) => '|',
        (Direction::Left, Direction::Left) | (Direction::Right, Direction::Right) => '-',
        (Direction::Right, Direction::Up) | (Direction::Down, Direction::Left) => '┘',
        (Direction::Up, Direction::Right) | (Direction::Left, Direction::Down) => '┌',
        (Direction::Down, Direction::Right) | (Direction::Left, Direction::Up) => '└',
        (Direction::Up, Direction::Left) | (Direction::Right, Direction::Down) => '┐',
        combo => unreachable!("invalid combination: {:?}", combo),
    };

    Some((path, piece))
}

fn explore(map: &Map, start: Coord) -> (Vec<Coord>, char) {
    Direction::iter()
        .filter_map(|dir| traverse(map, start, dir))
        .next()
        .unwrap()
}

pub fn part1(input: &Input) -> usize {
    let start = input
        .map
        .indexed_iter()
        .find_map(|(coord, c)| if *c == 'S' { Some(coord) } else { None })
        .unwrap();

    let (path, _) = explore(&input.map, start);
    path.len() / 2
}

pub fn part2(input: &Input) -> usize {
    let start = input
        .map
        .indexed_iter()
        .find_map(|(coord, c)| if *c == 'S' { Some(coord) } else { None })
        .unwrap();

    let (path, start_tile) = explore(&input.map, start);
    let path: HashSet<Coord> = path.into_iter().collect();

    let mut map = input.map.clone();
    map[start] = start_tile;

    let mut total_inside = 0;
    for (r, row) in map.rows().into_iter().enumerate() {
        let mut inside = false;
        for (c, tile) in row.iter().enumerate() {
            let coord = (r, c);
            match *tile {
                '|' | '┌' | '┐' if path.contains(&coord) => inside = !inside,
                _ if inside && !path.contains(&coord) => total_inside += 1,
                _ => {}
            }
        }
    }

    total_inside
}
