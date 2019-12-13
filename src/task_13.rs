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

impl Object {
    fn is_ball(&self) -> bool {
        match self {
            Object::Ball => true,
            _ => false,
        }
    }

    fn is_paddle(&self) -> bool {
        match self {
            Object::HorizontalPaddle => true,
            _ => false,
        }
    }
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
    fn empty() -> Self {
        Coordinate { x: 0, y: 0 }
    }
}

struct ArcadeCabinet {
    programm: Programm,
    layout: HashMap<Coordinate, Object>,
    ball_position: Coordinate,
    paddle_position: Coordinate,
    played: bool,
    score: i64,
    input: Vec<i64>,
}

impl ArcadeCabinet {
    fn play(&mut self) {
        if !self.played {
            self.programm.alter(0, 2);
            self.played = true;
        }
        while !self.programm.is_finished() {
            self.fill_layout();
            self.input
                .push(if self.paddle_position.x < self.ball_position.x {
                    1
                } else if self.paddle_position.x > self.ball_position.x {
                    -1
                } else {
                    0
                })
        }
    }

    fn fill_layout(&mut self) {
        let mut iter = self.programm.run(&mut self.input).into_iter();
        while let Some(x) = iter.next() {
            if let Some(y) = iter.next() {
                if x == -1 && y == 0 {
                    self.score = iter.next().unwrap();
                } else {
                    if let Some(o) = iter.next().map(|o| match o {
                        1 => Object::Wall,
                        2 => Object::Block,
                        3 => Object::HorizontalPaddle,
                        4 => Object::Ball,
                        _ => Object::Empty,
                    }) {
                        if o.is_ball() {
                            self.ball_position = Coordinate::new(x as usize, y as usize);
                        } else if o.is_paddle() {
                            self.paddle_position = Coordinate::new(x as usize, y as usize);
                        }
                        self.layout
                            .insert(Coordinate::new(x as usize, y as usize), o);
                    }
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
        Ok(ArcadeCabinet {
            programm: s.parse().unwrap(),
            layout: HashMap::new(),
            paddle_position: Coordinate::empty(),
            ball_position: Coordinate::empty(),
            played: false,
            score: 0,
            input: vec![],
        })
    }
}

pub fn run() {
    let input = File::open("input/task_13").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let mut cabinet: ArcadeCabinet = buffer.parse().unwrap();

    cabinet.fill_layout();
    let result = cabinet.count_blocks();
    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_13").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let mut cabinet: ArcadeCabinet = buffer.parse().unwrap();

    cabinet.play();
    println!("Result: {}", cabinet.score);
}
