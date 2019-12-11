use asteroids::Map;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod asteroids {
    use std::iter::FromIterator;
    use std::ops::Add;
    use std::ops::Mul;
    use std::ops::Sub;

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
    }

    impl Map {
        fn new(map: Vec<Coordinate>) -> Self {
            Map { layout: map }
        }

        pub fn coordinates(&self) -> &Vec<Coordinate> {
            &self.layout
        }

        pub fn visible_at(&self, coordinate: &Coordinate) -> usize {
            // let log = coordinate.0 == 3 && coordinate.1 == 4;

            self.layout
                .iter()
                .filter(|c| *c != coordinate)
                .fold(Vec::new(), |mut acc: Vec<(f32, bool, bool)>, c| {
                    let d_xc = coordinate.0 - c.0;
                    let d_yc = coordinate.1 - c.1;
                    let lz_x = d_xc > 0;
                    let lz_y = d_yc > 0;
                    let d_c = d_xc as f32 / d_yc as f32;
                    if !acc.contains(&(d_c, lz_x, lz_y)) {
                        acc.push((d_c, lz_x, lz_y));
                    }
                    acc
                })
                .len()
        }
    }

    impl FromIterator<String> for Map {
        fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
            let mut map = vec![];

            for (y, s) in iter.into_iter().enumerate() {
                for (x, c) in s.chars().enumerate() {
                    if c == '#' {
                        map.push(Coordinate::new(x as i32, y as i32));
                    }
                }
            }

            Map::new(map)
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
        .map(|c| (c, map.visible_at(c)))
        .max_by(|l, r| l.1.cmp(&r.1))
        .unwrap();
    println!("Result: {}:{:?}", result.1, result.0);
}

pub fn run_e() {}
