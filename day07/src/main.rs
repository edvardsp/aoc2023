use std::collections::HashMap;
use std::io::Read;

#[derive(Debug)]
struct Input {
    hands: Vec<Hand>,
}

impl Input {
    fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let stdin = std::io::stdin();
        let mut input = String::new();
        stdin.lock().read_to_string(&mut input)?;
        let hands = input.lines().map(Hand::from).collect();

        Ok(Self { hands })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl From<u8> for Card {
    fn from(card: u8) -> Self {
        match card {
            b'A' => Self::Ace,
            b'K' => Self::King,
            b'Q' => Self::Queen,
            b'J' => Self::Jack,
            b'T' => Self::Ten,
            b'9' => Self::Nine,
            b'8' => Self::Eight,
            b'7' => Self::Seven,
            b'6' => Self::Six,
            b'5' => Self::Five,
            b'4' => Self::Four,
            b'3' => Self::Three,
            b'2' => Self::Two,
            _ => panic!("invalid card: {}", card as char),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Card; 5],
    bid: usize,
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        let (cards, bid) = s.split_once(' ').unwrap();
        let mut cards_iter = cards.as_bytes().iter().copied().map(Card::from);
        let cards = std::array::from_fn(|_| cards_iter.next().unwrap());
        let bid = bid.parse().unwrap();
        Hand::new(cards, bid)
    }
}

impl Hand {
    fn new(cards: [Card; 5], bid: usize) -> Self {
        let mut counts = HashMap::new();
        let joker_count = cards.iter().filter(|&&card| card == Card::Joker).count();
        for card in cards.iter().filter(|&&card| card != Card::Joker) {
            *counts.entry(card).or_insert(0) += 1;
        }
        let max_count = counts.values().copied().max().unwrap_or(0);
        let hand_type = match max_count + joker_count {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 if counts.len() == 2 => HandType::FullHouse,
            3 => HandType::ThreeOfAKind,
            2 if counts.len() == 3 => HandType::TwoPair,
            2 => HandType::OnePair,
            1 => HandType::HighCard,
            c => panic!("impossible hand type: {}", c),
        };

        Self {
            cards,
            bid,
            hand_type,
        }
    }

    fn with_joker(&self) -> Self {
        let mut cards = self.cards;
        cards
            .iter_mut()
            .filter(|card| **card == Card::Jack)
            .for_each(|card| *card = Card::Joker);
        Self::new(cards, self.bid)
    }
}

fn part1(input: &Input) -> usize {
    let mut cards = input.hands.clone();
    cards.sort_by(|l, r| r.cmp(l));
    cards
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1))
}

fn part2(input: &Input) -> usize {
    let mut cards: Vec<_> = input.hands.iter().map(Hand::with_joker).collect();
    cards.sort_by(|l, r| r.cmp(l));
    cards
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1))
}

fn main() {
    let input = Input::parse().unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
