use ndarray::{s, Array2, Axis, Slice};
use std::ops::Sub;

#[derive(Debug)]
pub struct Input {
    mirrors: Vec<Mirror>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let s = s.replace("\r\n", "\n");
        let mirrors = s.split("\n\n").map(Mirror::from).collect();
        Self { mirrors }
    }
}

#[derive(Clone, Debug)]
struct Mirror(Array2<char>);

impl From<&str> for Mirror {
    fn from(s: &str) -> Self {
        let width: usize = s.lines().next().unwrap().len();
        let map: Vec<char> = s.lines().flat_map(|line| line.chars()).collect();
        let height: usize = map.len() / width;
        let map = Array2::from_shape_vec((height, width), map).unwrap();
        Self(map)
    }
}

impl Mirror {
    fn reflections(&self, axis: Axis) -> usize {
        let axis_iter = self.0.axis_iter(axis);
        let axis_iter2 = axis_iter.clone();

        axis_iter
            .zip(axis_iter2.skip(1))
            .enumerate()
            .filter_map(|(i, (l, r))| if l == r { Some(i as isize) } else { None })
            .find(|&ind| {
                let up = ind;

                let down = self.0.shape()[axis.0] as isize - 1 - ind - 1;
                let range = up.min(down);

                let up_slice = Slice::new(ind - range, Some(ind + 1), -1);
                let down_slice = Slice::new(ind + 1, Some(ind + 1 + range + 1), 1);

                self.0
                    .slice_axis(axis, up_slice)
                    .axis_iter(axis)
                    .zip(self.0.slice_axis(axis, down_slice).axis_iter(axis))
                    .all(|(l, r)| l == r)
            })
            .map(|ind| (ind + 1) as usize)
            .unwrap_or(0)
    }

    fn smudge(&self) -> usize {
        let smudge: Array2<isize> = self.0.map(|&c| if c == '#' { 1 } else { 0 });
        let shape = smudge.shape();
        let rows = shape[0];
        let cols = shape[1];

        for r in 0..(rows - 1) {
            let range = (r + 1).min(rows - r - 1);
            let up_range = r.saturating_sub(range - 1)..(r + 1);
            let down_range = (r + 1)..(r + 1 + range).min(rows);

            let up = smudge.slice(s![up_range;-1,..]);
            let down = smudge.slice(s![down_range, ..]);
            let diff = up.sub(&down);
            if diff.iter().filter(|&&v| v != 0).count() == 1 {
                return (r + 1) * 100;
            }
        }

        for c in 0..(cols - 1) {
            let range = (c + 1).min(cols - c - 1);
            let up_range = c.saturating_sub(range - 1)..(c + 1);
            let down_range = (c + 1)..(c + 1 + range).min(cols);

            let up = smudge.slice(s![..,up_range;-1]);
            let down = smudge.slice(s![.., down_range]);
            let diff = up.sub(&down);
            if diff.iter().filter(|&&v| v != 0).count() == 1 {
                return c + 1;
            }
        }

        unimplemented!()
    }
}

pub fn part1(input: &Input) -> usize {
    input
        .mirrors
        .iter()
        .map(|mirror| mirror.reflections(Axis(0)) * 100 + mirror.reflections(Axis(1)))
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input.mirrors.iter().map(|mirror| mirror.smudge()).sum()
}
