use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
enum Path {
    Straight(Coordinate),
    Intersection(Coordinate),
}

impl Path {
    fn coordinate(&self) -> &Coordinate {
        match self {
            Path::Straight(c) => c,
            Path::Intersection(c) => c,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
struct PathLength {
    path: Path,
    length: usize,
}

impl PathLength {
    fn new(path: Path) -> Self {
        PathLength {
            path,
            length: std::usize::MAX,
        }
    }
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

enum PathDirection {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl PathDirection {
    fn count(&self) -> &usize {
        match self {
            PathDirection::Up(c) => c,
            PathDirection::Down(c) => c,
            PathDirection::Left(c) => c,
            PathDirection::Right(c) => c,
        }
    }

    fn coordinates(&self, from: &Coordinate) -> Vec<Coordinate> {
        let mut result = vec![];
        let mut i = 0;
        let count = self.count();
        let mut c = from.clone();
        while &i < count {
            match self {
                PathDirection::Up(_) => c.up(),
                PathDirection::Down(_) => c.down(),
                PathDirection::Left(_) => c.left(),
                PathDirection::Right(_) => c.right(),
            };
            result.push(c.clone());
            i += 1;
        }
        result
    }
}

impl FromStr for PathDirection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.as_bytes()[0] as char;
        let count = String::from_utf8(s.as_bytes()[1..].to_vec())
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        match first {
            'U' => Ok(PathDirection::Up(count)),
            'D' => Ok(PathDirection::Down(count)),
            'R' => Ok(PathDirection::Right(count)),
            'L' => Ok(PathDirection::Left(count)),
            _ => Err(()),
        }
    }
}

struct Route {
    path: Vec<PathDirection>,
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

    fn trace(&self, from: &Coordinate) -> Vec<PathLength> {
        let path = self.coordinates(from);
        let mut result = vec![];

        println!("Tracing");
        for p in &path {
            result.push(PathLength::new(
                if path.iter().filter(|i| i == &p).nth(1).is_some() {
                    Path::Intersection(p.clone())
                } else {
                    Path::Straight(p.clone())
                },
            ));
        }

        println!("Stepping");

        Self::step(1, 0, &mut result, true);

        result
    }

    fn step(step: usize, position: usize, result: &mut Vec<PathLength>, up: bool) {
        if result.len() > position {
            if result[position].length > step {
                result[position].length = step;
                match &result[position].path {
                    Path::Straight(_) => Self::step(
                        step + 1,
                        if up { position + 1 } else { position - 1 },
                        result,
                        up,
                    ),
                    c => {
                        let positions = result
                            .iter()
                            .enumerate()
                            .filter_map(|i| if &i.1.path == c { Some(i.0) } else { None })
                            .collect::<Vec<usize>>();
                        for pos in positions {
                            Self::step(step + 1, pos + 1, result, true);
                            Self::step(step + 1, pos - 1, result, false);
                        }
                    }
                }
            }
        }
    }
}

impl FromIterator<PathDirection> for Route {
    fn from_iter<I: IntoIterator<Item = PathDirection>>(iter: I) -> Self {
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
                .filter_map(|s| s.parse::<PathDirection>().ok())
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
    let input = File::open("input/task_3").unwrap();
    let input = BufReader::new(input);

    let routes = input
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.split(",")
                .filter_map(|s| s.parse::<PathDirection>().ok())
                .collect::<Route>()
        })
        .collect::<Vec<Route>>();

    let intersections = routes
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
        .into_iter()
        .filter(|(_, v)| v > &&1)
        .map(|(k, _)| k)
        .collect::<Vec<Coordinate>>();

    println!("Intersections evaluated");
    let paths = routes
        .iter()
        .map(|route| route.trace(&Coordinate::new()))
        .collect::<Vec<Vec<PathLength>>>();

    println!("Lengths evaluated");

    let result = intersections
        .iter()
        .map(|c| {
            paths
                .iter()
                .map(|p| {
                    p.iter()
                        .find(|pl: &&PathLength| pl.path.coordinate() == c)
                        .unwrap()
                })
                .map(|p| p.length)
                .sum::<usize>()
        })
        .inspect(|l| println!("Found length: {}", l))
        .min()
        .unwrap();

    println!("Result: {:?}", result)
}
