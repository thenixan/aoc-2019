use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Clone)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

impl Default for Velocity {
    fn default() -> Self {
        Velocity { x: 0, y: 0, z: 0 }
    }
}

#[derive(Eq, PartialEq)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    v: Velocity,
}

impl std::str::FromStr for Moon {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s[1..s.len() - 1]
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let x = s[0][2..].parse::<i32>().unwrap();
        let y = s[1][2..].parse::<i32>().unwrap();
        let z = s[2][2..].parse::<i32>().unwrap();
        println!("{:?}", s);
        Ok(Moon {
            x,
            y,
            z,
            v: Velocity::default(),
        })
    }
}

impl Velocity {
    fn kinetic_energy(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

impl Moon {
    fn potential_energy(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }

    fn energy(&self) -> usize {
        self.potential_energy() * self.v.kinetic_energy()
    }

    fn apply_velocity(&mut self) {
        self.x += self.v.x;
        self.y += self.v.y;
        self.z += self.v.z;
    }
}

struct PlanetarySystem {
    moons: Vec<Moon>,
}

impl std::iter::FromIterator<String> for PlanetarySystem {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        PlanetarySystem {
            moons: iter
                .into_iter()
                .filter_map(|m| m.parse::<Moon>().ok())
                .collect(),
        }
    }
}

impl std::iter::Iterator for PlanetarySystem {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let energy = self.moons.iter().map(|m| m.energy()).sum();
        self.evaluate_velocities();
        self.apply_velocities();
        Some(energy)
    }
}

impl PlanetarySystem {
    fn evaluate_velocities(&mut self) {
        for i in 0..self.moons.len() {
            let moon = &self.moons[i];
            let v = self
                .moons
                .iter()
                .enumerate()
                .filter_map(|(n, m)| if n != i { Some(m) } else { None })
                .fold(moon.v.clone(), |mut acc, m| {
                    if m.x > moon.x {
                        acc.x += 1;
                    } else if m.x < moon.x {
                        acc.x -= 1;
                    }
                    if m.y > moon.y {
                        acc.y += 1;
                    } else if m.y < moon.y {
                        acc.y -= 1;
                    }
                    if m.z > moon.z {
                        acc.z += 1;
                    } else if m.z < moon.z {
                        acc.z -= 1;
                    }
                    acc
                });
            self.moons[i].v = v;
        }
    }

    fn apply_velocities(&mut self) {
        self.moons.iter_mut().for_each(|m| m.apply_velocity());
    }
}

pub fn run() {
    let input = File::open("input/task_12").unwrap();
    let input = BufReader::new(input);
    let system = input
        .lines()
        .filter_map(|l| l.ok())
        .collect::<PlanetarySystem>();

    let result = system.take(1001).last().unwrap();
    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_11").unwrap();
    let mut input = BufReader::new(input);
}
