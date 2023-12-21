use ndarray::{s, Array2};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Coord((usize, usize));

impl Coord {
    fn go(&self, dir: Direction) -> (Self, Direction) {
        use Direction::*;
        let (y, x) = self.0;
        match dir {
            Up => (Self((y - 1, x)), dir),
            Down => (Self((y + 1, x)), dir),
            Left => (Self((y, x - 1)), dir),
            Right => (Self((y, x + 1)), dir),
        }
    }

    fn possibilities(&self, dir: Direction) -> impl Iterator<Item = (Self, Direction)> {
        use Direction::*;
        match dir {
            Up => [Left, Up, Right],
            Down => [Right, Down, Left],
            Left => [Up, Left, Down],
            Right => [Up, Right, Down],
        }
        .map(|dir| self.go(dir))
        .into_iter()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(value: char) -> Option<Self> {
        use Direction::*;
        match value {
            '^' => Some(Up),
            'v' => Some(Down),
            '<' => Some(Left),
            '>' => Some(Right),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Input {
    map: Map,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let width = lines.peek().unwrap().len();
        let height = lines.count();

        let map = s.lines().flat_map(str::chars).collect();
        let map = Map::new(Array2::from_shape_vec((height, width), map).unwrap());
        Self { map }
    }
}

#[derive(Debug)]
struct Map {
    map: Array2<char>,
    forrest: HashSet<Coord>,
}

impl Map {
    fn new(map: Array2<char>) -> Self {
        let forrest = map
            .indexed_iter()
            .filter_map(|(coord, c)| if *c == '#' { Some(coord) } else { None })
            .map(Coord)
            .collect();
        Self { map, forrest }
    }

    fn at(&self, coord: &Coord) -> char {
        self.map[coord.0]
    }

    fn start(&self) -> Coord {
        self.map
            .slice(s![0, ..])
            .indexed_iter()
            .find_map(|(x, c)| if *c == '.' { Some(x) } else { None })
            .map(|x| Coord((0, x)))
            .unwrap()
    }

    fn goal(&self) -> Coord {
        let height = self.map.dim().0;
        self.map
            .slice(s![height - 1, ..])
            .indexed_iter()
            .find_map(|(x, c)| if *c == '.' { Some(x) } else { None })
            .map(|x| Coord((height - 1, x)))
            .unwrap()
    }

    fn is_path(&self, coord: &Coord) -> bool {
        !self.forrest.contains(coord)
    }
}

fn traverse(map: &Map, start: Coord, goal: Coord) -> HashMap<Coord, HashSet<(Coord, usize)>> {
    let mut visited = HashSet::new();
    let mut vertices: HashMap<Coord, HashSet<(Coord, usize)>> = HashMap::new();
    let mut queue = VecDeque::from([(start, Direction::Down)]);
    while let Some((intersection, dir)) = queue.pop_front() {
        vertices.entry(intersection).or_default();
        let mut flood = VecDeque::from([(intersection.go(dir), 1)]);
        let mut directional = None;
        while let Some(((node, dir), steps)) = flood.pop_front() {
            if node == goal {
                vertices
                    .get_mut(&intersection)
                    .unwrap()
                    .insert((goal, steps));
                break;
            }
            visited.insert(node);

            if let Some(icy) = Direction::new(map.at(&node)) {
                directional = Some(icy == dir);
            }
            let next: Vec<_> = node
                .possibilities(dir)
                .filter(|(coord, _)| map.is_path(coord))
                .collect();
            if next.len() > 1 {
                match directional {
                    None => {
                        vertices
                            .get_mut(&intersection)
                            .unwrap()
                            .insert((node, steps));
                        vertices
                            .entry(node)
                            .or_default()
                            .insert((intersection, steps));
                    }
                    Some(false) => {
                        vertices
                            .entry(node)
                            .or_default()
                            .insert((intersection, steps));
                    }
                    Some(true) => {
                        vertices
                            .get_mut(&intersection)
                            .unwrap()
                            .insert((node, steps));
                    }
                }

                for (coord, dir) in next {
                    if visited.contains(&coord) {
                        continue;
                    }
                    queue.push_back((node, dir));
                }
                break;
            } else {
                flood.extend(next.into_iter().map(|next| (next, steps + 1)));
            }
        }
    }

    vertices
}

fn dp(
    graph: &HashMap<Coord, HashSet<(Coord, usize)>>,
    coord: &Coord,
    goal: &Coord,
    cost: usize,
    mut visited: HashSet<Coord>,
) -> Option<usize> {
    if coord == goal {
        return Some(cost);
    }

    visited.insert(*coord);

    let subtree = graph[coord]
        .iter()
        .filter(|(next, _)| !visited.contains(next))
        .flat_map(|(next, steps)| dp(graph, next, goal, *steps, visited.clone()))
        .max()?;

    Some(subtree + cost)
}

#[allow(dead_code)]
fn graphviz(vertices: &HashMap<Coord, HashSet<(Coord, usize)>>) {
    for (from, tos) in vertices.iter() {
        for (to, cost) in tos {
            println!("\"{:?}\" -> \"{:?}\" [label=\"{cost}\"]", from.0, to.0);
        }
    }
}

pub fn part1(input: &Input) -> usize {
    let start: Coord = input.map.start();
    let goal: Coord = input.map.goal();
    let vertices = traverse(&input.map, start, goal);

    dp(&vertices, &start, &goal, 0, HashSet::new()).unwrap()
}

pub fn part2(input: &Input) -> usize {
    let start: Coord = input.map.start();
    let goal: Coord = input.map.goal();
    let mut vertices = traverse(&input.map, start, goal);

    let nodes: Vec<_> = vertices.keys().copied().collect();
    for from in nodes {
        let tos = vertices[&from].clone();
        for (to, cost) in tos {
            vertices.entry(to).or_default().insert((from, cost));
        }
    }

    dp(&vertices, &start, &goal, 0, HashSet::new()).unwrap()
}
