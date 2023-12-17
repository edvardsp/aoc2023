use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Input {
    steps1: Vec<Step>,
    steps2: Vec<Step>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let (steps1, steps2) = s
            .lines()
            .map(|line| {
                let mut tokens = line.split_ascii_whitespace();
                let step1 = Step {
                    dir: tokens.next().unwrap().into(),
                    len: tokens.next().unwrap().parse().unwrap(),
                };

                let tail = tokens.next().unwrap();
                let step2 = Step {
                    dir: tail[7..8].into(),
                    len: isize::from_str_radix(&tail[2..7], 16).unwrap(),
                };
                (step1, step2)
            })
            .unzip();
        Self { steps1, steps2 }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" | "3" => Direction::Up,
            "D" | "1" => Direction::Down,
            "L" | "2" => Direction::Left,
            "R" | "0" => Direction::Right,
            _ => panic!("invalid value for Direction: {}", s),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Step {
    dir: Direction,
    len: isize,
}

#[derive(Debug, PartialEq, Eq)]
enum Edge {
    Up(isize),
    Down(isize),
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Edge::*;
        let lhs = match self {
            Up(n) | Down(n) => *n..=*n,
        };
        let rhs = match other {
            Up(n) | Down(n) => *n..=*n,
        };
        lhs.cmp(rhs)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn row_sum(edges: &[Edge]) -> usize {
    let mut x = 0;
    edges.iter().fold(0, |mut summa, edge| {
        match edge {
            Edge::Up(n) => x = *n,
            Edge::Down(n) => summa += ((*n + 1) - x) as usize,
        }
        summa
    })
}

fn interior(steps: &[Step]) -> usize {
    let mut turns = HashMap::new();
    let mut yturns = HashSet::new();
    let mut up = Vec::new();
    let mut down = Vec::new();
    let mut pos = (0, 0);
    for (curr, next) in steps.iter().zip(steps.iter().cycle().skip(1)) {
        match curr.dir {
            Direction::Up => {
                up.push((pos.0, pos.1 - curr.len + 1..pos.1));
                pos.1 -= curr.len;
                if next.dir == Direction::Right {
                    turns
                        .entry(pos.1)
                        .or_insert(Vec::new())
                        .push(Edge::Up(pos.0));
                }
            }
            Direction::Down => {
                down.push((pos.0, pos.1 + 1..pos.1 + curr.len));
                pos.1 += curr.len;
                if next.dir == Direction::Left {
                    turns
                        .entry(pos.1)
                        .or_insert(Vec::new())
                        .push(Edge::Down(pos.0));
                }
            }
            Direction::Left => {
                yturns.insert(pos.1);
                pos.0 -= curr.len;
                if next.dir == Direction::Up {
                    turns
                        .entry(pos.1)
                        .or_insert(Vec::new())
                        .push(Edge::Up(pos.0));
                }
            }
            Direction::Right => {
                yturns.insert(pos.1);
                pos.0 += curr.len;
                if next.dir == Direction::Down {
                    turns
                        .entry(pos.1)
                        .or_insert(Vec::new())
                        .push(Edge::Down(pos.0));
                }
            }
        }
    }

    for y in &yturns {
        for (x, yr) in &up {
            if yr.contains(y) {
                turns.entry(*y).or_insert(Vec::new()).push(Edge::Up(*x));
            }
        }

        for (x, yr) in &down {
            if yr.contains(y) {
                turns.entry(*y).or_insert(Vec::new()).push(Edge::Down(*x));
            }
        }
    }

    let mut summa = 0;
    for turn in turns.values_mut() {
        turn.sort();
        summa += row_sum(turn.as_slice());
    }

    let mut yturns: Vec<_> = yturns.into_iter().collect();
    yturns.sort();
    for inbetween in yturns.windows(2).map(|ys| ys[0] + 1..ys[1]) {
        let up_iter = up
            .iter()
            .filter(|(_, yr)| yr.start <= inbetween.start && inbetween.end <= yr.end)
            .map(|(x, _)| Edge::Up(*x));
        let down_iter = down
            .iter()
            .filter(|(_, yr)| yr.start <= inbetween.start && inbetween.end <= yr.end)
            .map(|(x, _)| Edge::Down(*x));

        let mut edges: Vec<_> = up_iter.chain(down_iter).collect();
        edges.sort();

        summa += row_sum(edges.as_slice()) * inbetween.len();
    }

    summa
}

pub fn part1(input: &Input) -> usize {
    interior(&input.steps1)
}

pub fn part2(input: &Input) -> usize {
    interior(&input.steps2)
}
