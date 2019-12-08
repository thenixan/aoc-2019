use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;

struct PlanetarySystem(HashMap<String, Vec<String>>);

impl PlanetarySystem {
    fn count_length(&self, from: &str, counter: usize) -> usize {
        let target = self.0.get(from);
        if let Some(target) = target {
            target
                .iter()
                .map(|t| self.count_length(t, counter + 1) + counter + 1)
                .sum::<usize>()
        } else {
            0
        }
    }
}

impl FromStr for PlanetarySystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = HashMap::new();
        s.lines()
            .map(|l| l.split(")").collect::<Vec<&str>>())
            .for_each(|l| {
                result
                    .entry(l[0].to_string())
                    .or_insert(vec![])
                    .push(l[1].to_string());
            });
        Ok(PlanetarySystem(result))
    }
}

pub fn run() {
    let input = File::open("input/task_6").unwrap();
    let mut input = BufReader::new(input);

    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let system = buffer.parse::<PlanetarySystem>().unwrap();

    let result = system.count_length("COM", 0);

    println!("Result: {}", result)
}

pub fn run_e() {}
