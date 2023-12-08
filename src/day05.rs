use std::ops::Range;

#[derive(Debug)]
pub struct Input {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let input = s.replace("\r\n", "\n");
        let mut lines = input.split("\n\n");
        let seeds = lines
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let maps = lines.map(Map::from).collect();

        Self { seeds, maps }
    }
}

#[derive(Debug)]
struct Map {
    conversions: Vec<Conversion>,
}

#[derive(Debug)]
struct Conversion {
    dst: Range<usize>,
    src: Range<usize>,
}

impl Conversion {
    fn contains(&self, seed: usize) -> bool {
        self.src.start <= seed && seed < self.src.end
    }

    fn convert(&self, seed: usize) -> usize {
        self.dst.start + (seed - self.src.start)
    }

    fn overlaps(&self, other: &Range<usize>) -> bool {
        self.src.start < other.end && other.start < self.src.end
    }

    fn split(&self, other: &Range<usize>) -> (Option<Range<usize>>, Vec<Range<usize>>) {
        if self.overlaps(other) {
            let mut parts = Vec::new();
            let left = other.start..self.src.start.min(other.end);
            let mid = self.src.start.max(other.start)..self.src.end.min(other.end);
            let right = self.src.end.max(other.start)..other.end;

            assert!(!mid.is_empty());
            let converted = (mid.start + self.dst.start - self.src.start)
                ..(mid.end + self.dst.start - self.src.start);

            if !left.is_empty() {
                parts.push(left);
            }
            if !right.is_empty() {
                parts.push(right);
            }

            (Some(converted), parts)
        } else {
            (None, vec![other.clone()])
        }
    }
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();
        let _name = lines.next().unwrap();
        let conversions = lines
            .map(|line| {
                let mut parts = line.split_whitespace();
                let dest_range_start = parts.next().unwrap().parse().unwrap();
                let src_range_start = parts.next().unwrap().parse().unwrap();
                let range_len: usize = parts.next().unwrap().parse().unwrap();
                Conversion {
                    dst: dest_range_start..dest_range_start + range_len,
                    src: src_range_start..src_range_start + range_len,
                }
            })
            .collect();
        Self { conversions }
    }
}

impl Map {
    fn convert(&self, seeds: &[usize]) -> Vec<usize> {
        seeds
            .iter()
            .map(|&seed| {
                self.conversions
                    .iter()
                    .find(|conversion| conversion.contains(seed))
                    .map(|conversion| conversion.convert(seed))
                    .unwrap_or(seed)
            })
            .collect()
    }

    fn split(&self, seeds: &[Range<usize>]) -> Vec<Range<usize>> {
        let mut result = Vec::new();
        for seed in seeds {
            let mut seed_parts = vec![seed.clone()];
            for conv in &self.conversions {
                seed_parts = seed_parts
                    .into_iter()
                    .flat_map(|part| {
                        let (converted, parts) = conv.split(&part);
                        if let Some(converted) = converted {
                            result.push(converted);
                        }
                        parts
                    })
                    .collect();
            }
            result.extend(seed_parts);
        }
        result
    }
}

pub fn part1(input: &Input) -> usize {
    input
        .maps
        .iter()
        .fold(input.seeds.clone(), |seeds, map| map.convert(&seeds))
        .into_iter()
        .min()
        .unwrap()
}

pub fn part2(input: &Input) -> usize {
    let seeds: Vec<_> = input
        .seeds
        .chunks(2)
        .map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .collect();
    input
        .maps
        .iter()
        .fold(seeds, |seeds, map: &Map| map.split(&seeds))
        .into_iter()
        .min_by(|lhs, rhs| lhs.start.cmp(&rhs.start))
        .map(|range| range.start)
        .unwrap()
}
