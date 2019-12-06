use crate::nodes::Node;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::rc::Rc;
use std::str::FromStr;

#[derive(Eq, PartialEq)]
struct Orbiting(String);

impl FromStr for Orbiting {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Orbiting(s.to_string()))
    }
}

pub fn run() {
    let input = File::open("input/task_6").unwrap();
    let input = BufReader::new(input);

    let mut v = input
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.split(")")
                .filter_map(|s| s.parse::<Orbiting>().ok())
                .map(|s| Node::new(s))
                .collect::<Vec<Node<Orbiting>>>()
        })
        .map(move |n| {
            let mut left = n[0];
            left += n[1];
            left
        })
        .collect::<Vec<Node<Orbiting>>>();

    while !v.is_empty() {
        let mut i = 0;
        while i < v.len() {
            let mut j = i + 1;
            while j < v.len() {
                if v[0].value() == v[1].value() {
                    v[0] += v[1];
                    v.remove(1);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }

    println!("Result: {}", v.len());
}

pub fn run_e() {}
