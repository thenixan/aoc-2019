use iterslide::SlideIterator;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;

struct BruteforceRange {
    from: usize,
    to: usize,
}

impl<'a> IntoIterator for &'a BruteforceRange {
    type Item = Password;
    type IntoIter = PasswordIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PasswordIterator::new(&self)
    }
}

struct PasswordIterator<'a> {
    range: &'a BruteforceRange,
    current: usize,
}

impl<'a> PasswordIterator<'a> {
    fn new(range: &'a BruteforceRange) -> Self {
        PasswordIterator {
            range,
            current: range.from,
        }
    }
}

impl<'a> Iterator for PasswordIterator<'a> {
    type Item = Password;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.range.to + 1 {
            None
        } else {
            let result = Some(Password(self.current.to_string()));
            self.current += 1;
            result
        }
    }
}

struct Password(String);

impl Password {
    fn is_six_digit(&self) -> bool {
        self.0.len() == 6
    }
    fn has_same_two_digits(&self) -> bool {
        self.0.chars().slide(2).any(|o| o[0] == o[1])
    }
    fn is_increasing(&self) -> bool {
        self.0.chars().slide(2).all(|o| o[0] <= o[1])
    }
    fn is_not_part_of_larger(&self) -> bool {
        let pairs = self
            .0
            .chars()
            .slide(2)
            .filter_map(|o| if o[0] == o[1] { Some(o[0]) } else { None })
            .collect::<Vec<char>>();
        pairs
            .iter()
            .map(|p| self.0.chars().filter(|o| o == p).count())
            .any(|c| c == 2)
    }
}

impl BruteforceRange {
    fn new(from: usize, to: usize) -> BruteforceRange {
        BruteforceRange { from, to }
    }
}

impl FromStr for BruteforceRange {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split("-")
            .filter_map(|l| l.parse::<usize>().ok())
            .collect::<Vec<usize>>();
        Ok(BruteforceRange::new(parts[0], parts[1]))
    }
}

pub fn run() {
    let input = File::open("input/task_4").unwrap();
    let mut input = BufReader::new(input);

    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let range = buffer.parse::<BruteforceRange>().unwrap();

    let result = range
        .into_iter()
        .filter(|password| {
            password.is_six_digit() && password.is_increasing() && password.has_same_two_digits()
        })
        .count();

    println!("Result: {}", result)
}

pub fn run_e() {
    let input = File::open("input/task_4").unwrap();
    let mut input = BufReader::new(input);

    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let range = buffer.parse::<BruteforceRange>().unwrap();

    let result = range
        .into_iter()
        .filter(|password| {
            password.is_six_digit() && password.is_increasing() && password.is_not_part_of_larger()
        })
        .count();

    println!("Result: {}", result)
}
