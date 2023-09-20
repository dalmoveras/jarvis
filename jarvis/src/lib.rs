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
    pub tasks: Vec<Task>,
    pub jarvis_path: String,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "\nOwner: {} | Mission: {} | Created At: {} | Status: {} | Completed At: {:?} |",
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
        let home = env::var("HOME").unwrap();
        let jarvis_path = format!("{}/.jarvis", &home);
        let tasks: Vec<Task> = vec![];
        if !Path::new(&jarvis_path).exists() {
            let mut file = File::create(&jarvis_path).unwrap();
            write!(file, "JARVIS LOG FILE\n");
        }
        Ok(Self { tasks, jarvis_path })
    }

    pub fn add(&self, tasks: &[Task]) {
        if tasks.is_empty() {
            println!("[!] Add functionality needs at least one parameter <task> [!]");
            process::exit(1);
        }
        let mut todo_list = OpenOptions::new()
            .write(true)
            .read(true)
            .append(true)
            .open(&self.jarvis_path)
            .expect("Jarvis couldn't open the file");

        for task in tasks {
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
            todo.jarvis_path
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
