use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::Iterator;
use std::str::FromStr;

#[derive(Clone)]
struct Message {
    data: Vec<i32>,
}

impl Message {
    fn repeated(&mut self, times: usize) {
        self.data = self.data.repeat(times);
    }
}

impl FromStr for Message {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Message {
            data: s
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|i| i as i32)
                .collect(),
        })
    }
}

struct Phase {
    data: Vec<i32>,
}

impl Phase {
    fn new(data: Vec<i32>) -> Self {
        Phase { data }
    }

    fn key_for(&self, position: usize, size: usize) -> Vec<i32> {
        self.data
            .iter()
            .flat_map(|m| std::iter::repeat(*m).take(position))
            .cycle()
            .skip(1)
            .take(size)
            .collect()
    }

    fn run_transmission(&self, message: Message) -> PhaseIterator {
        PhaseIterator {
            phase: self,
            message,
        }
    }

    fn encode(&self, message: &Message) -> Message {
        let size = message.data.len();
        let result = std::iter::repeat(message.data.clone())
            .take(size)
            .enumerate()
            .map(|(position, data)| {
                data.into_iter()
                    .zip(self.key_for(position + 1, size))
                    .map(|(l, r)| l * r)
                    .sum()
            })
            .map(|item: i32| item % 10)
            .map(|item: i32| item.abs())
            .collect();
        Message { data: result }
    }
}

struct PhaseIterator<'a> {
    phase: &'a Phase,
    message: Message,
}

impl<'a> Iterator for PhaseIterator<'a> {
    type Item = Message;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.phase.encode(&self.message);
        self.message = result;
        Some(self.message.clone())
    }
}

pub fn run() {
    let input = File::open("input/task_16").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let message = buffer.parse::<Message>().unwrap();
    let phase = Phase::new(vec![0, 1, 0, -1]);
    let result = phase
        .run_transmission(message)
        .take(100)
        .last()
        .unwrap()
        .data
        .iter()
        .take(8)
        .fold("".to_string(), |mut acc, i| {
            acc += format!("{}", i).as_str();
            acc
        });
    println!("{}", result);
}

pub fn run_e() {
    let input = File::open("input/task_16").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let offset = buffer.clone()[0..7].parse::<usize>().unwrap();

    println!("Offset: {}", offset);
    let mut message = buffer.parse::<Message>().unwrap();
    message.repeated(10_000);
    let phase = Phase::new(vec![0, 1, 0, -1]);
    let result = phase
        .run_transmission(message)
        .take(100)
        .enumerate()
        .inspect(|(i, _)| println!("Running: {}", i))
        .map(|(_, v)| v)
        .last()
        .unwrap()
        .data
        .iter()
        .skip(offset)
        .take(8)
        .fold("".to_string(), |mut acc, i| {
            acc += format!("{}", i).as_str();
            acc
        });
    println!("{}", result);
}
