use chrono::{DateTime, NaiveDateTime, Utc, TimeZone};
use std::{io, error::Error, fmt, fs};
use serde::{Serialize, Deserialize};

fn read_string() -> String {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).expect("Can't read string from stdin");

    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }

    s
}

#[derive(Debug)]
struct TaskNotFoundError;

impl fmt::Display for TaskNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No task with such index")
    }
}

impl Error for TaskNotFoundError {

}

#[derive(Debug)]
struct IncorrectOptionError;

impl fmt::Display for IncorrectOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No such option")
    }
}

impl Error for IncorrectOptionError {

}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    title: String,
    description: String,
    due_date: Option<DateTime<Utc>>,
    status: bool,
}

impl Task {
    fn new(title: String, description: String, due_date: Option<DateTime<Utc>>) -> Task {
        Task {
            title,
            description,
            due_date,
            status: false
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let due_date: String;
        if self.due_date.is_none() {
            due_date = String::from("Not specified");
        }
        else {
            due_date = self.due_date.unwrap().format("%d-%m-%Y %H:%M:%S").to_string();
        }

        let status: String;
        if self.status {
            status = String::from("Completed");
        }
        else {
            status = String::from("Not completed");
        }

        let max_string_len = usize::max(
            usize::max(
                self.title.len() + String::from("Title: ").len(), 
                self.description.len() + String::from("Description: ").len()
            ),
            usize::max(
                due_date.len() + String::from("Due date: ").len(),
                status.len() + String::from("Status: ").len()
            )
        );

        let title = self.title.clone() + &str::repeat(" ", max_string_len - self.title.len() - String::from("Title: ").len());
        let description = self.description.clone() + &str::repeat(" ", max_string_len - self.description.len() - String::from("Description: ").len());
        let due_date = due_date.clone() + &str::repeat(" ", max_string_len - due_date.len() - String::from("Due date: ").len());
        let status = status.clone() + &str::repeat(" ", max_string_len - status.len() - String::from("Status: ").len());
        
        write!(f, "{}\n| Title: {} |\n| Description: {} |\n| Due date: {} |\n| Status: {} |\n{}", 
            str::repeat("-", max_string_len + 4), title, description, due_date, status, str::repeat("-", max_string_len + 4))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager {tasks: Vec::new()}
    }

    fn parse_data(line: String) -> Result<Option<DateTime<Utc>>, Box<dyn Error>> {
        if line.eq("none") {
            return Ok(None);
        }
        else {
            let datetime = 
                NaiveDateTime::parse_from_str(&line, "%d-%m-%Y %H:%M:%S")?;
            
            return Ok(Some(TimeZone::from_utc_datetime(&Utc, &datetime)));
        }
    }

    pub fn add_task(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Enter task title");
        let title = read_string();

        println!("Enter the task description");
        let description = read_string();

        println!("Enter the time in the following format: DD-MM-YYYY HH:MM:SS or enter none if the date shouldn't be specified");
        let datetime = read_string();
        
        if let Ok(datetime) = TaskManager::parse_data(datetime) {
            self.tasks.push(Task::new(title, description, datetime));
        }
        else {
            return Err(Box::new(IncorrectOptionError));
        }

        println!("Added task:");
        println!("{}", self.tasks[self.tasks.len() - 1]);

        Ok(())
    }

    pub fn list_items(&self) {
        for (id, item) in self.tasks.iter().enumerate() {
            println!("Task id {}\n{item}", id + 1);
        }
    }

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

    pub fn delete_task(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Input the task id you want deleted");

        let input: usize = read_string().parse()?;

        if input > self.tasks.len() || input == 0 {
            return Err(Box::new(TaskNotFoundError));
        }

        self.tasks.remove(input - 1);

        Ok(())
    }

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
            },
            "description" => {
                self.tasks[index - 1].description = changed_string;
            },
            "due date" => {
                if let Ok(new_date) = TaskManager::parse_data(changed_string) {
                    self.tasks[index - 1].due_date = new_date;
                }
                else {
                    return Err(Box::new(IncorrectOptionError));
                }
            },
            _ => {
                return Err(Box::new(IncorrectOptionError));
            }
        }

        Ok(())
    }
}

impl Drop for TaskManager {
    fn drop(&mut self) {
        let content = serde_json::to_string(&self).unwrap();
        if let Err(error) = fs::write("data.txt", content) {
            eprintln!("Error writing to the file: {error}");
        }
    }
}