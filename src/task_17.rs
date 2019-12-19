use crate::opcodes::Programm;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Clone)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum Object {
    Scaffold,
    Nothing,
    Robot(Direction),
}
struct Layout {
    width: usize,
    height: usize,
    content: Vec<Vec<Object>>,
}

impl Layout {
    fn item_in(&self, x: usize, y: usize) -> &Object {
        &self.content[y][x]
    }

    fn item_up(&self, x: usize, y: usize) -> &Object {
        let y = if y == 0 { self.height - 1 } else { y - 1 };
        self.item_in(x, y)
    }

    fn item_bottom(&self, x: usize, y: usize) -> &Object {
        let y = if y == self.height - 1 { 0 } else { y + 1 };
        self.item_in(x, y)
    }

    fn item_left(&self, x: usize, y: usize) -> &Object {
        let x = if x == 0 { self.width - 1 } else { x - 1 };
        self.item_in(x, y)
    }

    fn item_right(&self, x: usize, y: usize) -> &Object {
        let x = if x == self.width - 1 { 0 } else { x + 1 };
        self.item_in(x, y)
    }
}

impl FromStr for Layout {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{}", s);
        let mut result = vec![];
        let mut y = 0;
        let mut width = 0;
        for l in s.trim().lines() {
            result.push(vec![]);
            let mut x = 0;
            for c in l.chars() {
                match c {
                    '#' => result[y].push(Object::Scaffold),
                    '^' => result[y].push(Object::Robot(Direction::Top)),
                    'v' => result[y].push(Object::Robot(Direction::Bottom)),
                    '<' => result[y].push(Object::Robot(Direction::Left)),
                    '>' => result[y].push(Object::Robot(Direction::Right)),
                    _ => result[y].push(Object::Nothing),
                }
                x += 1;
            }
            width = std::cmp::max(x, width);
            y += 1;
        }
        Ok(Layout {
            width,
            height: y,
            content: result,
        })
    }
}

pub fn run() {
    let input = File::open("input/task_17").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let mut programm = buffer.parse::<Programm>().unwrap();

    let layout = programm
        .run(&mut vec![])
        .into_iter()
        .map(|c| std::char::from_u32(c as u32).unwrap())
        .collect::<String>()
        .parse::<Layout>()
        .unwrap();

    let mut result = 0;
    for y in 0..layout.height {
        for x in 0..layout.width {
            let item = layout.item_in(x, y);
            if item == &Object::Scaffold
                && item == layout.item_bottom(x, y)
                && item == layout.item_up(x, y)
                && item == layout.item_left(x, y)
                && item == layout.item_right(x, y)
            {
                result += x * y;
            }
        }
    }
    println!("{}", result);
}

pub fn run_e() {}
