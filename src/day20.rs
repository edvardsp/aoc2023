use dyn_clone::DynClone;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Input {
    machine: Machine,
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();
        let mut links = HashMap::new();
        let mut conjunctions = Vec::new();
        for line in s.lines() {
            let (head, tail) = line.split_once(" -> ").unwrap();
            let link: Vec<_> = tail.split(", ").map(|m| m.to_string()).collect();
            if head == "broadcaster" {
                modules.insert(head.to_string(), Box::new(Broadcaster::new()));
                links.insert(head.to_string(), link);
            } else if let Some(name) = head.strip_prefix('%') {
                modules.insert(name.to_string(), Box::new(FlipFlop::new()));
                links.insert(name.to_string(), link);
            } else if let Some(name) = head.strip_prefix('&') {
                conjunctions.push((name.to_string(), Conjunction::new()));
                links.insert(name.to_string(), link);
            }
        }

        for (name, mut module) in conjunctions {
            let mut inputs = Vec::new();
            for (from, to) in &links {
                if to.contains(&name) {
                    inputs.push(from.as_str());
                }
            }
            module.register(&inputs);
            modules.insert(name, Box::new(module));
        }

        links.insert("button".to_string(), vec!["broadcaster".to_string()]);

        let machine = Machine {
            modules,
            links,
            queue: VecDeque::new(),
        };
        Self { machine }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

trait Module: DynClone + Debug {
    fn process(&mut self, pulse: Pulse, from: &str) -> Option<Pulse>;
    fn is_high(&self) -> bool;
}

dyn_clone::clone_trait_object!(Module);

#[derive(Clone, Debug)]
struct Broadcaster {
    curr: Option<Pulse>,
}

impl Broadcaster {
    fn new() -> Self {
        Self { curr: None }
    }
}

impl Module for Broadcaster {
    fn process(&mut self, pulse: Pulse, _from: &str) -> Option<Pulse> {
        self.curr = Some(pulse);
        self.curr
    }

    fn is_high(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
struct FlipFlop {
    toggle: bool,
}

impl FlipFlop {
    fn new() -> Self {
        Self { toggle: false }
    }
}

impl Module for FlipFlop {
    fn process(&mut self, pulse: Pulse, _from: &str) -> Option<Pulse> {
        match (pulse, self.toggle) {
            (Pulse::High, _) => None,
            (Pulse::Low, false) => {
                self.toggle = true;
                Some(Pulse::High)
            }
            (Pulse::Low, true) => {
                self.toggle = false;
                Some(Pulse::Low)
            }
        }
    }

    fn is_high(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
struct Conjunction {
    memory: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            memory: Default::default(),
        }
    }

    fn register(&mut self, inputs: &[&str]) {
        self.memory = inputs
            .iter()
            .map(|input| (input.to_string(), Pulse::Low))
            .collect();
    }
}

impl Module for Conjunction {
    fn process(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        *self.memory.get_mut(from).unwrap() = pulse;

        if self.memory.values().all(|pulse| *pulse == Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }

    fn is_high(&self) -> bool {
        self.memory.values().all(|pulse| *pulse == Pulse::High)
    }
}

#[derive(Clone, Debug)]
struct Machine {
    modules: HashMap<String, Box<dyn Module>>,
    links: HashMap<String, Vec<String>>,
    queue: VecDeque<(String, Pulse, String)>,
}

impl Machine {
    fn run(&mut self, mut check: impl FnMut(&String, Pulse, &String)) {
        assert!(self.queue.is_empty());

        self.queue
            .push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));
        while let Some((from, pulse, to)) = self.queue.pop_front() {
            check(&from, pulse, &to);

            let module = match self.modules.get_mut(&to) {
                Some(module) => module,
                None => continue,
            };
            if let Some(pulse) = module.process(pulse, &from) {
                self.queue.extend(
                    self.links[&to]
                        .iter()
                        .map(|next| (to.clone(), pulse, next.clone())),
                );
            }
        }
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        b %= a;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn part1(input: &Input) -> usize {
    let mut high = 0;
    let mut low = 0;

    let mut machine = input.machine.clone();
    for _ in 1..=1000 {
        machine.run(|_from, pulse, _to| {
            // println!("{} -{:?}-> {}", _from, _pulse, _to);
            match pulse {
                Pulse::High => high += 1,
                Pulse::Low => low += 1,
            }
        });
    }

    high * low
}

pub fn part2(input: &Input) -> usize {
    let mut machine = input.machine.clone();

    let goal = "rx".to_string();
    let parent = machine
        .links
        .iter()
        .find_map(|(name, link)| {
            if link.contains(&goal) {
                Some(name.clone())
            } else {
                None
            }
        })
        .unwrap();

    let ancestors: Vec<_> = machine
        .links
        .iter()
        .filter_map(|(name, link)| {
            if link.contains(&parent) {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect();

    let mut history: HashMap<String, Option<usize>> =
        ancestors.iter().map(|a| (a.clone(), None)).collect();

    for i in 1.. {
        machine.run(|from, pulse, _to: &String| {
            if ancestors.contains(from) && pulse == Pulse::High {
                history.get_mut(from).unwrap().replace(i);
            }
        });

        if history.values().all(|list| list.is_some()) {
            return history.values().flatten().copied().fold(1, lcm);
        }
    }
    unreachable!()
}
