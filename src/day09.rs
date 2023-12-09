#[derive(Debug)]
pub struct Input {
    histories: Vec<History>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let histories = s.lines().map(History::from).collect();
        Self { histories }
    }
}

#[derive(Debug)]
struct History {
    num: Vec<isize>,
}

impl From<&str> for History {
    fn from(value: &str) -> Self {
        let num = value
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Self { num }
    }
}

impl History {
    fn difference(&self) -> Self {
        let num = self.num.windows(2).map(|w| w[1] - w[0]).collect();
        Self { num }
    }

    fn forward(&self) -> isize {
        if self.num.iter().all(|&n| n == 0) {
            return 0;
        }

        self.num.last().unwrap() + self.difference().forward()
    }

    fn backward(&self) -> isize {
        if self.num.iter().all(|&n| n == 0) {
            return 0;
        }

        self.num.first().unwrap() - self.difference().backward()
    }
}

pub fn part1(input: &Input) -> isize {
    input.histories.iter().map(History::forward).sum()
}

pub fn part2(input: &Input) -> isize {
    input.histories.iter().map(History::backward).sum()
}
