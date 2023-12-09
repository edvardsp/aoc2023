use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct Input {
    map: Vec<Vec<u8>>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let mut map: Vec<Vec<u8>> = s
            .lines()
            .map(|line| {
                std::iter::once(b'.')
                    .chain(line.as_bytes().iter().copied())
                    .chain(std::iter::once(b'.'))
                    .collect()
            })
            .collect();
        let len = map[0].len();
        map.insert(0, vec![b'.'; len]);
        map.push(vec![b'.'; len]);
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

type Coord = (usize, usize);

fn neighbors(coord: &Coord) -> [Coord; 4] {
    [
        (coord.0 - 1, coord.1),
        (coord.0 + 1, coord.1),
        (coord.0, coord.1 - 1),
        (coord.0, coord.1 + 1),
    ]
}

fn traverse(
    map: &[Vec<u8>],
    mut curr: Coord,
    mut direction: Direction,
) -> Option<(Vec<(Coord, Direction)>, bool)> {
    let mut path = Vec::new();
    let mut rotation: isize = 0;
    loop {
        path.push((curr, direction));
        let tile = map[curr.0][curr.1];
        curr = match (tile, direction) {
            (b'S' | b'J' | b'L' | b'|', Direction::Up) => (curr.0 - 1, curr.1),
            (b'S' | b'F' | b'7' | b'|', Direction::Down) => (curr.0 + 1, curr.1),
            (b'S' | b'J' | b'7' | b'-', Direction::Left) => (curr.0, curr.1 - 1),
            (b'S' | b'L' | b'F' | b'-', Direction::Right) => (curr.0, curr.1 + 1),
            _ => return None,
        };

        let tile = map[curr.0][curr.1];
        direction = match (tile, direction) {
            (b'.', _) => return None,
            (b'S', _) => return Some((path, rotation > 0)),
            (b'|', _) => direction,
            (b'-', _) => direction,
            (b'J', Direction::Right) => {
                rotation -= 1;
                Direction::Up
            }
            (b'J', Direction::Down) => {
                rotation += 1;
                Direction::Left
            }
            (b'L', Direction::Down) => {
                rotation -= 1;
                Direction::Right
            }
            (b'L', Direction::Left) => {
                rotation += 1;
                Direction::Up
            }
            (b'F', Direction::Left) => {
                rotation -= 1;
                Direction::Down
            }
            (b'F', Direction::Up) => {
                rotation += 1;
                Direction::Right
            }
            (b'7', Direction::Up) => {
                rotation -= 1;
                Direction::Left
            }
            (b'7', Direction::Right) => {
                rotation += 1;
                Direction::Down
            }
            _ => return None,
        }
    }
}

fn explore(map: &[Vec<u8>], start: Coord) -> (Vec<(Coord, Direction)>, bool) {
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    .filter_map(|dir| traverse(map, start, *dir))
    .next()
    .unwrap()
}

pub fn part1(input: &Input) -> usize {
    let start = input
        .map
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, v)| ((r, c), v)))
        .find_map(|(coord, v)| if *v == b'S' { Some(coord) } else { None })
        .unwrap();

    let (path, _rotation) = explore(&input.map, start);
    path.len() / 2
}

pub fn part2(input: &Input) -> usize {
    let start = input
        .map
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, v)| ((r, c), v)))
        .find_map(|(coord, v)| if *v == b'S' { Some(coord) } else { None })
        .unwrap();

    let (path, rotation) = explore(&input.map, start);
    let on_path: HashSet<Coord> = path.iter().map(|(coord, _)| *coord).collect();
    let adjacent: HashSet<Coord> = path
        .iter()
        .flat_map(|(coord, direction)| {
            match (direction, rotation) {
                (Direction::Up, false) => [
                    (coord.0, coord.1 - 1),
                    (coord.0 - 1, coord.1 - 1),
                    (coord.0 - 1, coord.1),
                ],
                (Direction::Up, true) => [
                    (coord.0, coord.1 + 1),
                    (coord.0 - 1, coord.1 + 1),
                    (coord.0 - 1, coord.1),
                ],
                (Direction::Down, false) => [
                    (coord.0, coord.1 + 1),
                    (coord.0 + 1, coord.1 + 1),
                    (coord.0 + 1, coord.1),
                ],
                (Direction::Down, true) => [
                    (coord.0, coord.1 - 1),
                    (coord.0 + 1, coord.1 - 1),
                    (coord.0 + 1, coord.1),
                ],
                (Direction::Left, false) => [
                    (coord.0 + 1, coord.1),
                    (coord.0 + 1, coord.1 - 1),
                    (coord.0, coord.1 - 1),
                ],
                (Direction::Left, true) => [
                    (coord.0 - 1, coord.1),
                    (coord.0 - 1, coord.1 - 1),
                    (coord.0, coord.1 - 1),
                ],
                (Direction::Right, false) => [
                    (coord.0 - 1, coord.1),
                    (coord.0 - 1, coord.1 + 1),
                    (coord.0, coord.1 + 1),
                ],
                (Direction::Right, true) => [
                    (coord.0 + 1, coord.1),
                    (coord.0 + 1, coord.1 + 1),
                    (coord.0, coord.1 + 1),
                ],
            }
            .into_iter()
        })
        .filter(|coord| !on_path.contains(coord))
        .collect();

    let mut flooded: HashSet<Coord> = HashSet::new();
    let mut nest: HashSet<Coord> = HashSet::new();

    for curr in adjacent {
        if flooded.contains(&curr) {
            continue;
        }

        let mut flood: HashSet<Coord> = HashSet::new();
        let mut queue = VecDeque::from([curr]);
        while let Some(tile) = queue.pop_front() {
            if on_path.contains(&tile) {
                continue;
            }
            if flood.insert(tile) {
                queue.extend(neighbors(&tile).into_iter());
            }
        }

        flooded.extend(flood.iter());
        nest.extend(flood.iter());
    }

    nest.len()
}
