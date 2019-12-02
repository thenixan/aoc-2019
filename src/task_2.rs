use std::fs::File;
use std::io::{BufReader, Read};

fn evaluate(data: &mut Vec<i32>, position: usize) {
    match data.get(position) {
        Some(1) => {
            let left = data[position + 1] as usize;
            let right = data[position + 2] as usize;
            let result = data[position + 3] as usize;
            data[result] = data[left] + data[right];
            evaluate(data, position + 4);
        }
        Some(2) => {
            let left = data[position + 1] as usize;
            let right = data[position + 2] as usize;
            let result = data[position + 3] as usize;
            data[result] = data[left] * data[right];
            evaluate(data, position + 4);
        }
        _ => (),
    };
}

pub fn run() {
    let input = File::open("input/task_2").unwrap();
    let mut input = BufReader::new(input);
    let mut line = String::new();

    input.read_to_string(&mut line).unwrap();

    let mut result = line
        .split(",")
        .filter_map(|l| l.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    result[1] = 12;
    result[2] = 2;

    evaluate(&mut result, 0);

    println!("Result: {}", result[0]);
}

fn evaluate_e(data: Vec<i32>, target: i32) -> i32 {
    (0..=99)
        .flat_map(move |left| (0..=99).map(move |right| (left, right)))
        .map(|(left, right)| {
            let mut input = data.clone();
            input[1] = left;
            input[2] = right;
            evaluate(&mut input, 0);
            (left, right, input[0])
        })
        .find_map(|(left, right, result)| {
            if result == target {
                Some(100 * left + right)
            } else {
                None
            }
        })
        .unwrap()
}

pub fn run_e() {
    let input = File::open("input/task_2").unwrap();
    let mut input = BufReader::new(input);
    let mut line = String::new();

    input.read_to_string(&mut line).unwrap();

    let result = line
        .split(",")
        .filter_map(|l| l.parse::<i32>().ok())
        .collect::<Vec<i32>>();

    let result = evaluate_e(result, 19690720);

    println!("Result: {}", result);
}
