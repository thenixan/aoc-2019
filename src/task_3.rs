use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn distance(&self, from: &Coordinate) -> usize {
        let x = self.x - from.x;
        let y = self.y - from.y;
        (x.abs() + y.abs()) as usize
    }
    fn new() -> Self {
        Coordinate { x: 0, y: 0 }
    }
    fn up(&mut self) {
        self.y += 1;
    }
    fn down(&mut self) {
        self.y -= 1;
    }
    fn right(&mut self) {
        self.x += 1;
    }
    fn left(&mut self) {
        self.x -= 1;
    }
}

enum Path {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Path {
    fn count(&self) -> &usize {
        match self {
            Path::Up(c) => c,
            Path::Down(c) => c,
            Path::Left(c) => c,
            Path::Right(c) => c,
        }
    }

    fn coordinates(&self, from: &Coordinate) -> Vec<Coordinate> {
        let mut result = vec![];
        let mut i = 0;
        let count = self.count();
        let mut c = from.clone();
        while &i < count {
            match self {
                Path::Up(_) => c.up(),
                Path::Down(_) => c.down(),
                Path::Left(_) => c.left(),
                Path::Right(_) => c.right(),
            };
            result.push(c.clone());
            i += 1;
        }
        result
    }
}

impl FromStr for Path {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.as_bytes()[0] as char;
        let count = String::from_utf8(s.as_bytes()[1..].to_vec())
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        match first {
            'U' => Ok(Path::Up(count)),
            'D' => Ok(Path::Down(count)),
            'R' => Ok(Path::Right(count)),
            'L' => Ok(Path::Left(count)),
            _ => Err(()),
        }
    }
}

struct Route {
    path: Vec<Path>,
}

impl Route {
    fn coordinates(&self, from: &Coordinate) -> Vec<Coordinate> {
        let mut result = vec![];
        let mut from = from.clone();
        for p in &self.path {
            result.append(&mut p.coordinates(&from));
            from = result.last().unwrap().clone();
        }
        result
    }
}

impl FromIterator<Path> for Route {
    fn from_iter<I: IntoIterator<Item = Path>>(iter: I) -> Self {
        Route {
            path: iter.into_iter().collect(),
        }
    }
}

pub fn run() {
    let input = File::open("input/task_3").unwrap();
    let input = BufReader::new(input);

    let routes = input
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.split(",")
                .filter_map(|s| s.parse::<Path>().ok())
                .collect::<Route>()
        })
        .collect::<Vec<Route>>();

    let result = routes
        .iter()
        .fold(
            HashMap::new(),
            |mut acc: HashMap<Coordinate, usize>, route: &Route| {
                let mut coords = route.coordinates(&Coordinate::new());
                coords.sort();
                coords.dedup();
                for r in coords {
                    *acc.entry(r).or_insert(0) += 1;
                }
                acc
            },
        )
        .iter()
        .filter(|(_, v)| v > &&1)
        .map(|(k, _)| k)
        .map(|k| k.distance(&Coordinate::new()))
        .min()
        .unwrap();

    println!("Result: {}", result)
}

pub fn run_e() {
    let input = File::open("input/task_2").unwrap();
    let _input = BufReader::new(input);
}
