use crate::opcodes::Programm;
use std::fs::File;
use std::io::{BufReader, Read};

struct ProgrammSet {
    p: Vec<Programm>,
}

impl ProgrammSet {
    fn new(count: usize, p: &Programm) -> Self {
        ProgrammSet {
            p: std::iter::repeat(p.clone()).take(count).collect(),
        }
    }

    fn run(&mut self, input: Vec<i32>) -> i32 {
        let mut i = input.clone();
        i.insert(i.len() - 1, 0);
        println!("acc: {:?}", i);
        self.p.iter_mut().fold(i, |mut acc, programm| {
            let r = programm.run(&mut acc);
            let p = if acc.len() >= 1 { acc.len() - 1 } else { 0 };
            acc.insert(p, r[0]);
            acc
        })[0]
    }

    fn run_with_loopback(&mut self, input: Vec<i32>) -> i32 {
        let mut inp = input.iter().map(|v| v + 5).collect::<Vec<i32>>();
        let mut outputs = vec![0];
        let mut finished = vec![false; self.p.len()];
        let mut i = 0;
        loop {
            let mut v = vec![];
            if let Some(input) = outputs.pop() {
                v.push(input);
            }
            if let Some(input) = inp.pop() {
                v.push(input);
            }
            let mut r = self.p[i].run(&mut v);
            finished[i] = self.p[i].is_finished();

            outputs.append(&mut r);
            if finished.iter().all(|f| *f == true) {
                return outputs[0];
            }
            i += 1;
            if i == self.p.len() {
                i = 0;
            }
        }
    }
}

struct InputGenerator {
    count: usize,
    pos: usize,
}

impl InputGenerator {
    fn new(count: usize) -> Self {
        InputGenerator { count, pos: 0 }
    }

    fn distinct(v: &Vec<i32>) -> bool {
        for i in 0..v.len() {
            if v.iter()
                .enumerate()
                .filter(|(n, _)| *n != i)
                .any(|(_, k)| *k == v[i])
            {
                return false;
            }
        }
        true
    }
}

impl Iterator for InputGenerator {
    type Item = Vec<i32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 5_usize.pow(self.count as u32) {
            None
        } else {
            let mut result = vec![0_i32; self.count];
            for i in 0..self.count {
                let a = (self.pos as i32 % 5_i32.pow(self.count as u32 - i as u32)) as i32;
                let b = (a / 5_i32.pow(self.count as u32 - i as u32 - 1)) as i32;
                result[i] = b;
            }
            self.pos += 1;
            if !InputGenerator::distinct(&result) {
                self.next()
            } else {
                Some(result)
            }
        }
    }
}

pub fn run() {
    let input = File::open("input/task_7").unwrap();
    let mut input = BufReader::new(input);

    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let programm = buffer.parse::<Programm>().unwrap();

    let result = InputGenerator::new(5)
        .map(|input| ProgrammSet::new(5, &programm).run(input))
        .max()
        .unwrap();
    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_7").unwrap();
    let mut input = BufReader::new(input);

    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let programm = buffer.parse::<Programm>().unwrap();

    let result = InputGenerator::new(5)
        .map(|input| ProgrammSet::new(5, &programm).run_with_loopback(input))
        .max()
        .unwrap();
    println!("Result: {}", result);
}
