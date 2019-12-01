use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Module {
    weight: usize,
}

impl Module {
    fn new(weight: usize) -> Self {
        Module { weight }
    }

    fn fuel_required(&self) -> usize {
        self.weight / 3 - 2
    }

    fn fuel_for_fuel(fuel: usize) -> usize {
        let result = fuel / 3;
        if result > 2 {
            result - 2 + Module::fuel_for_fuel(result - 2)
        } else {
            0
        }
    }
}

impl FromStr for Module {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<usize>().map(|w| Module::new(w)).map_err(|_| ())
    }
}

pub fn run() {
    let input = File::open("input/task_1").unwrap();
    let input = BufReader::new(input);

    let result = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Module>().ok())
        .map(|m| m.fuel_required())
        .sum::<usize>();

    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_1").unwrap();
    let input = BufReader::new(input);

    let result = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Module>().ok())
        .map(|m| m.fuel_required())
        .map(|m| Module::fuel_for_fuel(m) + m)
        .sum::<usize>();

    println!("Result: {}", result);
}
