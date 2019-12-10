use asteroids::Map;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod asteroids {
    use std::iter::FromIterator;
    use std::ops::Add;
    use std::ops::Mul;
    use std::ops::Sub;

    struct StepGenerator {
        width: usize,
        height: usize,
        current_x: usize,
        current_y: usize,
        counter: usize,
    }

    impl StepGenerator {
        fn is_primal(v: usize) -> bool {
            if v == 0 || v == 1 {
                true
            } else {
                !(2..v).any(|a| v % a == 0)
            }
        }

        fn should_skip(x: usize, y: usize) -> bool {
            if x == y && x != 1 {
                true
            } else if x == 0 && y != 1 {
                true
            } else if x != 1 && y == 0 {
                true
            } else if !Self::is_primal(x) && !Self::is_primal(y) {
                true
            } else {
                false
            }
        }
    }

    impl Iterator for StepGenerator {
        type Item = Coordinate;
        fn next(&mut self) -> Option<Self::Item> {
            let mut result = None;
            while result.is_none() && self.current_y != self.height {
                if self.current_x == self.width {
                    self.current_x = 0;
                    self.current_y += 1;
                }
                if !Self::should_skip(self.current_x, self.current_y) {
                    result = match self.counter {
                        0 => Some(Coordinate::new(
                            self.current_x as i32,
                            self.current_y as i32,
                        )),
                        1 => {
                            if self.current_x == 0 {
                                None
                            } else {
                                Some(Coordinate::new(
                                    -(self.current_x as i32),
                                    self.current_y as i32,
                                ))
                            }
                        }
                        2 => {
                            if self.current_y == 0 {
                                None
                            } else {
                                Some(Coordinate::new(
                                    self.current_x as i32,
                                    -(self.current_y as i32),
                                ))
                            }
                        }
                        _ => {
                            if self.current_x == 0 && self.current_y == 0 {
                                None
                            } else {
                                Some(Coordinate::new(
                                    -(self.current_x as i32),
                                    -(self.current_y as i32),
                                ))
                            }
                        }
                    };
                }
                if self.counter == 3 {
                    self.counter = 0;
                    self.current_x += 1;
                } else {
                    self.counter += 1;
                }
            }
            println!("R: {:?}", result);
            result
        }
    }

    #[derive(Eq, PartialEq, Hash, Debug)]
    pub struct Coordinate(i32, i32);

    impl Coordinate {
        fn new(x: i32, y: i32) -> Self {
            Coordinate(x, y)
        }
    }

    impl Mul<i32> for &Coordinate {
        type Output = Coordinate;
        fn mul(self, rhs: i32) -> Self::Output {
            Coordinate::new(self.0 * rhs, self.1 * rhs)
        }
    }

    impl Add for &Coordinate {
        type Output = Coordinate;
        fn add(self, rhs: Self) -> Self::Output {
            Coordinate::new(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    impl Sub for &Coordinate {
        type Output = Coordinate;
        fn sub(self, rhs: Self) -> Self::Output {
            Coordinate::new(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    pub struct Map {
        layout: Vec<Coordinate>,
        width: usize,
        height: usize,
        primes: Vec<Coordinate>,
    }

    impl Map {
        fn new(map: Vec<Coordinate>, width: usize, height: usize) -> Self {
            let primes = Self::steps(width, height).collect::<Vec<_>>();
            Map {
                layout: map,
                width,
                height,
                primes,
            }
        }

        fn steps(width: usize, height: usize) -> StepGenerator {
            StepGenerator {
                width: width,
                height: height,
                current_x: 0,
                current_y: 0,
                counter: 0,
            }
        }

        pub fn coordinates(&self) -> &Vec<Coordinate> {
            &self.layout
        }

        pub fn len(&self) -> usize {
            self.layout.len()
        }

        fn is_valid(&self, coordinate: &Coordinate) -> bool {
            coordinate.0 >= 0
                && self.width as i32 > coordinate.0
                && coordinate.1 >= 0
                && self.height as i32 > coordinate.1
        }

        pub fn visible_at(&self, coordinate: &Coordinate) -> usize {
            let log = coordinate.0 == 3 && coordinate.1 == 4;
            self.primes
                .iter()
                .filter(|c| {
                    let mut moved = coordinate + c;
                    while self.is_valid(&moved) {
                        if self.layout.contains(&moved) {
                            if log {
                                println!("{:?}", moved);
                            }
                            return true;
                        } else {
                            moved = &moved + c;
                        }
                    }
                    false
                })
                .count()
        }
    }

    impl FromIterator<String> for Map {
        fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
            let mut map = vec![];

            let mut width = 0;
            let mut height = 0;
            for (y, s) in iter.into_iter().enumerate() {
                height = std::cmp::max(height, y);
                for (x, c) in s.chars().enumerate() {
                    width = std::cmp::max(width, x);
                    if c == '#' {
                        map.push(Coordinate::new(x as i32, y as i32));
                    }
                }
            }

            Map::new(map, width, height)
        }
    }
}

pub fn run() {
    let input = File::open("input/task_10").unwrap();
    let input = BufReader::new(input);
    let map = input.lines().filter_map(|l| l.ok()).collect::<Map>();

    let result = map
        .coordinates()
        .iter()
        .map(|c| map.visible_at(c))
        .max()
        .unwrap();
    println!("Result: {}", result);
}

pub fn run_e() {}
