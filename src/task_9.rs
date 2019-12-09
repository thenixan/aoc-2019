use crate::opcodes::Programm;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn run() {
    let input = File::open("input/task_9").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let mut programm = buffer.parse::<Programm>().unwrap();

    let result = programm.run(&mut vec![1]);

    println!("Result: {:?}", result);
}

pub fn run_e() {
    let input = File::open("input/task_9").unwrap();
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let mut programm = buffer.parse::<Programm>().unwrap();

    let result = programm.run(&mut vec![2]);

    println!("Result: {:?}", result);
}
