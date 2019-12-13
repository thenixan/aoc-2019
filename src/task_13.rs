use crate::opcodes::Programm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;

enum Object {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

#[derive(Hash, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }
}

struct ArcadeCabinet {
    layout: HashMap<Coordinate, Object>,
}

impl ArcadeCabinet {
    fn fill_layout(&mut self, p: &mut Programm) {
        let mut iter = p.run(&mut Vec::new()).into_iter();
        while let Some(x) = iter.next() {
            if let Some(y) = iter.next() {
                if let Some(o) = iter.next().map(|o| match o {
                    1 => Object::Wall,
                    2 => Object::Block,
                    3 => Object::HorizontalPaddle,
                    4 => Object::Ball,
                    _ => Object::Empty,
                }) {
                    self.layout
                        .insert(Coordinate::new(x as usize, y as usize), o);
                }
            }
        }
    }

    fn count_blocks(&self) -> usize {
        self.layout
            .values()
            .filter(|o| match o {
                Object::Block => true,
                _ => false,
            })
            .count()
    }
}

impl FromStr for ArcadeCabinet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: ArcadeCabinet = ArcadeCabinet {
            layout: HashMap::new(),
        };
        let mut programm: Programm = s.parse().unwrap();
        result.fill_layout(&mut programm);
        Ok(result)
    }
}

pub fn run() {
    let input = File::open("input/task_13").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let cabinet: ArcadeCabinet = buffer.parse().unwrap();

    let result = cabinet.count_blocks();
    println!("Result: {}", result);
}

pub fn run_e() {}
