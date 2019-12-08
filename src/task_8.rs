use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};

struct Layer {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Layer {
    fn join(&self, other: &Layer) -> Self {
        let mut data = vec![2; self.data.len()];
        for i in 0..self.data.len() {
            if self.data[i] == b'2' {
                data[i] = other.data[i];
            } else {
                data[i] = self.data[i];
            }
        }
        Layer {
            data,
            width: self.width,
            height: self.height,
        }
    }
}

impl Display for Layer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for i in 0..self.height {
            let mut line = String::new();
            for j in 0..self.width {
                line.push(match self.data[self.width * i + j] {
                    b'1' => '#',
                    b'0' => ' ',
                    _ => '.',
                });
            }
            result.push_str(line.as_str());
            result.push_str("\n");
        }
        write!(f, "{}", result)
    }
}

struct LayeredIterator<I> {
    width: usize,
    height: usize,
    iter: I,
}

trait Layered<I>: Iterator<Item = std::io::Result<u8>> {
    fn layered(self, width: usize, height: usize) -> LayeredIterator<I>;
}

impl<I> Layered<I> for I
where
    I: Iterator<Item = std::io::Result<u8>>,
{
    fn layered(self, width: usize, height: usize) -> LayeredIterator<I> {
        LayeredIterator {
            width,
            height,
            iter: self,
        }
    }
}

impl<I> LayeredIterator<I>
where
    I: Iterator<Item = std::io::Result<u8>>,
{
    fn join(&mut self) -> Option<Layer> {
        let mut first = self.next();
        while let Some(second) = self.next() {
            first = Some(first.unwrap().join(&second));
        }
        first
    }
}

impl<I> Iterator for LayeredIterator<I>
where
    I: Iterator<Item = std::io::Result<u8>>,
{
    type Item = Layer;

    fn next(&mut self) -> Option<Layer> {
        let mut counter = self.width * self.height;
        let mut result = vec![];
        while let Some(Ok(b)) = self.iter.next() {
            result.push(b);
            counter -= 1;
            if counter == 0 {
                let l = Layer {
                    data: result,
                    width: self.width,
                    height: self.height,
                };
                return Some(l);
            }
        }
        None
    }
}

pub fn run() {
    let input = File::open("input/task_8").unwrap();
    let input = BufReader::new(input);
    let result = input
        .bytes()
        .layered(25, 6)
        .min_by(|l, r| {
            l.data
                .iter()
                .filter(|b| **b == b'0')
                .count()
                .cmp(&r.data.iter().filter(|b| **b == b'0').count())
        })
        .map(|l| {
            l.data.iter().filter(|b| **b == b'1').count()
                * l.data.iter().filter(|b| **b == b'2').count()
        });

    println!("Result: {:?}", result);
}

pub fn run_e() {
    let input = File::open("input/task_8").unwrap();
    let input = BufReader::new(input);
    let result = input.bytes().layered(25, 6).join();

    println!("{}", result.unwrap());
}
