use asteroids::Map;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod asteroids {
    extern crate termion;
    use std::env;
    use std::iter::FromIterator;
    use std::ops::Add;
    use std::ops::Mul;
    use std::ops::Sub;
    use termion::{clear, color, cursor};
    #[derive(Eq, PartialEq, Hash, Debug, Clone)]
    pub struct Coordinate(i32, i32);

    impl Coordinate {
        fn new(x: i32, y: i32) -> Self {
            Coordinate(x, y)
        }

        pub fn x(&self) -> i32 {
            self.0
        }

        pub fn y(&self) -> i32 {
            self.1
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

    #[derive(Debug)]
    pub struct Slope<'a> {
        angle: f32,
        distance: f32,
        coordinate: &'a Coordinate,
    }

    impl<'a> Slope<'a> {
        fn new(from: &Coordinate, to: &'a Coordinate) -> Self {
            let diff = to - from;
            let angle = (diff.1 as f32).atan2(diff.0 as f32) * 180_f32 / std::f32::consts::PI;
            let angle = if angle < 0_f32 {
                angle + 450_f32
            } else {
                angle + 90_f32
            };
            let angle = if angle >= 360_f32 {
                angle - 360_f32
            } else {
                angle
            };
            let distance = ((diff.0 as f32).powi(2) + (diff.1 as f32).powi(2)).sqrt();
            Slope {
                angle,
                distance,
                coordinate: to,
            }
        }
    }

    #[derive(Debug)]
    pub struct SlopeMap<'a> {
        data: Vec<Slope<'a>>,
    }

    impl<'a> FromIterator<Slope<'a>> for SlopeMap<'a> {
        fn from_iter<I: IntoIterator<Item = Slope<'a>>>(iter: I) -> Self {
            let mut data: Vec<Slope> = iter.into_iter().collect();
            data.sort_by(|a, b| a.angle.partial_cmp(&b.angle).unwrap());
            SlopeMap { data }
        }
    }

    impl<'a> SlopeMap<'a> {
        fn visible(&self) -> Vec<&Coordinate> {
            self.data
                .iter()
                .filter(|c| c.distance > 0_f32)
                .fold(vec![], |mut acc: Vec<&Slope>, c: &Slope| {
                    if let Some(i) = acc.iter().enumerate().find_map(|a| {
                        if a.1.angle == c.angle {
                            Some(a.0)
                        } else {
                            None
                        }
                    }) {
                        if acc[i].distance > c.distance {
                            acc.remove(i);
                            acc.push(c);
                        }
                    } else {
                        acc.push(c);
                    }
                    acc
                })
                .iter()
                .map(|c| c.coordinate)
                .collect()
        }
    }

    fn animate(map: Vec<&Coordinate>, accent_one: &Coordinate, accent_two: &Coordinate) {
        let mut s = format!(
            "{clear}{goto}{white}",
            clear = clear::All,
            goto = cursor::Goto(1, 1),
            white = color::Fg(color::Reset),
        );
        let width = map.iter().map(|c| c.0).max().map(|c| c + 1).unwrap();
        let height = map.iter().map(|c| c.1).max().map(|c| c + 1).unwrap();

        for j in 0..height {
            let mut line = String::new();
            for i in 0..width {
                let c = Coordinate::new(i, j);
                if map.contains(&&c) && accent_one == &c {
                    line += format!(
                        "{red}{v}{reset}",
                        red = color::Fg(color::Red),
                        v = '*',
                        reset = color::Fg(color::Reset)
                    )
                    .as_str();
                } else if map.contains(&&c) && accent_two == &c {
                    line += format!(
                        "{red}{v}{reset}",
                        red = color::Fg(color::Yellow),
                        v = 'X',
                        reset = color::Fg(color::Reset)
                    )
                    .as_str();
                } else if map.contains(&&c) {
                    line += "#";
                } else {
                    line += ".";
                }
            }
            s += line.as_str();
            s += "\n";
        }
        println!("{}", s);
    }

    pub struct Map {
        layout: Vec<Coordinate>,
    }

    impl Map {
        fn new(map: Vec<Coordinate>) -> Self {
            Map { layout: map }
        }

        fn slopes(&self, from: &Coordinate) -> SlopeMap {
            self.layout.iter().map(|c| Slope::new(from, c)).collect()
        }

        pub fn coordinates(&self) -> &Vec<Coordinate> {
            &self.layout
        }

        pub fn visible_at(&self, coordinate: &Coordinate) -> usize {
            let slopes = self.slopes(coordinate);
            slopes.visible().len()
        }

        pub fn nth_destroyed(&self, coordinate: &Coordinate, n: usize) -> Coordinate {
            let slopes = self.slopes(coordinate);
            let visible = slopes.visible();
            if visible.len() > n {
                if env::args().nth(2).map(|v| v == "animate").unwrap_or(false) {
                    for i in 0..n {
                        animate(
                            self.layout
                                .iter()
                                .filter(|c| {
                                    !visible.iter().take(i).collect::<Vec<_>>().contains(&c)
                                })
                                .collect(),
                            visible[i],
                            coordinate,
                        );
                        std::thread::sleep(std::time::Duration::from_millis(20));
                    }
                }
                visible[n - 1].clone()
            } else {
                self.layout
                    .iter()
                    .filter(|p| !visible.contains(p))
                    .map(|c| c.clone())
                    .collect::<Map>()
                    .nth_destroyed(coordinate, n - visible.len())
            }
        }
    }

    impl FromIterator<Coordinate> for Map {
        fn from_iter<I: IntoIterator<Item = Coordinate>>(iter: I) -> Self {
            Map::new(iter.into_iter().collect())
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
    println!("Result: {}", result.1);
}

pub fn run_e() {
    let input = File::open("input/task_10").unwrap();
    let input = BufReader::new(input);
    let map = input.lines().filter_map(|l| l.ok()).collect::<Map>();

    let target = map
        .coordinates()
        .iter()
        .map(|c| (c, map.visible_at(c)))
        .max_by(|l, r| l.1.cmp(&r.1))
        .unwrap()
        .0;

    let result = map.nth_destroyed(target, 200);
    let result = result.x() * 100 + result.y();
    println!("Result: {}", result);
}
