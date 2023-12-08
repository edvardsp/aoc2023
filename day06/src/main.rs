use std::io::Read;

#[derive(Debug)]
struct Input {
    races1: Vec<Race>,
    race2: Race,
}

impl Input {
    fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open("input.txt")?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        let mut lines = input.lines();
        let time_str = lines.next().unwrap().strip_prefix("Time:").unwrap();
        let distance_str = lines.next().unwrap().strip_prefix("Distance:").unwrap();

        let times = time_str
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let distances = distance_str
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let races1 = times
            .iter()
            .zip(distances.iter())
            .map(|(&time, &distance)| Race { time, distance })
            .collect();

        let time = time_str.replace(' ', "").parse().unwrap();
        let distance = distance_str.replace(' ', "").parse().unwrap();
        let race2 = Race { time, distance };

        Ok(Self { races1, race2 })
    }
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn simulate_win(&self, hold_time: usize) -> bool {
        let time_left = self.time - hold_time;
        let distance = hold_time * time_left;
        distance > self.distance
    }

    fn win_possibilities(&self) -> usize {
        (1..self.time).filter(|&t| self.simulate_win(t)).count()
    }
}

fn part1(input: &Input) -> usize {
    input.races1.iter().map(|r| r.win_possibilities()).product()
}

fn part2(input: &Input) -> usize {
    input.race2.win_possibilities()
}

fn main() {
    let input = Input::parse().unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
