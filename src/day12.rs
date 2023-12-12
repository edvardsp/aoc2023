#[derive(Debug)]
pub struct Input {
    springs: Vec<Spring>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let springs = s.lines().map(Spring::from).collect();
        Self { springs }
    }
}

#[derive(Debug)]
struct Spring {
    condition: Vec<char>,
    groups: Vec<usize>,
}

impl From<&str> for Spring {
    fn from(s: &str) -> Self {
        let (condition, groups) = s.split_once(' ').unwrap();
        let condition = condition.chars().collect();
        let groups = groups.split(',').map(|n| n.parse().unwrap()).collect();
        Self { condition, groups }
    }
}

#[memoize::memoize]
fn matches(springs: Vec<char>, groups: Vec<usize>) -> Option<usize> {
    if let Some((&n, groups_left)) = groups.split_first() {
        let token = if groups_left.is_empty() {
            "#".repeat(n)
        } else {
            format!("{}.", "#".repeat(n))
        };

        if token.len() > springs.len() {
            return None;
        }

        let mut summa = 0;
        for i in 0..=(springs.len() - token.len()) {
            if springs[..i].iter().any(|&c| c == '#') {
                break;
            }
            let (head, tail) = springs[i..].split_at(token.len());

            if !head
                .iter()
                .zip(token.chars())
                .all(|(&l, r)| l == '?' || l == r)
            {
                continue;
            }

            match matches(tail.to_owned(), groups_left.to_owned()) {
                Some(m) => summa += m,
                None => continue,
            }
        }
        Some(summa)
    } else if springs.iter().any(|&c| c == '#') {
        None
    } else {
        Some(1)
    }
}

impl Spring {
    fn arrangements(&self) -> usize {
        matches(self.condition.to_owned(), self.groups.to_owned()).unwrap_or(0)
    }
}

pub fn part1(input: &Input) -> usize {
    input.springs.iter().map(Spring::arrangements).sum()
}

pub fn part2(input: &Input) -> usize {
    input
        .springs
        .iter()
        .map(|spring| {
            let mut condition = spring.condition.clone();
            condition.push('?');
            condition = condition.repeat(5);
            condition.pop();
            let groups = spring.groups.repeat(5);
            Spring { condition, groups }
        })
        .map(|spring| spring.arrangements())
        .sum()
}
