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
    fn key_for(item: i32, iteration: usize, position: usize) -> i32 {
        let p = ((position + 1) / (iteration + 1)) % 4;
        match p {
            1 => item,
            3 => -item,
            _ => 0,
        }
    }

    fn encode(&mut self) {
        let size = self.data.len();
        self.data = self
            .data
            .iter()
            .cycle()
            .take(size * size)
            .enumerate()
            .map(|(position, item)| (position / size, position % size, item))
            .map(|(iteration, position, item)| {
                (iteration, Message::key_for(*item, iteration, position))
            })
            .fold(vec![], |mut acc, (iteration, item)| {
                let v = if iteration == acc.len() {
                    0
                } else {
                    acc.pop().unwrap()
                };
                acc.push(v + item);
                acc
            })
            .into_iter()
            .map(|i| i % 10)
            .map(|i| i.abs())
            .collect();
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

struct Phase {}

impl Phase {
    fn run_transmission(message: Message) -> PhaseIterator {
        PhaseIterator { message }
    }
}

struct PhaseIterator {
    message: Message,
}

impl Iterator for PhaseIterator {
    type Item = Message;
    fn next(&mut self) -> Option<Self::Item> {
        self.message.encode();
        Some(self.message.clone())
    }
}

pub fn run() {
    let input = File::open("input/task_16").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let message = buffer.parse::<Message>().unwrap();
    let result = Phase::run_transmission(message)
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
    let result = Phase::run_transmission(message)
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
