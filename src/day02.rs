#[derive(Debug)]
pub struct Input {
    games: Vec<Game>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let games = s.lines().map(Game::from).collect();
        Self { games }
    }
}

#[derive(Debug)]
struct Game {
    n: usize,
    cubes: Vec<Cubes>,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let s = s.strip_prefix("Game ").unwrap();
        let (n, cubes) = s.split_once(": ").unwrap();
        let n = n.parse().unwrap();
        let cubes = cubes.split("; ").map(Cubes::from).collect();
        Self { n, cubes }
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

impl From<&str> for Cubes {
    fn from(s: &str) -> Self {
        let mut cube = Cubes::default();
        for ncolor in s.split(", ") {
            let (n, color) = ncolor.split_once(' ').unwrap();
            let n = n.parse().unwrap();
            match color {
                "red" => cube.red = n,
                "green" => cube.green = n,
                "blue" => cube.blue = n,
                _ => panic!("invalid color: {}", color),
            }
        }
        cube
    }
}

impl Cubes {
    fn is_playable(&self, other: &Cubes) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

pub fn part1(input: &Input) -> usize {
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

pub fn part2(input: &Input) -> usize {
    input.games.iter().map(Game::power_set).sum()
}
