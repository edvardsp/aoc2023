use std::io::Read;

#[derive(Debug)]
struct Input {
    lines: Vec<Vec<u8>>,
}

impl Input {
    fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open("input.txt")?;
        let mut input = String::new();
        file.read_to_string(&mut input)?;
        let lines = input.lines().map(|line| line.as_bytes().to_vec()).collect();

        Ok(Self { lines })
    }
}

fn part1(input: &Input) -> usize {
    unimplemented!("part1")
}

fn part2(input: &Input) -> usize {
    unimplemented!("part2")
}

fn main() {
    let input = Input::parse().unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
