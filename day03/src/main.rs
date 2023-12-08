use std::collections::{HashMap, HashSet};
use std::io::Read;

#[derive(Debug)]
struct Input {
    map: Vec<Vec<u8>>,
}

impl Input {
    fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open("input.txt")?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        let map = input.lines().map(|line| line.as_bytes().to_vec()).collect();

        Ok(Self { map })
    }
}

struct NumberPartIter<'b> {
    row: &'b [u8],
    cursor: usize,
    item: Option<((usize, usize), usize)>,
}

impl<'b> NumberPartIter<'b> {
    fn new(row: &'b [u8]) -> Self {
        Self {
            row,
            cursor: 0,
            item: None,
        }
    }
}

impl<'b> Iterator for NumberPartIter<'b> {
    type Item = ((usize, usize), usize);

    fn next(&mut self) -> Option<Self::Item> {
        for x in self.cursor..self.row.len() {
            match self.row[x] {
                n @ b'0'..=b'9' => {
                    let n = (n - b'0') as usize;
                    if let Some(item) = self.item.as_mut() {
                        item.0 .1 = x;
                        item.1 = item.1 * 10 + n;
                    } else {
                        self.item = Some(((x, x), n));
                    }
                }
                _ => {
                    let item = self.item.take();
                    if item.is_some() {
                        self.cursor = x + 1;
                        return item;
                    }
                }
            }
        }
        self.cursor = self.row.len();
        self.item.take()
    }
}

fn part1(input: &Input) -> usize {
    let ymax = input.map.len();
    let xmax = input.map[0].len();

    let mut symbols = HashSet::new();
    for (y, row) in input.map.iter().enumerate() {
        for (x, &symbol) in row.iter().enumerate() {
            if symbol != b'.' && !(symbol >= b'0' && symbol <= b'9') {
                symbols.insert((y, x));
            }
        }
    }

    let mut summa = 0;
    for (y, row) in input.map.iter().enumerate() {
        for (coords, number) in NumberPartIter::new(row) {
            let ystart = y.saturating_sub(1);
            let yend = (y + 1).min(ymax - 1);
            let xstart = coords.0.saturating_sub(1);
            let xend = (coords.1 + 1).min(xmax - 1);
            for yi in ystart..=yend {
                for xi in xstart..=xend {
                    if symbols.contains(&(yi, xi)) {
                        summa += number;
                    }
                }
            }
        }
    }

    summa
}

fn part2(input: &Input) -> usize {
    let ymax = input.map.len();
    let xmax = input.map[0].len();

    let mut gears = HashMap::new();
    for (y, row) in input.map.iter().enumerate() {
        for (x, &symbol) in row.iter().enumerate() {
            if symbol == b'*' {
                gears.insert((y, x), Vec::new());
            }
        }
    }

    for (y, row) in input.map.iter().enumerate() {
        for (coords, number) in NumberPartIter::new(row) {
            let ystart = y.saturating_sub(1);
            let yend = (y + 1).min(ymax - 1);
            let xstart = coords.0.saturating_sub(1);
            let xend = (coords.1 + 1).min(xmax - 1);
            for yi in ystart..=yend {
                for xi in xstart..=xend {
                    if gears.contains_key(&(yi, xi)) {
                        gears.get_mut(&(yi, xi)).unwrap().push(number);
                    }
                }
            }
        }
    }

    gears
        .values()
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0] * numbers[1])
        .sum()
}

fn main() {
    let input = Input::parse().unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
