use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    steps: Vec<Vec<u8>>,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        Self {
            steps: s.split(',').map(|kv| kv.as_bytes().to_vec()).collect(),
        }
    }
}

fn digest(bytes: &[u8]) -> usize {
    let mut hash = 0;
    for &b in bytes {
        hash += b as usize;
        hash *= 17;
        hash %= 256;
    }
    hash
}

pub fn part1(input: &Input) -> usize {
    input.steps.iter().map(|step| digest(step.as_slice())).sum()
}

pub fn part2(input: &Input) -> usize {
    let mut hashmap = HashMap::new();
    for step in &input.steps {
        let (id, cmd) = if step[step.len() - 1].is_ascii_digit() {
            step.as_slice().split_at(step.len() - 2)
        } else {
            step.as_slice().split_at(step.len() - 1)
        };
        let hash = digest(id);
        let id: String = std::str::from_utf8(id).unwrap().to_string();
        match cmd[0] {
            b'=' => {
                let value = (cmd[1] - b'0') as usize;
                let lense: &mut Vec<(String, usize)> = hashmap.entry(hash).or_insert(Vec::new());
                if let Some(item) = lense.iter_mut().find(|item| item.0 == id) {
                    item.1 = value;
                } else {
                    lense.push((id, value));
                }
            }
            b'-' => {
                let lense: &mut Vec<(String, usize)> = hashmap.entry(hash).or_insert(Vec::new());
                for i in 0..lense.len() {
                    if lense[i].0 == id {
                        lense.remove(i);
                        break;
                    }
                }
            }
            c => unreachable!("invalid command: {}", c),
        }
    }

    hashmap
        .iter()
        .map(|(boxn, lense)| {
            lense
                .iter()
                .enumerate()
                .map(|(slot, (_id, value))| (boxn + 1) * (slot + 1) * value)
                .sum::<usize>()
        })
        .sum()
}
