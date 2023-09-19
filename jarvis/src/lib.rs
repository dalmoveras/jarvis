use chrono::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Error, Write};
use std::path::Path;
use std::{env, process};
pub struct TodoList {
    pub todo: Vec<String>,
    pub file: File,
    pub todo_path: String,
    //pub backup: String,
}

impl TodoList {
    pub fn new() -> Result<Self, String> {
        //let todo = vec!();
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

    pub fn add(&self, tasks: &[String]) {
        if tasks.is_empty() {
            println!("[!] Add functionality needs at least one parameter <task> [!]");
            process::exit(1);
        }
        let timestamp: DateTime<Local> = Local::now();
        let todo_list = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&self.todo_path)
            .expect("Jarvis couldn't open the file");

        let mut buf_reader = BufReader::new(&todo_list);
        let backup = String::new();
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
}
