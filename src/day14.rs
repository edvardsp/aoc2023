use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    map: Map,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let map = Map::from(s);
        Self { map }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let map: Vec<Vec<_>> = s.lines().map(|line| line.chars().collect()).collect();
        let width = map[0].len();
        let height = map.len();
        Self { map, width, height }
    }
}

impl Map {
    fn tilt_north(&mut self) {
        for x in 0..self.width {
            let mut cursor = 0;
            for y in 0..self.height {
                match self.map[y][x] {
                    'O' => {
                        self.map[y][x] = '.';
                        self.map[cursor][x] = 'O';
                        cursor += 1;
                    }
                    '#' => cursor = y + 1,
                    _ => {}
                }
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.height {
            let mut cursor = 0;
            for x in 0..self.width {
                match self.map[y][x] {
                    'O' => {
                        self.map[y][x] = '.';
                        self.map[y][cursor] = 'O';
                        cursor += 1;
                    }
                    '#' => cursor = x + 1,
                    _ => {}
                }
            }
        }
    }

    fn tilt_south(&mut self) {
        for x in 0..self.width {
            let mut cursor = self.height - 1;
            for y in (0..self.height).rev() {
                match self.map[y][x] {
                    'O' => {
                        self.map[y][x] = '.';
                        self.map[cursor][x] = 'O';
                        cursor = cursor.saturating_sub(1);
                    }
                    '#' => cursor = y.saturating_sub(1),
                    _ => {}
                }
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.height {
            let mut cursor = self.width - 1;
            for x in (0..self.width).rev() {
                match self.map[y][x] {
                    'O' => {
                        self.map[y][x] = '.';
                        self.map[y][cursor] = 'O';
                        cursor = cursor.saturating_sub(1);
                    }
                    '#' => cursor = x.saturating_sub(1),
                    _ => {}
                }
            }
        }
    }

    fn load(&self) -> usize {
        let mut summa = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                if self.map[y][x] == 'O' {
                    summa += self.height - y;
                }
            }
        }
        summa
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }
}

pub fn part1(input: &Input) -> usize {
    let mut map = input.map.clone();
    map.tilt_north();
    map.load()
}

pub fn part2(input: &Input) -> usize {
    let mut map = input.map.clone();
    let mut sequence = Vec::new();
    let mut states = HashMap::new();
    for i in 0.. {
        map.cycle();
        sequence.push(map.clone());
        if let Some(n) = states.insert(map.clone(), i) {
            let period = i - n;
            let result = (1_000_000_000 - n) % period + n - 1;
            return sequence[result].load();
        }
    }
    0
}
