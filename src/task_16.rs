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

    fn encode_fast(&mut self) {
        let mut result: Vec<i32> = Vec::new();
        while let Some(i) = self.data.pop() {
            if let Some(m) = result.pop() {
                result.push(m);
                result.push(((m + i) % 10).abs());
            } else {
                result.push((i % 10).abs());
            }
        }
        result.reverse();
        self.data = result;
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
    fn run_transmission(message: Message, offset: usize) -> PhaseIterator {
        let fast = offset > message.data.len() / 2;
        let m = if offset == 0 {
            message
        } else {
            Message {
                data: message.data.into_iter().skip(offset).collect(),
            }
        };
        PhaseIterator { message: m, fast }
    }
}

struct PhaseIterator {
    message: Message,
    fast: bool,
}

impl Iterator for PhaseIterator {
    type Item = Message;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.fast {
            self.message.encode();
        } else {
            self.message.encode_fast();
        }
        Some(self.message.clone())
    }
}

pub fn run() {
    let input = File::open("input/task_16").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let message = buffer.parse::<Message>().unwrap();
    let result = Phase::run_transmission(message, 0)
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

    let mut message = buffer.parse::<Message>().unwrap();
    message.repeated(10_000);
    let result = Phase::run_transmission(message, offset)
        .take(100)
        .enumerate()
        .map(|(_, v)| v)
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
