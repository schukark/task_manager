use chrono::{DateTime, NaiveDateTime, Utc, TimeZone};
use std::{io, error::Error, fmt, fs};
use serde::{Serialize, Deserialize};

fn read_string() -> String {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).expect("Can' read string from stdin");

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

#[derive(Debug, Serialize, Deserialize)]
struct Task {
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

    pub fn add_task(&mut self) -> Result<(), Box<dyn Error>> {
        println!("Enter task title");
        let title = read_string();

        println!("Enter the task description");
        let description = read_string();

        println!("Enter the time in the following format: DD-MM-YYYY HH:MM:SS or enter none if the date shouldn't be specified");
        let datetime = read_string();
        
        if datetime.eq("none") {
            self.tasks.push(Task::new(title, description, None));
        }
        else {
            let datetime = 
                NaiveDateTime::parse_from_str(&datetime, "%d-%m-%Y %H:%M:%S")?;
            
                self.tasks.push(Task::new(title, description, Some(TimeZone::from_utc_datetime(&Utc, &datetime))));
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
        todo!();
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