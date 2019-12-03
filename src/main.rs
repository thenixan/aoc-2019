use std::env;

mod task_1;
mod task_2;
mod task_3;

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
                _ => println!("Unresolved task"),
            };
        }
        None => println!("Provide task"),
    }
}
