use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    directions: Vec<Direction>,
    map: HashMap<Id, Node>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();
        let directions = lines
            .next()
            .unwrap()
            .as_bytes()
            .iter()
            .copied()
            .map(Direction::from)
            .collect();
        lines.next().unwrap();
        let map = lines
            .map(|line| {
                let (id, lr) = line.split_once(" = ").unwrap();
                let id = id.into();
                let left = lr[1..4].into();
                let right = lr[6..9].into();
                (id, Node { left, right })
            })
            .collect();

        Self { directions, map }
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl From<u8> for Direction {
    fn from(direction: u8) -> Self {
        match direction {
            b'L' => Self::Left,
            b'R' => Self::Right,
            _ => panic!("invalid direction"),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Id {
    id: usize,
    start: bool,
    end: bool,
}

impl From<&str> for Id {
    fn from(id: &str) -> Self {
        let start = id.ends_with('A');
        let end = id.ends_with('Z');
        // Since each ID is 3 chars, a simple hash function is to treat the ID as a 3 byte long ID.
        let id = id.chars().fold(0, |acc, c| acc * 0x100 + c as usize);
        Self { id, start, end }
    }
}

#[derive(Clone, Debug)]
struct Node {
    left: Id,
    right: Id,
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b %= a;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn traverse(input: &Input, start: Id, pred: impl Fn(&Id) -> bool) -> usize {
    input
        .directions
        .iter()
        .cycle()
        .scan(start, move |state, direction| {
            if pred(state) {
                return None;
            }

            let node = input.map.get(state).unwrap();
            *state = match direction {
                Direction::Left => node.left,
                Direction::Right => node.right,
            };

            Some(())
        })
        .count()
}

pub fn part1(input: &Input) -> usize {
    let start: Id = "AAA".into();
    let end: Id = "ZZZ".into();
    traverse(input, start, |id| id == &end)
}

pub fn part2(input: &Input) -> usize {
    input
        .map
        .keys()
        .filter(|id| id.start)
        .map(|start| traverse(input, *start, |id| id.end))
        .fold(1, lcm)
}
