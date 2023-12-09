use ndarray::{Array2, Axis};

#[derive(Debug)]
pub struct Input {
    map: Map,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let width: usize = s.lines().next().unwrap().len();
        let map: Vec<char> = s.lines().flat_map(|line| line.chars()).collect();
        let height: usize = map.len() / width;
        let map = Array2::from_shape_vec((height, width), map).unwrap();
        Self { map }
    }
}

type Coord = (usize, usize);
type Map = Array2<char>;

fn manhatten_distance(x: Coord, y: Coord) -> usize {
    x.0.abs_diff(y.0) + x.1.abs_diff(y.1)
}

fn empty_lanes(map: &Map, axis: Axis) -> Vec<usize> {
    map.axis_iter(axis)
        .enumerate()
        .filter_map(|(r, row)| {
            if row.into_iter().all(|v| *v == '.') {
                Some(r)
            } else {
                None
            }
        })
        .collect()
}

fn expand_universe(map: &Map, n: usize) -> usize {
    let empty_rows = empty_lanes(map, Axis(0));
    let empty_cols = empty_lanes(map, Axis(1));

    let galaxies: Vec<_> = map
        .indexed_iter()
        .filter_map(|(coord, c)| if *c == '#' { Some(coord) } else { None })
        .map(|coord| {
            let expand_row = empty_rows.iter().take_while(|r| **r < coord.0).count();
            let expand_col = empty_cols.iter().take_while(|c| **c < coord.1).count();
            (
                coord.0 + expand_row * (n - 1),
                coord.1 + expand_col * (n - 1),
            )
        })
        .collect();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, coord0)| {
            galaxies[i + 1..]
                .iter()
                .map(move |coord1| (*coord0, *coord1))
        })
        .map(|(coord0, coord1)| manhatten_distance(coord0, coord1))
        .sum()
}

pub fn part1(input: &Input) -> usize {
    expand_universe(&input.map, 2)
}

pub fn part2(input: &Input) -> usize {
    expand_universe(&input.map, 1_000_000)
}
