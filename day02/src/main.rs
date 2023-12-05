use std::convert::TryFrom;
use std::io::Read;

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("io error")]
    Io(#[from] std::io::Error),
    #[error("parse int error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("parse input error")]
    ParseInput(String),
}

#[derive(Debug)]
struct Input {
    games: Vec<Game>,
}

impl Input {
    fn parse() -> Result<Self, Error> {
        let stdin = std::io::stdin();
        let mut input = String::new();
        stdin.lock().read_to_string(&mut input)?;
        let games = input
            .lines()
            .map(Game::try_from)
            .collect::<Result<_, _>>()?;
        Ok(Self { games })
    }
}

#[derive(Debug)]
struct Game {
    n: usize,
    cubes: Vec<Cubes>,
}

impl TryFrom<&str> for Game {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s
            .strip_prefix("Game ")
            .ok_or_else(|| Error::ParseInput(s.to_string()))?;
        let (n, cubes) = s
            .split_once(": ")
            .ok_or_else(|| Error::ParseInput(s.to_string()))?;
        let n = n.parse()?;
        let cubes = cubes
            .split("; ")
            .map(Cubes::try_from)
            .collect::<Result<_, _>>()?;
        Ok(Self { n, cubes })
    }
}

impl Game {
    fn is_playable(&self, cubes: &Cubes) -> bool {
        self.cubes.iter().all(|c| c.is_playable(cubes))
    }

    fn power_set(&self) -> usize {
        let mut minimal_cubes = Cubes::default();
        for c in &self.cubes {
            minimal_cubes.red = minimal_cubes.red.max(c.red);
            minimal_cubes.green = minimal_cubes.green.max(c.green);
            minimal_cubes.blue = minimal_cubes.blue.max(c.blue);
        }
        minimal_cubes.red * minimal_cubes.green * minimal_cubes.blue
    }
}

#[derive(Debug, Default)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl TryFrom<&str> for Cubes {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut cube = Cubes::default();
        for ncolor in s.split(", ") {
            let (n, color) = ncolor
                .split_once(" ")
                .ok_or_else(|| Error::ParseInput(s.to_string()))?;
            let n = n.parse()?;
            match color {
                "red" => cube.red = n,
                "green" => cube.green = n,
                "blue" => cube.blue = n,
                _ => return Err(Error::ParseInput(s.to_string())),
            }
        }
        Ok(cube)
    }
}

impl Cubes {
    fn is_playable(&self, other: &Cubes) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

fn part1(input: &Input) -> usize {
    const CUBES: Cubes = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .games
        .iter()
        .filter_map(|game| {
            if game.is_playable(&CUBES) {
                Some(game.n)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &Input) -> usize {
    input.games.iter().map(Game::power_set).sum()
}

fn main() {
    let input = Input::parse().unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
