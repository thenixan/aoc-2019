use crate::opcodes::Programm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::Iterator;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Coordinate(i32, i32);

impl Coordinate {
    fn start() -> Self {
        Coordinate(0, 0)
    }
    fn up(&self) -> Self {
        Coordinate(self.0, self.1 + 1)
    }
    fn right(&self) -> Self {
        Coordinate(self.0 + 1, self.1)
    }
    fn down(&self) -> Self {
        Coordinate(self.0, self.1 - 1)
    }
    fn left(&self) -> Self {
        Coordinate(self.0 - 1, self.1)
    }
}
enum Content {
    Nothing,
    Wall,
    Target,
}
enum ContentInTime {
    Wall,
    Target,
    Nothing(Option<usize>),
}

struct OxygenInMaze {
    layout: HashMap<Coordinate, ContentInTime>,
}

impl Iterator for OxygenInMaze {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let max = self
            .layout
            .iter()
            .map(|(_, v)| v)
            .filter_map(|i| match i {
                ContentInTime::Wall => None,
                ContentInTime::Target => Some(0),
                ContentInTime::Nothing(Some(v)) => Some(*v),
                ContentInTime::Nothing(None) => None,
            })
            .max()
            .unwrap();
        let current_edges = self
            .layout
            .iter()
            .filter_map(|(k, v)| match v {
                ContentInTime::Target => {
                    if max == 0 {
                        Some(k)
                    } else {
                        None
                    }
                }
                ContentInTime::Nothing(Some(v)) => {
                    if *v == max {
                        Some(k)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .map(|v| v.clone())
            .collect::<Vec<_>>();
        let mut changed = vec![];
        for c in current_edges {
            for i in 0..4 {
                let coord = match i {
                    0 => c.up(),
                    1 => c.down(),
                    2 => c.left(),
                    _ => c.right(),
                };
                if let Some(item) = self.layout.get(&coord) {
                    match item {
                        ContentInTime::Nothing(None) => {
                            changed.push(coord.clone());
                            *self
                                .layout
                                .entry(coord)
                                .or_insert(ContentInTime::Nothing(None)) =
                                ContentInTime::Nothing(Some(max + 1));
                        }
                        _ => {}
                    }
                }
            }
        }
        println!("{}: {:?}", max, changed);
        if !changed.is_empty() {
            Some(max + 1)
        } else {
            None
        }
    }
}

struct Maze {
    layout: HashMap<Coordinate, Content>,
}

impl Maze {
    fn new() -> Self {
        Maze {
            layout: HashMap::new(),
        }
    }

    fn fill_with_oxygen(&self) -> OxygenInMaze {
        OxygenInMaze {
            layout: self
                .layout
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        match v {
                            Content::Wall => ContentInTime::Wall,
                            Content::Target => ContentInTime::Target,
                            Content::Nothing => ContentInTime::Nothing(None),
                        },
                    )
                })
                .collect(),
        }
    }

    fn run(
        maze: &mut Maze,
        programm: &mut Programm,
        c: &Coordinate,
        found_min: Option<usize>,
    ) -> Option<usize> {
        let mut this_min = found_min;
        for i in 1..=4 {
            let coord = match i {
                1 => c.up(),
                2 => c.down(),
                3 => c.left(),
                _ => c.right(),
            };
            if maze.layout.contains_key(&coord) {
                continue;
            }
            let r = programm.run(&mut vec![i]);
            let r = r[0];
            if r == 0 {
                maze.layout.insert(coord.clone(), Content::Wall);
            // do nothing and stop
            } else if r == 1 {
                maze.layout.insert(coord.clone(), Content::Nothing);
                if let Some(m) =
                    Maze::run(maze, programm, &coord, this_min.map(|s| s - 1)).map(|s| s + 1)
                {
                    if this_min.is_none() {
                        this_min = Some(m);
                    } else {
                        if this_min.unwrap() > m {
                            this_min = Some(m);
                        }
                    }
                }
                let j = match i {
                    1 => 2,
                    2 => 1,
                    3 => 4,
                    _ => 3,
                };
                programm.run(&mut vec![j]);
            } else {
                maze.layout.insert(coord.clone(), Content::Target);
                this_min = Some(1);
            }
        }
        this_min
    }
}

pub fn run() {
    let input = File::open("input/task_15").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let mut programm = buffer.parse::<Programm>().unwrap();
    let mut maze = Maze::new();
    let result = Maze::run(&mut maze, &mut programm, &Coordinate::start(), None).unwrap();
    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_15").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let mut programm = buffer.parse::<Programm>().unwrap();
    let mut maze = Maze::new();
    Maze::run(&mut maze, &mut programm, &Coordinate::start(), None);

    let result = maze.fill_with_oxygen().count();
    println!("Result: {}", result);
}
