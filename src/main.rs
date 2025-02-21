fn main() {
    let _task = std::env::args().nth(1).expect("No task given!");
    let _data = std::env::args().nth(3).expect("No task name given!");
    let _task_id = std::env::args().nth(2).expect("No task ID given!");

    match _task.as_str() {
        "add" => println!("{_task} function"),
        "update" => {
            println!("{_task} function");
            let task_id = _task_id.trim().parse::<i32>().expect("Unable to parse task ID!");
            println!("updating ID:{task_id} with data: {_data}")
        }
        "list" => println!("{_task} all tasks!"),
        "delete" => {
            println!("{_task} function");
            let task_id = _task_id.trim().parse::<i32>().expect("Unable to parse task ID!");
            println!("Deleting task ID: {task_id}")
        }
        "mark" => {
            println!("{_task} function");
            let task_id = _task_id.trim().parse::<i32>().expect("Unable to parse task ID!");
            println!("mark task ID: {task_id} as {_data}")
        }
        _ => {
            println!("Wrong action: {_task}!")
        }
    }

}
