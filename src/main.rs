use std::env::args;

fn main() {
    let _task = std::env::args().nth(1).expect("No task given!");
    let _name = std::env::args().nth(2).expect("No task name given!");

    println!("Task {_task} '{_name}'")

}
