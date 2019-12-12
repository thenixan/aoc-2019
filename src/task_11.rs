use crate::opcodes::Programm;
extern crate gif;
extern crate termion;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Eq, PartialEq, Hash, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn forward(&self, orientation: &Orientation) -> Self {
        match orientation {
            Orientation::North => Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Orientation::East => Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Orientation::South => Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Orientation::West => Coordinate {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Clone)]
enum Color {
    Black,
    White,
}

#[derive(Clone)]
struct CellState {
    painted: usize,
    color: Color,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }
}

#[derive(Clone)]
enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    fn left(&self) -> Self {
        match self {
            Orientation::North => Orientation::West,
            Orientation::West => Orientation::South,
            Orientation::South => Orientation::East,
            Orientation::East => Orientation::North,
        }
    }

    fn right(&self) -> Self {
        match self {
            Orientation::North => Orientation::East,
            Orientation::East => Orientation::South,
            Orientation::South => Orientation::West,
            Orientation::West => Orientation::North,
        }
    }
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Orientation::East => '⇨',
                Orientation::North => '⇧',
                Orientation::South => '⇩',
                Orientation::West => '⇦',
            }
        )
    }
}

#[derive(Clone)]
struct Plane {
    programm: Programm,
    paints: HashMap<Coordinate, CellState>,
    orientation: Orientation,
    position: Coordinate,
}

impl Plane {
    fn new(programm: Programm) -> Self {
        Plane {
            programm,
            paints: HashMap::new(),
            orientation: Orientation::North,
            position: Coordinate::new(0, 0),
        }
    }

    fn with_color(programm: Programm, color: Color) -> Self {
        let mut paints = HashMap::new();
        paints.insert(Coordinate::new(0, 0), CellState { painted: 0, color });
        Plane {
            programm,
            paints,
            orientation: Orientation::North,
            position: Coordinate::new(0, 0),
        }
    }
}

impl Plane {
    fn current_color(&mut self) -> i64 {
        let state = self
            .paints
            .entry(self.position.clone())
            .or_insert(CellState {
                painted: 0,
                color: Color::Black,
            });
        match state.color {
            Color::Black => 0,
            Color::White => 1,
        }
    }

    fn paint(&mut self, color: Color) -> usize {
        let c = self
            .paints
            .entry(self.position.clone())
            .or_insert(CellState {
                painted: 0,
                color: Color::Black,
            });
        c.color = color;
        c.painted += 1;
        c.painted
    }

    fn parse_results(&mut self, r: Vec<i64>) -> bool {
        let color = r[0];
        let orientation = r[1];
        let color = if color == 0 {
            Color::Black
        } else {
            Color::White
        };
        self.orientation = if orientation == 0 {
            self.orientation.left()
        } else {
            self.orientation.right()
        };
        let result = self.paint(color);
        self.position = self.position.forward(&self.orientation);
        result == 1
    }
}

impl Iterator for Plane {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        if self.programm.is_finished() {
            None
        } else {
            let current_color = self.current_color();
            let result = self.programm.run(&mut vec![current_color]);
            let result = self.parse_results(result);
            Some(result)
        }
    }
}

impl Display for Plane {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let left = self.paints.keys().map(|k| k.x).min().unwrap();
        let right = self.paints.keys().map(|k| k.x).max().unwrap();
        let top = self.paints.keys().map(|k| k.y).max().unwrap();
        let bottom = self.paints.keys().map(|k| k.y).min().unwrap();

        let width = (right - left).abs();
        let height = (top - bottom).abs();
        let mut s = String::new();
        for j in 0..=height {
            let y = height - j + bottom;
            let mut l = String::new();
            for i in 0..=width {
                let x = i + left;
                if let Some(m) = self.paints.get(&Coordinate::new(x, y)) {
                    match m.color {
                        Color::Black => l.push(' '),
                        Color::White => l.push('#'),
                    }
                } else {
                    l.push(' ');
                }
            }
            s += l.as_str();
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

pub fn run() {
    let input = File::open("input/task_11").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let programm = buffer.parse::<Programm>().unwrap();

    let plane = Plane::new(programm);
    let result = plane.filter(|o| *o).count();
    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_11").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let programm = buffer.parse::<Programm>().unwrap();

    let plane = &mut Plane::with_color(programm, Color::White);
    let _result = plane.all(|_| true);

    println!("{}", plane);
}
