use task_manager::TaskManager;
use std::fs;
use clap::Parser;


/*
    struct containing all the command line parameters
*/
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
pub struct Options {
    #[clap(short, long, action)]
    /// Adds a task to the list
    pub add: bool,

    #[clap(short, long, action)]
    /// Removes a task from the list
    pub remove: bool,

    #[clap(short, long, action)]
    /// Marks the task as complete
    pub complete: bool,

    #[clap(short, long, action)]
    /// Prints the list of all tasks
    pub list: bool,

    #[clap(short, long, action)]
    /// Updates the task title/description or due date
    pub update: bool,
}

/*
    Helper enum to parse the command line arguments
*/
#[derive(Debug)]
pub enum Commands {
    Add,
    Remove,
    Complete,
    List,
    Update,
    None,
}

fn main() {
    let args = Options::parse();

    let mut task_manager;

    if let Ok(contents) = fs::read_to_string("data.txt") {
        task_manager = serde_json::from_str(&contents).unwrap();
    }
    else {
        task_manager = TaskManager::new();
    }
    
    if args.add as i8 + args.complete as i8 + args.remove as i8 + args.list as i8 + args.update as i8 > 1 {
        println!("Can't have more than one command line argument");
        std::process::exit(0);
    }

    let input;

    if args.add {
        input = Commands::Add;
    }
    else if args.complete {
        input = Commands::Complete;
    }
    else if args.remove {
        input = Commands::Remove;
    }
    else if args.list {
        input = Commands::List;
    }
    else if args.update {
        input = Commands::Update;
    }
    else {
        input = Commands::None;
    }
    
    match input {
        Commands::Add => {
            if task_manager.add_task().is_err() {
                println!("Couldn't add the task due to incorrect input format");
            }

        },
        Commands::List => {
            task_manager.list_items();
        },
        Commands::Remove => {
            if task_manager.delete_task().is_err() {
                println!("Couldn't delete the task due to incorrect input");
            }
        },
        Commands::Complete => {
            if task_manager.complete_task().is_err() {
                println!("Couldn't mark the task as complete due to incorrect input");
            }
        },
        Commands::Update => {
            if task_manager.update().is_err() {
                println!("Couldn't update the task due to incorrect input");
            }
        }
        Commands::None => {
            println!("Unknown command");
        }
    }
}
