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

    fn length_to_target(&self, from: &str, obj: &str) -> Option<usize> {
        let target = self.0.get(from);
        if let Some(target) = target {
            if target.iter().any(|l| l == obj) {
                Some(1)
            } else {
                target
                    .iter()
                    .filter_map(|t| self.length_to_target(t, obj))
                    .map(|c| c + 1)
                    .min()
            }
        } else {
            None
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

pub fn run_e() {
    let input = File::open("input/task_6").unwrap();
    let mut input = BufReader::new(input);

    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let system = buffer.parse::<PlanetarySystem>().unwrap();

    let result = system
        .0
        .keys()
        .filter_map(|k| {
            let left = system.length_to_target(k, "YOU");
            let right = system.length_to_target(k, "SAN");
            match (left, right) {
                (Some(l), Some(r)) => Some(l + r),
                _ => None,
            }
        })
        .min()
        .map(|v| v - 2);

    println!("Result: {}", result.unwrap())
}
