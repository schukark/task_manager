use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, fs, io};

/*
    Helper boiler plate code to input a string and
    cut the \n or \r symbol form the end
*/
fn read_string() -> String {
    let mut s: String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Can't read string from stdin");

    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }

    s
}

/*
    Custom error that means there is no task with such id
*/
#[derive(Debug)]
struct TaskNotFoundError;

impl fmt::Display for TaskNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No task with such index")
    }
}

impl Error for TaskNotFoundError {}

/*
    Custom error struct that means there is no such option to choose from
*/
#[derive(Debug)]
struct IncorrectOptionError;

impl fmt::Display for IncorrectOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No such option")
    }
}

impl Error for IncorrectOptionError {}

/*
    The task struct with these fields:
    - title
    - description
    - due_date
    - priority
    - completeness status
*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    title: String,
    description: String,
    due_date: Option<DateTime<Utc>>,
    priority: Priority,
    status: bool,
}

impl Task {
    /*
       Task constructor from all the fields
    */
    fn new(
        title: String,
        description: String,
        due_date: Option<DateTime<Utc>>,
        priority: Option<Priority>,
    ) -> Task {
        Task {
            title,
            description,
            due_date,
            priority: priority.unwrap_or(Priority::Low),
            status: false,
        }
    }
}

/*
    Priority enum
*/

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Priority {
    High,
    Medium,
    Low,
}

impl Priority {
    fn get_numerical(&self) -> i8 {
        match self {
            Priority::High => 3,
            Priority::Medium => 2,
            Priority::Low => 1,
        }
    }
}

/*
    Implementation of the Display trait
*/
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let due_date = match self.due_date.is_none() {
            true => String::from("Not specified"),
            false => self
                .due_date
                .unwrap()
                .format("%d-%m-%Y %H:%M:%S")
                .to_string(),
        };

        let status = match self.status {
            true => String::from("Completed"),
            false => String::from("Not completed"),
        };

        let max_string_len = usize::max(
            usize::max(
                self.title.len() + String::from("Title: ").len(),
                self.description.len() + String::from("Description: ").len(),
            ),
            usize::max(
                due_date.len() + String::from("Due date: ").len(),
                status.len() + String::from("Status: ").len(),
            ),
        );

        let priority = match self.priority {
            Priority::High => "High",
            Priority::Medium => "Medium",
            Priority::Low => "Low",
        };
        let priority = priority.to_string();

        let max_string_len = usize::max(
            max_string_len,
            priority.len() + String::from("Priority: ").len(),
        );

        let title = self.title.clone()
            + &str::repeat(
                " ",
                max_string_len - self.title.len() - String::from("Title: ").len(),
            );
        let description = self.description.clone()
            + &str::repeat(
                " ",
                max_string_len - self.description.len() - String::from("Description: ").len(),
            );
        let due_date = due_date.clone()
            + &str::repeat(
                " ",
                max_string_len - due_date.len() - String::from("Due date: ").len(),
            );
        let status = status.clone()
            + &str::repeat(
                " ",
                max_string_len - status.len() - String::from("Status: ").len(),
            );
        let priority = priority.clone()
            + &str::repeat(
                " ",
                max_string_len - priority.len() - String::from("Priority: ").len(),
            );

        write!(f, "{}\n| Title: {} |\n| Description: {} |\n| Due date: {} |\n| Priority: {} |\n| Status: {} |\n{}", 
            str::repeat("-", max_string_len + 4), title, description, due_date, priority, status, str::repeat("-", max_string_len + 4))
    }
}
/*
    TaskManager struct that just stores all structs
*/
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    /*
       TaskManager empty constructor
    */
    pub fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    /*
       Static private function to parse date time from string
    */
    fn parse_data(line: String) -> Result<Option<DateTime<Utc>>, Box<dyn Error>> {
        if line.eq("none") {
            Ok(None)
        } else {
            let datetime = NaiveDateTime::parse_from_str(&line, "%d-%m-%Y %H:%M:%S")?;

            Ok(Some(TimeZone::from_utc_datetime(&Utc, &datetime)))
        }
    }

    /*
       Function that adds the task, gets its input from stdin
    */
    pub fn add_task(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Enter task title");
        let title = read_string();

        println!("Enter the task description");
        let description = read_string();

        println!("Enter the time in the following format: DD-MM-YYYY HH:MM:SS or enter none if the date shouldn't be specified");
        let datetime = read_string();

        println!("Enter the task priority: High/Medium/Low or none");
        let priority = read_string();

        let priority = match &priority[..] {
            "High" => Some(Priority::High),
            "Medium" => Some(Priority::Medium),
            "Low" => Some(Priority::Low),
            "none" => None,
            _ => {
                return Err(Box::new(IncorrectOptionError));
            }
        };

        if let Ok(datetime) = TaskManager::parse_data(datetime) {
            self.tasks
                .push(Task::new(title, description, datetime, priority));
        } else {
            return Err(Box::new(IncorrectOptionError));
        }

        println!("Added task:");
        println!("{}", self.tasks[self.tasks.len() - 1]);

        Ok(())
    }

    /*
       List all the items to stdin
    */
    pub fn list_items(&self) {
        for (id, item) in self.tasks.iter().enumerate() {
            println!("Task id {}\n{item}", id + 1);
        }
    }

    /*
        Mark the task as complete, get its input from stdin
    */
    pub fn complete_task(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Input the task id of the task you want to mark complete");
        let input: usize = read_string().parse()?;

        if input > self.tasks.len() || input == 0 {
            return Err(Box::new(TaskNotFoundError));
        }

        self.tasks[input - 1].status = true;
        println!("Marked the task with id {} correct", input - 1);

        Ok(())
    }

    /*
        Delete a task, gets the task id from stdin
    */
    pub fn delete_task(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Input the task id you want deleted");

        let input: usize = read_string().parse()?;

        if input > self.tasks.len() || input == 0 {
            return Err(Box::new(TaskNotFoundError));
        }

        self.tasks.remove(input - 1);

        Ok(())
    }

    /*
        Update the task with a given id, gets its id from stdin
    */
    pub fn update(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Print task id you would like to change");
        let index: usize = read_string().parse()?;

        if index > self.tasks.len() || index == 0 {
            return Err(Box::new(TaskNotFoundError));
        }

        println!("Print 'title'/'description' or 'due date' if you'd like to change those things");
        let input = read_string();

        println!("Print the changed title/description/due date");
        let changed_string = read_string();

        match &input[..] {
            "title" => {
                self.tasks[index - 1].title = changed_string;
            }
            "description" => {
                self.tasks[index - 1].description = changed_string;
            }
            "due date" => {
                if let Ok(new_date) = TaskManager::parse_data(changed_string) {
                    self.tasks[index - 1].due_date = new_date;
                } else {
                    return Err(Box::new(IncorrectOptionError));
                }
            }
            _ => {
                return Err(Box::new(IncorrectOptionError));
            }
        }

        Ok(())
    }

    /*
        Sorts the tasks based on some criteria
    */
    pub fn sort(&self) -> Result<(), Box<dyn Error>> {
        println!("Print 'priority' or 'due date' to sort by these parameters");
        let input = read_string();

        match &input[..] {
            "priority" => {
                let mut tasks = self.tasks.clone();
                tasks.sort_by(|a, b| a.priority.get_numerical().cmp(&b.priority.get_numerical()));

                for (id, item) in tasks.iter().enumerate() {
                    println!("Task id {}\n{item}", id + 1);
                }
            }
            "due date" => {
                let mut tasks = self.tasks.clone();
                tasks.sort_by(|a, b| a.due_date.cmp(&b.due_date));

                for (id, item) in tasks.iter().enumerate() {
                    println!("Task id {}\n{item}", id + 1);
                }
            }
            _ => {
                return Err(Box::new(IncorrectOptionError));
            }
        }

        Ok(())
    }
}

/*
    Implement the Drop trait for TaskManager
    Saves the TaskManager struct to json-like txt file
*/
impl Drop for TaskManager {
    fn drop(&mut self) {
        let content = serde_json::to_string(&self).unwrap();
        if let Err(error) = fs::write("data.txt", content) {
            eprintln!("Error writing to the file: {error}");
        }
    }
}
