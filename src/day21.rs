use std::collections::HashSet;

use ndarray::Array2;

#[derive(Debug)]
pub struct Input {
    map: Array2<char>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();
        let map: Vec<_> = s.lines().flat_map(|line| line.chars()).collect();
        let map = Array2::from_shape_vec((height, width), map).unwrap();
        Self { map }
    }
}

fn manhatten_distance(lhs: &(isize, isize), rhs: &(isize, isize)) -> usize {
    lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1)
}

fn flood(map: &Array2<char>, max_steps: usize) -> usize {
    let (height, width) = map.dim();
    let rocks: HashSet<_> = map
        .indexed_iter()
        .filter_map(|(coord, c)| if *c == '#' { Some(coord) } else { None })
        .collect();
    let start = map
        .indexed_iter()
        .find_map(|(coord, c)| if *c == 'S' { Some(coord) } else { None })
        .map(|(y, x)| (y as isize, x as isize))
        .unwrap();

    let mut visited = HashSet::new();
    let mut check = vec![start];
    for _ in 0..=max_steps {
        let from = check.clone();
        check.clear();
        for (y, x) in from {
            if !visited.insert((y, x)) {
                continue;
            }
            for to in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
                let normalized = (
                    to.0.rem_euclid(height as isize) as usize,
                    to.1.rem_euclid(width as isize) as usize,
                );
                if !rocks.contains(&normalized) {
                    check.push(to);
                }
            }
        }
    }

    let modal = max_steps % 2;
    visited
        .into_iter()
        .filter(|coord| manhatten_distance(&start, coord) % 2 == modal)
        .count()
}

pub fn part1(input: &Input) -> usize {
    flood(&input.map, 64)
}

pub fn part2(input: &Input) -> usize {
    const STEPS: usize = 26501365;
    let dim = input.map.dim();
    let width: usize = dim.1;
    let half_width = width / 2;

    // Taken from: https://github.com/goggle/AdventOfCode2023.jl/blob/main/src/day21.jl#L44
    let r1 = flood(&input.map, half_width) as isize;
    let r2 = flood(&input.map, half_width + width) as isize;
    let r3 = flood(&input.map, half_width + 2 * width) as isize;

    let n = ((STEPS - half_width) / width) as isize;

    let a = (r3 - 2 * r2 + r1) / 2;
    let b = (4 * r2 - 3 * r1 - r3) / 2;
    let c = r1;

    (a * n * n + b * n + c) as usize
}
