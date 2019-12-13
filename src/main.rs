use std::env;

mod opcodes;

mod task_1;
mod task_10;
mod task_11;
mod task_12;
mod task_13;
mod task_2;
mod task_3;
mod task_4;
mod task_5;
mod task_6;
mod task_7;
mod task_8;
mod task_9;

fn main() {
    let arg = env::args().nth(1);
    match arg {
        Some(x) => {
            println!("Task: {}", x);
            match x.as_str() {
                "1" => task_1::run(),
                "1e" => task_1::run_e(),
                "2" => task_2::run(),
                "2e" => task_2::run_e(),
                "3" => task_3::run(),
                "3e" => task_3::run_e(),
                "4" => task_4::run(),
                "4e" => task_4::run_e(),
                "5" => task_5::run(),
                "5e" => task_5::run_e(),
                "6" => task_6::run(),
                "6e" => task_6::run_e(),
                "7" => task_7::run(),
                "7e" => task_7::run_e(),
                "8" => task_8::run(),
                "8e" => task_8::run_e(),
                "9" => task_9::run(),
                "9e" => task_9::run_e(),
                "10" => task_10::run(),
                "10e" => task_10::run_e(),
                "11" => task_11::run(),
                "11e" => task_11::run_e(),
                "12" => task_12::run(),
                "12e" => task_12::run_e(),
                "13" => task_13::run(),
                "13e" => task_13::run_e(),
                _ => println!("Unresolved task"),
            };
        }
        None => println!("Provide task"),
    }
}
