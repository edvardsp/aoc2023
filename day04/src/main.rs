use std::collections::HashSet;
use std::io::Read;

#[derive(Debug)]
struct Input {
    cards: Vec<ScratchCard>,
}

impl Input {
    fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open("input.txt")?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        let cards = input.lines().map(ScratchCard::from).collect();

        Ok(Self { cards })
    }
}

#[derive(Clone, Debug)]
struct ScratchCard {
    winning_numbers: HashSet<usize>,
    numbers: HashSet<usize>,
}

impl From<&str> for ScratchCard {
    fn from(s: &str) -> Self {
        let (_card, s) = s.split_once(':').unwrap();
        let (winning_numbers, numbers) = s.split_once('|').unwrap();
        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        Self {
            winning_numbers,
            numbers,
        }
    }
}

impl ScratchCard {
    fn winnings(&self) -> usize {
        self.winning_numbers.intersection(&self.numbers).count()
    }

    fn worth(&self) -> usize {
        match self.winnings() {
            0 => 0,
            n => 2usize.pow((n - 1) as u32),
        }
    }
}

fn part1(input: &Input) -> usize {
    input.cards.iter().map(ScratchCard::worth).sum()
}

fn part2(input: &Input) -> usize {
    let total_cards = input.cards.len();
    input
        .cards
        .iter()
        .enumerate()
        .map(|(i, card)| {
            let winnings = card.winnings();
            let start = (i + 1).min(total_cards);
            let end = (start + winnings).min(total_cards);
            (i, start..end)
        })
        .fold(vec![1; total_cards], |mut pocket, (i, range)| {
            let amount = pocket[i];
            pocket[range].iter_mut().for_each(|n| *n += amount);
            pocket
        })
        .iter()
        .sum()
}

fn main() {
    let input = Input::parse().unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
