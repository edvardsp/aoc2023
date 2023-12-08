const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Debug)]
pub struct Input {
    lines: Vec<Vec<u8>>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let lines = s.lines().map(|line| line.as_bytes().to_vec()).collect();
        Self { lines }
    }
}

fn is_digit(c: &u8) -> bool {
    b'0' <= *c && *c <= b'9'
}

fn to_digit(c: u8) -> usize {
    (c - b'0') as usize
}

struct DigitIterator<'b> {
    bytes: &'b [u8],
    cursor: usize,
    part_two: bool,
}

impl<'b> DigitIterator<'b> {
    fn new(bytes: &'b [u8], part_two: bool) -> Self {
        Self {
            bytes,
            cursor: 0,
            part_two,
        }
    }
}

impl<'b> Iterator for DigitIterator<'b> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.bytes.len() {
            return None;
        }

        for i in self.cursor..self.bytes.len() {
            if is_digit(&self.bytes[i]) {
                self.cursor = i + 1;
                return Some(to_digit(self.bytes[i]));
            }

            if self.part_two {
                for (d, digit) in DIGITS.iter().enumerate() {
                    if self.bytes[i..].starts_with(digit.as_bytes()) {
                        self.cursor = i + 1;
                        return Some(d + 1);
                    }
                }
            }
        }

        None
    }
}

pub fn part1(input: &Input) -> usize {
    input
        .lines
        .iter()
        .map(|line| DigitIterator::new(line, false))
        .map(|mut line| {
            let first = line.next().unwrap();
            let last = line.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
    input
        .lines
        .iter()
        .map(|line| DigitIterator::new(line, true))
        .map(|mut line| {
            let first = line.next().unwrap();
            let last = line.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}
