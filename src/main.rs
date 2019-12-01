use std::env;

mod task_1;

fn main() {
    let arg = env::args().nth(1);
    match arg {
        Some(x) => {
            println!("Task: {}", x);
            match x.as_str() {
                "1" => task_1::run(),
                "1e" => task_1::run_e(),
                _ => println!("Unresolved task"),
            };
        }
        None => println!("Provide task"),
    }
}
