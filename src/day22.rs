use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Input {
    bricks: Vec<Brick>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let bricks = s
            .lines()
            .enumerate()
            .map(|(id, line)| Brick::from_str(id, line))
            .collect();
        Self { bricks }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Brick {
    id: usize,
    x: Range,
    y: Range,
    z: Range,
}

impl Brick {
    fn from_str(id: usize, s: &str) -> Self {
        let (from, to) = s.split_once('~').unwrap();
        let mut iter = from
            .split(',')
            .map(|n| n.parse().unwrap())
            .zip(to.split(',').map(|n| n.parse().unwrap()))
            .map(|(f, t)| Range { start: f, end: t });

        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        let z = iter.next().unwrap();
        Self { id, x, y, z }
    }

    fn fall(&self) -> Option<Self> {
        if self.z.start > 1 {
            let mut new = *self;
            new.z.start -= 1;
            new.z.end -= 1;
            Some(new)
        } else {
            None
        }
    }

    fn support(&self) -> Self {
        let mut new = *self;
        new.z.start = self.z.end + 1;
        new.z.end = self.z.end + 1;
        new
    }

    fn collides(&self, other: &Self) -> bool {
        self.x.overlaps(&other.x) && self.y.overlaps(&other.y) && self.z.overlaps(&other.z)
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z
            .start
            .cmp(&other.z.start)
            .then_with(|| self.y.start.cmp(&other.y.start))
            .then_with(|| self.x.start.cmp(&other.x.start))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct SupportState {
    supports: HashMap<usize, HashSet<usize>>,
    supported_by: HashMap<usize, HashSet<usize>>,
}

impl SupportState {
    fn bricks(&self) -> impl Iterator<Item = &usize> {
        self.supports.keys()
    }

    fn disintegrate(&self, brick: usize) -> bool {
        self.supports[&brick]
            .iter()
            .all(|above| self.supported_by[above].len() > 1)
    }
}

fn apply_gravity(mut bricks: Vec<Brick>) -> SupportState {
    let mut falling = true;
    while falling {
        bricks.sort();
        falling = false;
        for i in 0..bricks.len() {
            if let Some(fall) = bricks[i].fall() {
                if (0..bricks.len())
                    .filter(|&j| j != i)
                    .take_while(|&j| fall.z.end >= bricks[j].z.start)
                    .all(|j| !fall.collides(&bricks[j]))
                {
                    bricks[i] = fall;
                    falling = true;
                }
            }
        }
    }

    let mut supports: HashMap<_, _> = (0..bricks.len()).map(|i| (i, HashSet::new())).collect();
    let mut supported_by: HashMap<_, _> = (0..bricks.len()).map(|i| (i, HashSet::new())).collect();
    for i in 0..bricks.len() {
        let support = bricks[i].support();
        for j in (0..bricks.len())
            .filter(|&j| j != i)
            .filter(|&j| bricks[j].collides(&support))
        {
            supports.get_mut(&support.id).unwrap().insert(bricks[j].id);
            supported_by
                .get_mut(&bricks[j].id)
                .unwrap()
                .insert(support.id);
        }
    }

    SupportState {
        supports,
        supported_by,
    }
}

pub fn part1(input: &Input) -> usize {
    let state = apply_gravity(input.bricks.clone());

    state
        .bricks()
        .filter(|&&brick| state.disintegrate(brick))
        .count()
}

fn check_chain(state: &SupportState, brick: usize) -> usize {
    let mut removed = HashSet::from([brick]);
    let mut check = VecDeque::from([brick]);
    while let Some(other) = check.pop_front() {
        let mut inner = HashSet::new();

        for above in state.supports[&other]
            .iter()
            .filter(|above| !removed.contains(above))
            .filter(|above| state.supported_by[above].difference(&removed).count() == 0)
        {
            inner.insert(*above);
            check.push_back(*above);
        }

        removed.extend(inner);
    }

    removed.len() - 1
}

pub fn part2(input: &Input) -> usize {
    let state = apply_gravity(input.bricks.clone());

    state
        .bricks()
        .map(|&brick| check_chain(&state, brick))
        .sum()
}
