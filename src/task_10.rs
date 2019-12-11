use asteroids::Map;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod asteroids {
    use std::iter::FromIterator;
    use std::ops::Add;
    use std::ops::Mul;
    use std::ops::Sub;

    struct PrimalIterator {
        to: usize,
        current: usize,
    }

    impl PrimalIterator {
        fn new(to: usize) -> Self {
            PrimalIterator { to, current: 0 }
        }
    }

    impl Iterator for PrimalIterator {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            self.current += 1;
            if self.current > self.to {
                None
            } else {
                while self.to % self.current != 0 {
                    self.current += 1;
                }
                Some(self.current)
            }
        }
    }

    struct PerimeterIterator {
        width: usize,
        height: usize,
        current_x: usize,
        current_y: usize,
        finished: usize,
    }

    impl PerimeterIterator {
        fn new(width: usize, height: usize) -> Self {
            PerimeterIterator {
                width,
                height,
                current_y: 0,
                current_x: 0,
                finished: 0,
            }
        }
    }

    impl Iterator for PerimeterIterator {
        type Item = Coordinate;
        fn next(&mut self) -> Option<Self::Item> {
            let result = if self.finished == 4 {
                None
            } else {
                Some(Coordinate::new(
                    self.current_x as i32,
                    self.current_y as i32,
                ))
            };
            if self.finished == 0 && self.current_x == self.width - 1 {
                self.finished = 1;
            }
            if self.finished == 0 {
                self.current_x += 1;
            }
            if self.finished == 1 && self.current_y == self.height - 1 {
                self.finished = 2;
            }
            if self.finished == 1 {
                self.current_y += 1;
            }
            if self.finished == 2 && self.current_x == 0 {
                self.finished = 3;
            }
            if self.finished == 2 {
                self.current_x -= 1;
            }
            if self.finished == 3 && self.current_y == 0 {
                self.finished = 4;
            }
            if self.finished == 3 {
                self.current_y -= 1;
            }
            result
        }
    }

    struct CoordinateStepIterator<'a> {
        from: &'a Coordinate,
        to: &'a Coordinate,
        primal_iterator: PrimalIterator,
    }

    impl<'a> CoordinateStepIterator<'a> {
        fn new(from: &'a Coordinate, to: &'a Coordinate) -> Self {
            let d_x = (from.0 - to.0).abs();
            let d_y = (from.1 - to.1).abs();
            let d = std::cmp::max(d_x, d_y) as usize;
            CoordinateStepIterator {
                from,
                to,
                primal_iterator: PrimalIterator::new(d),
            }
        }
    }

    impl<'a> Iterator for CoordinateStepIterator<'a> {
        type Item = Coordinate;
        fn next(&mut self) -> Option<Self::Item> {
            while let Some(p) = self.primal_iterator.next() {
                if (self.from.0 - self.to.0) % p as i32 == 0
                    && (self.from.1 - self.to.1) % p as i32 == 0
                {
                    let x = if self.from.0 > self.to.0 {
                        -(self.from.0 - self.to.0) / p as i32
                    } else {
                        (self.to.0 - self.from.0) / p as i32
                    };
                    let y = if self.from.1 > self.to.1 {
                        -(self.from.1 - self.to.1) / p as i32
                    } else {
                        (self.to.1 - self.from.1) / p as i32
                    };
                    return Some(Coordinate::new(self.from.0 + x, self.from.1 + y));
                }
            }
            None
        }
    }

    #[derive(Eq, PartialEq, Hash, Debug)]
    pub struct Coordinate(i32, i32);

    impl Coordinate {
        fn new(x: i32, y: i32) -> Self {
            Coordinate(x, y)
        }

        fn steps_to<'a>(&'a self, other: &'a Coordinate) -> CoordinateStepIterator<'a> {
            CoordinateStepIterator::new(self, other)
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
        perimeter: Vec<Coordinate>,
    }

    impl Map {
        fn new(map: Vec<Coordinate>, width: usize, height: usize) -> Self {
            let perimeter = Self::perimeter(width, height).collect::<Vec<_>>();
            Map {
                layout: map,
                width,
                height,
                perimeter,
            }
        }

        fn perimeter(width: usize, height: usize) -> PerimeterIterator {
            PerimeterIterator::new(width, height)
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
            let on_top = coordinate.1 == 0;
            let on_bottom = coordinate.1 as usize == self.height - 1;
            let on_left = coordinate.0 == 0;
            let on_right = coordinate.0 as usize == self.width - 1;

            if log {
                println!(
                    "T: {}, B: {}, L: {}, R: {}",
                    on_top, on_bottom, on_left, on_right
                );
            }

            self.perimeter
                .iter()
                .filter(|c| {
                    let mut result = true;
                    if on_top && c.1 == 0 && c.0 != 0 && c.0 as usize != self.width - 1 {
                        result = false;
                    }
                    if on_left && c.0 == 0 && c.1 != 0 && c.1 as usize != self.height - 1 {
                        result = false;
                    }
                    if on_right
                        && c.0 as usize == self.width - 1
                        && c.1 != 0
                        && c.1 as usize != self.height - 1
                    {
                        result = false;
                    }
                    if on_bottom
                        && c.1 as usize == self.height - 1
                        && c.0 != 0
                        && c.0 as usize != self.width - 1
                    {
                        result = false;
                    }
                    result
                })
                .filter_map(|c| coordinate.steps_to(c).find(|m| self.layout.contains(&m)))
                .inspect(|c| {
                    if log {
                        println!("P: {},{}", c.0, c.1);
                    }
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

            Map::new(map, width + 1, height + 1)
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
