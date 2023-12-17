#![allow(clippy::single_range_in_vec_init)]

use std::collections::HashMap;
use std::ops::Range;

#[derive(Debug)]
pub struct Input {
    workflows: HashMap<String, Workflow>,
    ratings: Vec<Rating>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let s = s.replace("\r\n", "\n");
        let mut sections = s.split("\n\n");
        let workflows = sections
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let delim = line.find('{').unwrap();
                let id = line[0..delim].to_string();
                let workflow = Workflow::from(&line[delim + 1..line.len() - 1]);
                (id, workflow)
            })
            .collect();
        let ratings = sections.next().unwrap().lines().map(Rating::from).collect();

        Self { workflows, ratings }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<(Rule, Result)>,
    or_else: Result,
}

impl Workflow {
    fn run(&self, rating: &Rating) -> Result {
        for (rule, result) in &self.rules {
            match rule {
                Rule::Greater(part, n) => {
                    if rating.part(part) > *n {
                        return result.clone();
                    }
                }
                Rule::Less(part, n) => {
                    if rating.part(part) < *n {
                        return result.clone();
                    }
                }
            }
        }
        self.or_else.clone()
    }
}

impl From<&str> for Workflow {
    fn from(s: &str) -> Self {
        let rules_inclusive: Vec<_> = s.split(',').collect();
        let rules = rules_inclusive[..rules_inclusive.len() - 1]
            .iter()
            .map(|&rule| {
                let (rule, result) = rule.split_once(':').unwrap();
                (rule.into(), result.into())
            })
            .collect();
        let or_else = Result::from(*rules_inclusive.last().unwrap());
        Self { rules, or_else }
    }
}

#[derive(Debug)]
enum Part {
    X,
    M,
    A,
    S,
}

impl From<&str> for Part {
    fn from(s: &str) -> Self {
        match s {
            "x" => Part::X,
            "m" => Part::M,
            "a" => Part::A,
            "s" => Part::S,
            _ => unreachable!("invalid part: {}", s),
        }
    }
}

#[derive(Debug)]
enum Rule {
    Less(Part, usize),
    Greater(Part, usize),
}

impl From<&str> for Rule {
    fn from(s: &str) -> Self {
        let part = Part::from(&s[0..1]);
        let n = s[2..].parse().unwrap();
        match &s[1..2] {
            "<" => Self::Less(part, n),
            ">" => Self::Greater(part, n),
            op => unreachable!("invalid operator: {}", op),
        }
    }
}

#[derive(Clone, Debug)]
enum Result {
    Accept,
    Reject,
    Next(String),
}

impl From<&str> for Result {
    fn from(s: &str) -> Self {
        match s {
            "A" => Result::Accept,
            "R" => Result::Reject,
            _ => Result::Next(s.to_string()),
        }
    }
}

#[derive(Debug)]
struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    fn part(&self, part: &Part) -> usize {
        match part {
            Part::X => self.x,
            Part::M => self.m,
            Part::A => self.a,
            Part::S => self.s,
        }
    }

    fn value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    fn is_accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut curr = "in".to_string();
        loop {
            let workflow = workflows.get(&curr).unwrap();
            match workflow.run(self) {
                Result::Accept => return true,
                Result::Reject => return false,
                Result::Next(id) => curr = id,
            }
        }
    }
}

impl From<&str> for Rating {
    fn from(s: &str) -> Self {
        let mut token = s[1..s.len() - 1].split(',').map(|p| &p[2..]);
        let x = token.next().unwrap().parse().unwrap();
        let m = token.next().unwrap().parse().unwrap();
        let a = token.next().unwrap().parse().unwrap();
        let s = token.next().unwrap().parse().unwrap();
        Self { x, m, a, s }
    }
}

#[derive(Clone, Debug)]
struct SuperRating {
    x: Vec<Range<usize>>,
    m: Vec<Range<usize>>,
    a: Vec<Range<usize>>,
    s: Vec<Range<usize>>,
}

impl SuperRating {
    fn new() -> Self {
        Self {
            x: vec![1..4001],
            m: vec![1..4001],
            a: vec![1..4001],
            s: vec![1..4001],
        }
    }

    fn combinations(&self) -> usize {
        self.x.iter().map(|r| r.len()).sum::<usize>()
            * self.m.iter().map(|r| r.len()).sum::<usize>()
            * self.a.iter().map(|r| r.len()).sum::<usize>()
            * self.s.iter().map(|r| r.len()).sum::<usize>()
    }

    fn part(&self, part: &Part) -> &Vec<Range<usize>> {
        match part {
            Part::X => &self.x,
            Part::M => &self.m,
            Part::A => &self.a,
            Part::S => &self.s,
        }
    }

    fn part_mut(&mut self, part: &Part) -> &mut Vec<Range<usize>> {
        match part {
            Part::X => &mut self.x,
            Part::M => &mut self.m,
            Part::A => &mut self.a,
            Part::S => &mut self.s,
        }
    }

    fn split(&self, rule: &Rule) -> (Self, Self) {
        let mut is_match = self.clone();
        let mut or_else = self.clone();

        match rule {
            Rule::Greater(part, n) => {
                let ranges = self.part(part);
                let succeed = is_match.part_mut(part);
                let fail = or_else.part_mut(part);

                succeed.clear();
                fail.clear();
                for range in ranges {
                    if range.start > *n {
                        succeed.push(range.clone());
                    } else if range.end <= *n + 1 {
                        fail.push(range.clone());
                    } else {
                        fail.push(range.start..*n + 1);
                        succeed.push(*n + 1..range.end);
                    }
                }
            }
            Rule::Less(part, n) => {
                let ranges = self.part(part);
                let succeed = is_match.part_mut(part);
                let fail = or_else.part_mut(part);

                succeed.clear();
                fail.clear();
                for range in ranges {
                    if range.end <= *n {
                        succeed.push(range.clone());
                    } else if range.start >= *n {
                        fail.push(range.clone());
                    } else {
                        succeed.push(range.start..*n);
                        fail.push(*n..range.end);
                    }
                }
            }
        }

        (is_match, or_else)
    }
}

fn combinations(
    workflows: &HashMap<String, Workflow>,
    curr: String,
    mut rating: SuperRating,
) -> usize {
    let workflow = workflows.get(&curr).unwrap();

    let mut summa = 0;
    for (rule, result) in &workflow.rules {
        let (is_match, or_else) = rating.split(rule);
        match result {
            Result::Accept => {
                summa += is_match.combinations();
            }
            Result::Reject => {}
            Result::Next(id) => {
                summa += combinations(workflows, id.clone(), is_match);
            }
        }
        rating = or_else;
    }

    match &workflow.or_else {
        Result::Accept => {
            summa += rating.combinations();
        }
        Result::Reject => {}
        Result::Next(id) => {
            summa += combinations(workflows, id.clone(), rating.clone());
        }
    }

    summa
}

pub fn part1(input: &Input) -> usize {
    input
        .ratings
        .iter()
        .filter(|rating| rating.is_accepted(&input.workflows))
        .map(|rating| rating.value())
        .sum()
}

pub fn part2(input: &Input) -> usize {
    combinations(&input.workflows, "in".to_string(), SuperRating::new())
}
