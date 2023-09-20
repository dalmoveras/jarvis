use chrono::prelude::*;
use std::error;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Error, Write};
use std::path::Path;
use std::{env, process};

pub struct Task {
    pub owner: String,
    pub mission: String,
    pub created_at: DateTime<Local>,
    pub status: String,
    pub completed_at: Option<DateTime<Local>>,
}

pub struct TodoList {
    pub todo: Vec<String>,
    pub file: File,
    pub todo_path: String,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Owner: {}, Mission: {}, Created At: {}, Status: {}, Completed At: {:?}",
            self.owner, self.mission, self.created_at, self.status, self.completed_at
        )
    }
}

impl Task {
    pub fn new(mission: String) -> Result<Self, Box<dyn error::Error>> {
        let owner: String = match env::var("USER") {
            Ok(owner) => owner,
            Err(_) => "anon".to_string(),
        };
        let created_at: DateTime<Local> = Local::now();
        let status: String = "To Be Completed".to_string();
        let completed_at = Default::default();

        Ok(Self {
            owner,
            mission,
            created_at,
            status,
            completed_at,
        })
    }
}

impl TodoList {
    pub fn new() -> Result<Self, Error> {
        let todo_path: String = match env::var("JARVIS_PATH") {
            Ok(todo_path) => todo_path,
            Err(_) => {
                let home = env::var("HOME").unwrap();
                let previous_todo = format!("{}/.jarvis", &home);
                match Path::new(&previous_todo).exists() {
                    true => previous_todo,
                    false => format!("{}/.jarvis", &home),
                }
            }
        };
        let todo = vec![];
        let mut file = File::create(&todo_path).unwrap();
        write!(file, "JARVIS LOG FILE\n");
        Ok(Self {
            todo,
            file,
            todo_path,
        })
    }

    pub fn add(&self, tasks: &[Task]) {
        if tasks.is_empty() {
            println!("[!] Add functionality needs at least one parameter <task> [!]");
            process::exit(1);
        }
        let timestamp: DateTime<Local> = Local::now();
        let mut todo_list = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .open(&self.todo_path)
            .expect("Jarvis couldn't open the file");

        for task in tasks {
            println!("{}", task);
            //write!(todo_list, "{} - {:?}\n",task, timestamp);
            write!(todo_list, "{}", task);
        }
    }

    pub fn list() {}
    pub fn remove() {}
    pub fn encrypt() {}
    pub fn delete() {}
    pub fn restore() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_todolist() {
        let todo = TodoList::new().unwrap();
        assert_eq!(
            format!("{}/.jarvis", env::var("HOME").unwrap()),
            todo.todo_path
        );
    }

    #[test]
    fn task_new() {
        let task1: Task = Task::new("Conquer the world".to_string()).unwrap();
        let task2: Task = Task::new("Conquer Mars".to_string()).unwrap();
        let todo = TodoList::new().unwrap();
        todo.add(&[task1, task2]);
        assert_eq!("Task", "Task");
    }
}
