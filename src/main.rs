use task_manager::TaskManager;
use std::{io, fs};

fn main() {
    let mut task_manager;

    if let Ok(contents) = fs::read_to_string("data.txt") {
        task_manager = serde_json::from_str(&contents).unwrap();
    }
    else {
        task_manager = TaskManager::new();
    }

    let help_string = "TaskManager supports the following functions:
add - add a new task
help - to print help
list - to list all the tasks
complete - to mark a task as complete
update - to update an existing task
remove - to remove the task completely
exit - to exit the program";

    println!("{}", help_string);

    loop {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Could't read input");
        input.pop();
        
        match &input[..] {
            "add" => {
                if task_manager.add_task().is_err() {
                    println!("Couldn't add the task due to incorrect input format");
                }

            },
            "help" => {
                println!("{}", help_string);
            },
            "list" => {
                task_manager.list_items();
            },
            "remove" => {
                if task_manager.delete_task().is_err() {
                    println!("Couldn't delete the task due to incorrect input");
                }
            },
            "complete" => {
                if task_manager.complete_task().is_err() {
                    println!("Couldn't mark the task as complete due to incorrect input");
                }
            },
            "update" => {
                if task_manager.update().is_err() {
                    println!("Couldn't update the task due to incorrect input");
                }
            }
            "exit" => {
                break;
            }
            _ => {
                println!("Unknown command");
            }
        }
    }

    println!("Good bye!");
}
